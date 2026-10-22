use eframe::{egui::PaintCallbackInfo, glow, glow::HasContext};

use crate::grid::Upload;

const VERTEX: &str = "#version 330 core
void main() {
  // One triangle that covers the whole viewport.
  vec2 p = vec2((gl_VertexID << 1) & 2, gl_VertexID & 2);
  gl_Position = vec4(p * 2.0 - 1.0, 0.0, 1.0);
}
";

const FRAGMENT: &str = include_str!("voxel.glsl");

/// Everything the shader needs to draw one frame.
#[derive(Clone, Copy)]
pub struct Frame {
  pub eye:        [f32; 3],
  pub forward:    [f32; 3],
  pub right:      [f32; 3],
  pub up:         [f32; 3],
  pub pixel:      f32,
  pub max_height: f32,
  pub hover:      Option<(i32, i32)>,
}

/// Draws the terrain with a fragment shader. Created once with the GL context,
/// and shared with the paint callback.
pub struct Renderer {
  program: glow::Program,
  vao:     glow::VertexArray,
  heights: glow::Texture,
  colors:  glow::Texture,

  size:    [i32; 2],
  pending: Vec<Upload>,
}

impl Renderer {
  pub fn new(gl: &glow::Context) -> Result<Self, String> {
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

      Ok(Renderer {
        program,
        vao,
        heights: make_texture(gl)?,
        colors: make_texture(gl)?,
        size: [0, 0],
        pending: vec![],
      })
    }
  }

  /// Queues new terrain, which is uploaded on the next paint. A full upload
  /// replaces everything queued before it.
  pub fn queue(&mut self, upload: Upload) {
    if matches!(upload, Upload::Full { .. }) {
      self.pending.clear();
    }
    self.pending.push(upload);
  }

  pub fn paint(&mut self, info: &PaintCallbackInfo, gl: &glow::Context, frame: &Frame) {
    unsafe {
      gl.pixel_store_i32(glow::UNPACK_ALIGNMENT, 1);
      for upload in std::mem::take(&mut self.pending) {
        match upload {
          Upload::Full { size, heights, colors } => {
            self.size = [size[0] as i32, size[1] as i32];

            gl.bind_texture(glow::TEXTURE_2D, Some(self.heights));
            gl.tex_image_2d(
              glow::TEXTURE_2D,
              0,
              glow::R32F as i32,
              self.size[0],
              self.size[1],
              0,
              glow::RED,
              glow::FLOAT,
              Some(bytes_of(&heights)),
            );

            gl.bind_texture(glow::TEXTURE_2D, Some(self.colors));
            gl.tex_image_2d(
              glow::TEXTURE_2D,
              0,
              glow::RGBA8 as i32,
              self.size[0],
              self.size[1],
              0,
              glow::RGBA,
              glow::UNSIGNED_BYTE,
              Some(&colors),
            );
          }
          Upload::Tile { x, y, size, heights, colors } => {
            let (x, y, size) = (x as i32, y as i32, size as i32);

            gl.bind_texture(glow::TEXTURE_2D, Some(self.heights));
            gl.tex_sub_image_2d(
              glow::TEXTURE_2D,
              0,
              x,
              y,
              size,
              size,
              glow::RED,
              glow::FLOAT,
              glow::PixelUnpackData::Slice(bytes_of(&heights)),
            );

            gl.bind_texture(glow::TEXTURE_2D, Some(self.colors));
            gl.tex_sub_image_2d(
              glow::TEXTURE_2D,
              0,
              x,
              y,
              size,
              size,
              glow::RGBA,
              glow::UNSIGNED_BYTE,
              glow::PixelUnpackData::Slice(&colors),
            );
          }
        }
      }
      if self.size[0] == 0 {
        return;
      }

      let viewport = info.viewport_in_pixels();

      gl.disable(glow::BLEND);
      gl.disable(glow::DEPTH_TEST);
      gl.use_program(Some(self.program));
      gl.bind_vertex_array(Some(self.vao));

      gl.active_texture(glow::TEXTURE0);
      gl.bind_texture(glow::TEXTURE_2D, Some(self.heights));
      gl.active_texture(glow::TEXTURE1);
      gl.bind_texture(glow::TEXTURE_2D, Some(self.colors));

      let uniform = |name: &str| gl.get_uniform_location(self.program, name);
      gl.uniform_1_i32(uniform("uHeight").as_ref(), 0);
      gl.uniform_1_i32(uniform("uColor").as_ref(), 1);
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
      gl.uniform_2_i32(uniform("uSize").as_ref(), self.size[0], self.size[1]);
      gl.uniform_1_f32(uniform("uMaxHeight").as_ref(), frame.max_height);
      let (hx, hz) = frame.hover.unwrap_or((-1, -1));
      gl.uniform_2_i32(uniform("uHover").as_ref(), hx, hz);

      gl.draw_arrays(glow::TRIANGLES, 0, 3);

      gl.active_texture(glow::TEXTURE0);
    }
  }
}

fn bytes_of(floats: &[f32]) -> &[u8] {
  // SAFETY: f32 has no padding, and u8 has no alignment requirement.
  unsafe { std::slice::from_raw_parts(floats.as_ptr().cast(), floats.len() * 4) }
}
