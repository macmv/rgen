use eframe::{
  egui::{self, PaintCallbackInfo},
  glow,
  glow::HasContext,
};

use crate::{
  chunks::{CHUNK, ChunkUpdate},
  grid::Upload,
};

const VERTEX: &str = "#version 330 core
void main() {
  // One triangle that covers the whole viewport.
  vec2 p = vec2((gl_VertexID << 1) & 2, gl_VertexID & 2);
  gl_Position = vec4(p * 2.0 - 1.0, 0.0, 1.0);
}
";

const FRAGMENT: &str = include_str!("voxel.glsl");

/// The most bricks that are copied to the GPU in one frame. Copying them all at
/// once stalls the driver, and can freeze the whole desktop, so the rest wait
/// for later frames.
const BRICKS_PER_FRAME: usize = 96;

/// Everything the shader needs to draw one frame.
#[derive(Clone, Copy)]
pub struct Frame {
  pub eye:     [f32; 3],
  pub forward: [f32; 3],
  pub right:   [f32; 3],
  pub up:      [f32; 3],
  pub pixel:   f32,
  /// How far the grid is rotated to get the window, in blocks.
  pub shift:   [i32; 2],
  pub hover:   Option<(i32, i32)>,
}

/// Draws the terrain with a fragment shader. Created once with the GL context,
/// and shared with the paint callback.
pub struct Renderer {
  program: glow::Program,
  vao:     glow::VertexArray,
  /// The color of every palette index, 256 texels wide.
  palette: glow::Texture,
  /// One `u32` per chunk: empty, solid, or the chunk's slot in the atlas.
  table:   glow::Texture,
  /// The blocks of every chunk that has any, in slots of `CHUNK` cubed.
  bricks:  glow::Texture,

  /// What `table` holds. It's uploaded in one piece, rather than a texel at a
  /// time, as small writes to an integer texture are slow.
  table_data:  Vec<u32>,
  table_dirty: bool,

  size:           [i32; 2],
  /// The number of chunks along x, y and z.
  chunks:         [i32; 3],
  /// The number of slots along each side of the atlas. The atlas is
  /// reallocated to this size on the next paint.
  side:           i32,
  /// The number of slots along each side of the atlas that is allocated now.
  allocated_side: i32,
  /// The most slots along a side that the driver allows.
  max_side:       i32,
  /// A new palette to copy to `palette` on the next paint.
  palette_data:   Option<Vec<u8>>,
  pending:        Vec<Upload>,
  /// Repainted while uploads are waiting, so that they keep going.
  ctx:            egui::Context,
}

