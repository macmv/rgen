//! The real terrain generator, for the blocks of a tile.
//!
//! This runs the same generation as the game: `Region` generates and decorates
//! the chunks, and this turns their blocks into bricks of palette indices for
//! the ray marcher. Every block is a cube of one color, picked by its kind.

use std::sync::Arc;

use rgen_base::{BlockKind, ChunkPos, ChunkRelPos, StateId};
use rgen_biome::WorldBiomes;
use rgen_world::{Context, Region};

use crate::{
  chunks::{Brick, CHUNK, WORLD_HEIGHT, brick_index},
  grid::TILE_BLOCKS,
};

/// What the workers need to generate real terrain.
pub struct RealTerrain {
  ctx:     Context,
  palette: Palette,
}

impl RealTerrain {
  pub fn new(seed: u64) -> Self {
    let ctx = Context::new_test(seed);
    let palette = Palette::new(&ctx);
    RealTerrain { ctx, palette }
  }

  pub fn palette(&self) -> &Palette { &self.palette }
}

/// Maps blocks to palette indices, which are what bricks store. Index 0 is air.
pub struct Palette {
  /// The palette index of every block id.
  index:  Vec<u8>,
  colors: Vec<[u8; 3]>,
}

impl Palette {
  fn new(ctx: &Context) -> Self {
    let mut index = vec![0u8; BlockKind::ALL.len()];
    let mut colors = vec![[0, 0, 0]];
    for &kind in BlockKind::ALL {
      if kind == BlockKind::Air {
        continue;
      }
      let id = ctx.blocks.lookup(kind).expect("every block has an id");
      // Blocks past the 255th share the last color. There are far fewer
      // distinct blocks than that in the terrain.
      if colors.len() < 256 {
        index[id.0 as usize] = colors.len() as u8;
        colors.push(block_color(kind));
      } else {
        index[id.0 as usize] = 255;
      }
    }
    Palette { index, colors }
  }

  pub fn index(&self, state: StateId) -> u8 { self.index[state.block().0 as usize] }

  /// The colors as an RGBA8 texture that is 256 texels wide.
  pub fn texture(&self) -> Vec<u8> {
    let mut out = vec![255; 256 * 4];
    for (i, c) in self.colors.iter().enumerate() {
      out[i * 4..i * 4 + 3].copy_from_slice(c);
    }
    out
  }
}

/// The color of a block. The common ones are picked by hand, and the rest come
/// from a hash of the name, which is stable and distinct enough to tell apart.
fn block_color(kind: BlockKind) -> [u8; 3] {
  let name = kind.name();
  let name = name.split_once(':').map_or(name, |(_, n)| n);

  match name {
    "stone" | "cobblestone" => [125, 125, 125],
    "dirt" | "farmland" | "coarse_dirt" => [121, 85, 58],
    "grass" | "grass_path" => [95, 159, 53],
    "sand" => [219, 207, 163],
    "sandstone" | "red_sandstone" => [214, 200, 150],
    "gravel" => [131, 127, 126],
    "water" | "flowing_water" => [48, 92, 200],
    "lava" | "flowing_lava" => [230, 110, 20],
    "bedrock" => [40, 40, 40],
    "snow" | "snow_layer" => [245, 250, 250],
    "ice" | "packed_ice" | "frosted_ice" => [160, 200, 240],
    "clay" | "hardened_clay" => [160, 165, 175],
    "log" | "log2" => [102, 81, 50],
    "leaves" | "leaves2" => [52, 120, 40],
    "tallgrass" | "double_plant" => [110, 175, 70],
    "coal_ore" => [60, 60, 60],
    "iron_ore" => [200, 160, 130],
    "gold_ore" => [230, 200, 60],
    "redstone_ore" => [180, 30, 30],
    "lapis_ore" => [40, 70, 170],
    "diamond_ore" => [90, 220, 210],
    _ => {
      // FNV-1a, spread out to a color that isn't too dark.
      let mut h = 0xcbf29ce484222325u64;
      for b in name.bytes() {
        h = (h ^ u64::from(b)).wrapping_mul(0x100000001b3);
      }
      [0, 1, 2].map(|i| 90 + ((h >> (i * 8)) & 0x7f) as u8)
    }
  }
}

