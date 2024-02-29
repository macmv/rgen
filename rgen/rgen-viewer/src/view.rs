use std::collections::HashSet;

use rgen_base::{ChunkPos, Pos};
use sdl2::pixels::Color;

use crate::{render::RenderGrid, terrain::TerrainGenerator, world::World, RenderMode};

pub struct WorldViewer {
  pub grid: RenderGrid,

  placed_chunks: HashSet<ChunkPos>,
}

impl WorldViewer {
  pub fn new(screen_width: u32, screen_height: u32) -> WorldViewer {
    let grid = RenderGrid::new(screen_width, screen_height, 4);

    WorldViewer { grid, placed_chunks: HashSet::new() }
  }

  pub fn place_chunk(&mut self, world: &World<TerrainGenerator>, chunk_pos: ChunkPos) {
    if self.placed_chunks.contains(&chunk_pos) {
      return;
    }

    if !world.has_chunk(chunk_pos + ChunkPos::new(1, 1))
      || !world.has_chunk(chunk_pos + ChunkPos::new(1, 0))
      || !world.has_chunk(chunk_pos + ChunkPos::new(1, -1))
      || !world.has_chunk(chunk_pos + ChunkPos::new(0, 1))
      || !world.has_chunk(chunk_pos + ChunkPos::new(0, -1))
      || !world.has_chunk(chunk_pos + ChunkPos::new(-1, 1))
      || !world.has_chunk(chunk_pos + ChunkPos::new(-1, 0))
      || !world.has_chunk(chunk_pos + ChunkPos::new(-1, -1))
    {
      return;
    }

    self.placed_chunks.insert(chunk_pos);

    for rel_x in 0..16 {
      for rel_z in 0..16 {
        let pos = chunk_pos.min_block_pos() + Pos::new(rel_x, 0, rel_z);
        let column = world.column_at(pos);

        let biome = column.biome;
        let meter_height = column.height as f64;

        let block_distance = 1;
        // ╔═╦═╦═╗
        // ║a║b║c║
        // ╠═╬═╬═╣     MINECRAFT
        // ║d║é║f║     - X & Z is flat plane
        // ╠═╬═╬═╣     - Y is up
        // ║g║h║i║
        // ╚═╩═╩═╝ <- var table  || block_distance

        let a = world.height_at(pos + Pos::new(-block_distance, 0, block_distance));
        let b = world.height_at(pos + Pos::new(0, 0, block_distance));
        let c = world.height_at(pos + Pos::new(block_distance, 0, block_distance));

        let d = world.height_at(pos + Pos::new(-block_distance, 0, 0));
        let f = world.height_at(pos + Pos::new(block_distance, 0, 0));

        let g = world.height_at(pos + Pos::new(-block_distance, 0, -block_distance));
        let h = world.height_at(pos + Pos::new(0, 0, -block_distance));
        let i = world.height_at(pos + Pos::new(block_distance, 0, -block_distance));

        let dz_dx = ((c + (2.0 * f) + i) * 4.0 - (a + (2.0 * d) + g) * 4.0) / (8.0 * 1.0);
        //[dz/dx] = ((c + 2f + i)*4/wght1 - (a + 2d + g)*4/wght2) / (8 * x_cellsize)

        let dz_dy = ((g + (2.0 * h) + i) * 4.0 - (a + (2.0 * b) + c) * 4.0) / (8.0 * 1.0);
        //[dz/dy] = ((g + 2h + i)*4/wght3 - (a + 2b + c)*4/wght4) / (8 * y_cellsize)

        //claculates cell slope at that location

        let cell_slope = ((dz_dx).powi(2) + (dz_dy).powi(2)).sqrt().atan();
        //dbg!(cell_slope);
        //Slope = arctan(sqrt((dz/dx)^2 + (dz/dy)^2))

        //calulates cell aspect this is the direction the cell is facing as a slope
        let cell_aspect = dz_dx.atan2(-dz_dy);
        //arctan2(dz/dx, -dz/dy)

        let azimuth = 315.0 / 180.0 * std::f64::consts::PI;
        let altidue = 45.0 / 180.0 * std::f64::consts::PI;

        let solar_incidence_angle = (altidue.sin() * cell_slope.sin()
          + altidue.cos() * cell_slope.cos() * (azimuth - cell_aspect).cos())
        .acos();

        let brightness = ((((solar_incidence_angle).cos() + 1.0) / 2.0) * 255.0) as u8;

        let mode = RenderMode::Brightness;

        let brightness = match mode {
          RenderMode::Height => (meter_height * 2.0) as u8,
          RenderMode::Slope => (cell_slope * 255.0 / std::f64::consts::PI) as u8,
          RenderMode::Aspect => {
            //let asp = (cell_aspect * 255.0 / std::f64::consts::PI) as u8;
            //println!("Aspect: {asp}");
            (cell_aspect * 255.0 / std::f64::consts::PI) as u8
          }
          RenderMode::Brightness => {
            //let bright = (brightness as f64 * 0.2 + meter_height as f64 * 2.0) as u8;
            //println!("Brightness: {bright}");
            (brightness as f64 * 0.2 + meter_height as f64 * 2.0) as u8
          }
          RenderMode::BiomeColors => 0,
          //
          //HSV
          //Hue:0-360         - This is the color of the terrain
          //Saturation:0-100  - This is the terrain height
          //Value:0-100       - Keep locked too set darkness to max-light
        };

        let height_color = Color::RGB(brightness, brightness, brightness);
        let biome_color = world.color_for_biome(biome);

        self.grid.set(
          pos.x,
          pos.z,
          //ERROR THAT I DON'T FEE LIKE FIXING TRACKED DOWN
          Color::RGB(
            height_color.r, //+ biome_color.r,
            height_color.g, //+ biome_color.g,
            height_color.b, //+ biome_color.b,
          ),
        );
      }
    }
  }
}