impl Renderer {
  pub fn new(gl: &glow::Context, ctx: egui::Context) -> Result<Self, String> {
    unsafe {
      let program = gl.create_program()?;
      let mut shaders = vec![];
      for (kind, source) in [(glow::VERTEX_SHADER, VERTEX), (glow::FRAGMENT_SHADER, FRAGMENT)] {
        let shader = gl.create_shader(kind)?;
        gl.shader_source(shader, source);
        gl.compile_shader(shader);
        if !gl.get_shader_compile_status(shader) {
          return Err(gl.get_shader_info_log(shader));
        }
        gl.attach_shader(program, shader);
        shaders.push(shader);
      }
      gl.link_program(program);
      if !gl.get_program_link_status(program) {
        return Err(gl.get_program_info_log(program));
      }
      for shader in shaders {
        gl.detach_shader(program, shader);
        gl.delete_shader(shader);
      }

      let vao = gl.create_vertex_array()?;

      let make_texture = |gl: &glow::Context| -> Result<glow::Texture, String> {
        let texture = gl.create_texture()?;
        gl.bind_texture(glow::TEXTURE_2D, Some(texture));
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::NEAREST as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::NEAREST as i32);
        Ok(texture)
      };

      let make_volume = |gl: &glow::Context| -> Result<glow::Texture, String> {
        let texture = gl.create_texture()?;
        gl.bind_texture(glow::TEXTURE_3D, Some(texture));
        gl.tex_parameter_i32(glow::TEXTURE_3D, glow::TEXTURE_MIN_FILTER, glow::NEAREST as i32);
        gl.tex_parameter_i32(glow::TEXTURE_3D, glow::TEXTURE_MAG_FILTER, glow::NEAREST as i32);
        Ok(texture)
      };

      // The atlas starts out tiny, and is sized by `set_slots` once the view
      // distance is known.
      let max_side = (gl.get_parameter_i32(glow::MAX_3D_TEXTURE_SIZE) / CHUNK as i32).max(1);

      let bricks = make_volume(gl)?;
      Self::alloc_atlas(gl, bricks, 1);

      Ok(Renderer {
        program,
        vao,
        palette: make_texture(gl)?,
        palette_data: None,
        table: make_volume(gl)?,
        bricks,
        table_data: vec![],
        table_dirty: false,
        size: [0, 0],
        chunks: [0, 0, 0],
        side: 1,
        allocated_side: 1,
        max_side,
        pending: vec![],
        ctx,
      })
    }
  }

  /// The number of chunks the atlas has room for blocks of.
  pub fn brick_capacity(&self) -> usize { (self.side as usize).pow(3) }

  /// The video memory the atlas takes, in bytes.
  pub fn atlas_bytes(&self) -> usize { self.brick_capacity() * CHUNK.pow(3) }

  /// Sizes the atlas to hold at least `slots` chunks, or as many as the driver
  /// allows. Returns whether the size changed, in which case the atlas is
  /// emptied, and the grid has to start over.
  pub fn set_slots(&mut self, slots: usize) -> bool {
    let side = ((slots as f64).cbrt().ceil() as i32).clamp(1, self.max_side);
    let changed = side != self.side;
    self.side = side;
    changed
  }

  /// Allocates storage for an atlas that is `side` slots along each edge.
  unsafe fn alloc_atlas(gl: &glow::Context, bricks: glow::Texture, side: i32) {
    unsafe {
      let edge = side * CHUNK as i32;
      gl.bind_texture(glow::TEXTURE_3D, Some(bricks));
      gl.pixel_store_i32(glow::UNPACK_ALIGNMENT, 1);
      gl.tex_image_3d(
        glow::TEXTURE_3D,
        0,
        glow::R8UI as i32,
        edge,
        edge,
        edge,
        0,
        glow::RED_INTEGER,
        glow::UNSIGNED_BYTE,
        None,
      );
    }
  }

  /// Sets the color of every block, as RGBA8 with 256 texels.
  pub fn set_palette(&mut self, palette: Vec<u8>) { self.palette_data = Some(palette); }

  /// Queues new terrain, which is uploaded on the next paint. Emptying the
  /// chunks replaces everything.
  pub fn queue(&mut self, upload: Upload) {
    match upload {
      Upload::Reset { .. } => self.pending.clear(),
      _ => {}
    }
    self.pending.push(upload);
  }

  /// Where a slot starts in the atlas, in texels.
  fn slot_origin(&self, slot: u32) -> [i32; 3] {
    let (slot, side) = (slot as i32, self.side);
    let chunk = CHUNK as i32;
    [slot % side * chunk, slot / side % side * chunk, slot / (side * side) * chunk]
  }

  /// Copies the bricks of some updates to the atlas, and records where they
  /// are in the table, which is uploaded later.
  fn upload_chunks(&mut self, gl: &glow::Context, updates: &[ChunkUpdate]) {
    let [w, h, _] = self.chunks.map(|c| c as usize);
    for update in updates {
      if let Some(brick) = &update.brick {
        let [x, y, z] = self.slot_origin(update.value - crate::chunks::FIRST_SLOT);
        unsafe {
          gl.bind_texture(glow::TEXTURE_3D, Some(self.bricks));
          gl.tex_sub_image_3d(
            glow::TEXTURE_3D,
            0,
            x,
            y,
            z,
            CHUNK as i32,
            CHUNK as i32,
            CHUNK as i32,
            glow::RED_INTEGER,
            glow::UNSIGNED_BYTE,
            glow::PixelUnpackData::Slice(brick),
          );
        }
      }

      let [x, y, z] = update.chunk;
      self.table_data[x + w * (y + h * z)] = update.value;
    }
    self.table_dirty = true;
  }

  pub fn paint(&mut self, info: &PaintCallbackInfo, gl: &glow::Context, frame: &Frame) {
    unsafe {
      if self.side != self.allocated_side {
        // Reallocating the texture drops its old storage. Whatever was in the
        // atlas is gone, but the grid restarts whenever the size changes.
        Self::alloc_atlas(gl, self.bricks, self.side);
        self.allocated_side = self.side;
      }
      gl.pixel_store_i32(glow::UNPACK_ALIGNMENT, 1);
      if let Some(palette) = self.palette_data.take() {
        gl.bind_texture(glow::TEXTURE_2D, Some(self.palette));
        gl.tex_image_2d(
          glow::TEXTURE_2D,
          0,
          glow::RGBA8 as i32,
          256,
          1,
          0,
          glow::RGBA,
          glow::UNSIGNED_BYTE,
          Some(&palette),
        );
      }
      let mut budget = BRICKS_PER_FRAME;
      let mut queue = std::mem::take(&mut self.pending).into_iter();
      while let Some(upload) = queue.next() {
        match upload {
          Upload::Reset { size, dims } => {
            self.size = [size[0] as i32, size[1] as i32];

            self.chunks = dims.map(|d| d as i32);
            let [w, h, d] = self.chunks;
            self.table_data = vec![0; (w * h * d) as usize];
            self.table_dirty = false;

            gl.bind_texture(glow::TEXTURE_3D, Some(self.table));
            gl.tex_image_3d(
              glow::TEXTURE_3D,
              0,
              glow::R32UI as i32,
              w,
              h,
              d,
              0,
              glow::RED_INTEGER,
              glow::UNSIGNED_INT,
              Some(&vec![0u8; (w * h * d) as usize * 4]),
            );
          }
          Upload::Chunks(mut updates) => {
            // Copy as many bricks as are left in this frame's budget, and
            // leave the rest, and everything after them, for the next frame.
            let (mut bricks, mut take) = (0, updates.len());
            for (i, update) in updates.iter().enumerate() {
              if update.brick.is_some() {
                if bricks == budget {
                  take = i;
                  break;
                }
                bricks += 1;
              }
            }
            let rest = updates.split_off(take);
            self.upload_chunks(gl, &updates);
            budget -= bricks;

            if !rest.is_empty() {
              self.pending.push(Upload::Chunks(rest));
              self.pending.extend(queue);
              break;
            }
          }
        }
      }

      if self.table_dirty {
        let [w, h, d] = self.chunks;
        gl.bind_texture(glow::TEXTURE_3D, Some(self.table));
        gl.tex_sub_image_3d(
          glow::TEXTURE_3D,
          0,
          0,
          0,
          0,
          w,
          h,
          d,
          glow::RED_INTEGER,
          glow::UNSIGNED_INT,
          glow::PixelUnpackData::Slice(bytes_of_u32(&self.table_data)),
        );
        self.table_dirty = false;
      }
      if !self.pending.is_empty() {
        self.ctx.request_repaint();
      }

      if self.size[0] == 0 || self.chunks[0] == 0 {
        return;
      }

      let viewport = info.viewport_in_pixels();

      gl.disable(glow::BLEND);
      gl.disable(glow::DEPTH_TEST);
      gl.use_program(Some(self.program));
      gl.bind_vertex_array(Some(self.vao));

      gl.active_texture(glow::TEXTURE0);
      gl.bind_texture(glow::TEXTURE_2D, Some(self.palette));
      gl.active_texture(glow::TEXTURE1);
      gl.bind_texture(glow::TEXTURE_3D, Some(self.table));
      gl.active_texture(glow::TEXTURE2);
      gl.bind_texture(glow::TEXTURE_3D, Some(self.bricks));

      let uniform = |name: &str| gl.get_uniform_location(self.program, name);
      gl.uniform_1_i32(uniform("uPalette").as_ref(), 0);
      gl.uniform_1_i32(uniform("uTable").as_ref(), 1);
      gl.uniform_1_i32(uniform("uBricks").as_ref(), 2);
      gl.uniform_3_i32(uniform("uChunks").as_ref(), self.chunks[0], self.chunks[1], self.chunks[2]);
      gl.uniform_1_i32(uniform("uAtlasSide").as_ref(), self.side);
      gl.uniform_2_i32(uniform("uShift").as_ref(), frame.shift[0], frame.shift[1]);
      gl.uniform_3_f32_slice(uniform("uEye").as_ref(), &frame.eye);
      gl.uniform_3_f32_slice(uniform("uForward").as_ref(), &frame.forward);
      gl.uniform_3_f32_slice(uniform("uRight").as_ref(), &frame.right);
      gl.uniform_3_f32_slice(uniform("uUp").as_ref(), &frame.up);
      gl.uniform_4_f32(
        uniform("uViewport").as_ref(),
        viewport.left_px as f32,
        viewport.from_bottom_px as f32,
        viewport.width_px as f32,
        viewport.height_px as f32,
      );
      gl.uniform_1_f32(uniform("uPixel").as_ref(), frame.pixel);
      let (hx, hz) = frame.hover.unwrap_or((-1, -1));
      gl.uniform_2_i32(uniform("uHover").as_ref(), hx, hz);

      gl.draw_arrays(glow::TRIANGLES, 0, 3);

      gl.active_texture(glow::TEXTURE0);
    }
  }
}

fn bytes_of_u32(values: &[u32]) -> &[u8] {
  // SAFETY: u32 has no padding, and u8 has no alignment requirement.
  unsafe { std::slice::from_raw_parts(values.as_ptr().cast(), values.len() * 4) }
}