/// The chunks (of the grid, not of `Region`) along each side of a tile.
const PER_TILE: usize = TILE_BLOCKS / CHUNK;
/// The generator's chunks along each side of a tile.
const GEN_PER_TILE: i32 = (TILE_BLOCKS / 16) as i32;
const LAYERS: usize = WORLD_HEIGHT / CHUNK;

/// The bricks of one tile: the `PER_TILE` squared columns of chunks, with the
/// layers of each from the bottom up.
pub struct TileBlocks {
  /// Indexed by `cz * PER_TILE + cx`.
  pub columns: Vec<Vec<Brick>>,
}

impl RealTerrain {
  /// Generates and decorates the blocks of one tile.
  pub fn generate(&self, world: &WorldBiomes, tile: [i32; 2]) -> TileBlocks {
    let min = ChunkPos::new(tile[0] * GEN_PER_TILE, tile[1] * GEN_PER_TILE);
    let max = ChunkPos::new(min.x + GEN_PER_TILE - 1, min.z + GEN_PER_TILE - 1);
    let region = Region::generate(&self.ctx, world, min, max);

    let mut columns = Vec::with_capacity(PER_TILE * PER_TILE);
    for cz in 0..PER_TILE {
      for cx in 0..PER_TILE {
        columns.push(self.column(&region, min, cx, cz));
      }
    }
    TileBlocks { columns }
  }

  /// The bricks above one `CHUNK` wide square of the tile.
  fn column(&self, region: &Region, min: ChunkPos, cx: usize, cz: usize) -> Vec<Brick> {
    let mut layers = vec![vec![0u8; CHUNK * CHUNK * CHUNK]; LAYERS];

    for lz in 0..CHUNK {
      for lx in 0..CHUNK {
        let x = cx * CHUNK + lx;
        let z = cz * CHUNK + lz;
        let chunk = region
          .chunk(ChunkPos::new(min.x + (x / 16) as i32, min.z + (z / 16) as i32))
          .expect("the region has every chunk of the tile");

        for y in 0..WORLD_HEIGHT {
          let state = chunk.get(ChunkRelPos::new((x % 16) as u8, y as i32, (z % 16) as u8));
          if state != StateId::AIR {
            layers[y / CHUNK][brick_index(lx, y % CHUNK, lz)] = self.palette.index(state);
          }
        }
      }
    }

    layers
      .into_iter()
      .map(|blocks| match blocks[0] {
        first if blocks.iter().all(|&b| b == first) => {
          if first == 0 {
            Brick::Empty
          } else {
            Brick::Uniform(first)
          }
        }
        _ => Brick::Blocks(Arc::from(blocks)),
      })
      .collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_tile_has_stone_below_and_sky_above() {
    let real = RealTerrain::new(1234);
    let world = WorldBiomes::new(&real.ctx.blocks, real.ctx.seed);
    let blocks = real.generate(&world, [3, -2]);

    assert_eq!(blocks.columns.len(), PER_TILE * PER_TILE);
    for column in &blocks.columns {
      assert_eq!(column.len(), LAYERS);
      // The bottom of the world is solid, and the top is air.
      assert!(!matches!(column[0], Brick::Empty));
      assert!(matches!(column[LAYERS - 1], Brick::Empty));
    }
    // Somewhere the surface passes through a chunk.
    assert!(blocks.columns.iter().flatten().any(|b| matches!(b, Brick::Blocks(_))));
  }

  #[test]
  fn the_palette_colors_air_as_index_zero() {
    let real = RealTerrain::new(1);
    assert_eq!(real.palette.index(StateId::AIR), 0);
    let stone = real.ctx.blocks.encode(rgen_base::block![stone]);
    assert_ne!(real.palette.index(stone), 0);
    let texture = real.palette.texture();
    assert_eq!(texture.len(), 256 * 4);
    let i = real.palette.index(stone) as usize * 4;
    assert_eq!(texture[i..i + 3], [125, 125, 125]);
  }
}
