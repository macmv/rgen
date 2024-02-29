use rgen_base::{Biome, Block, Blocks, ChunkPos, Pos};
use rgen_placer::{grid::PointGrid, Placer, Random, Rng};
use rgen_world::PartialWorld;

pub enum PlacerStage {
  Sand,
  Sand2,
  Tree,
  Ore,
}

pub struct BiomeBuilder {
  pub name: &'static str,
  pub id:   rgen_base::Biome,

  pub top_block:      Block,
  pub top_block_data: u8,

  placers: Vec<PlacerBuilder>,
}

struct PlacerBuilder {
  placer: Box<dyn Placer>,
  grid:   PointGrid,
}

impl PlacerBuilder {
  fn new(placer: Box<dyn Placer>) -> Self { Self { placer, grid: PointGrid::new() } }
}

impl BiomeBuilder {
  pub fn new(name: &'static str, blocks: &Blocks) -> Self {
    Self { name, id: Biome::VOID, top_block: blocks.grass, top_block_data: 0, placers: vec![] }
  }

  pub fn place(&mut self, name: &str, stage: PlacerStage, placer: impl Placer + 'static) {
    // TODO: Do we even need name? Its a pain to add them later, so I'm keeping them
    // for now.
    let _ = name;

    self.place0(stage, Box::new(placer));
  }

  // Don't monomorphise this.
  fn place0(&mut self, _stage: PlacerStage, placer: Box<dyn Placer>) {
    // TODO: Using the stage, insert this at the right spot.
    self.placers.push(PlacerBuilder::new(placer));
  }

  /// Decorates the given chunk. The `rng` passed in should only be seeded with
  /// the world seed.
  pub fn decorate(
    &self,
    blocks: &Blocks,
    rng: &mut Rng,
    chunk_pos: ChunkPos,
    world: &mut PartialWorld,
  ) {
    for placer in self.placers.iter() {
      let seed = rng.next();

      const SCALE: f64 = 1.0 / 4.0;

      let min_x = chunk_pos.min_block_pos().x as f64 * SCALE;
      let min_y = chunk_pos.min_block_pos().z as f64 * SCALE;
      let max_x = (chunk_pos.min_block_pos().x + 15) as f64 * SCALE;
      let max_y = (chunk_pos.min_block_pos().z + 15) as f64 * SCALE;

      for point in placer.grid.points_in_area(seed, min_x, min_y, max_x, max_y) {
        let pos = world.top_block_excluding(
          Pos::new((point.0 / SCALE) as i32, 0, (point.1 / SCALE) as i32),
          &[blocks.leaves],
        );

        // This builds a unique seed for each placer. This gives the placer the same
        // seed if it crosses chunk boundaries.
        let seed = rng.next() ^ (pos.x as u64) << 32 ^ pos.z as u64;
        placer.placer.place(world, &mut Rng::new(seed), pos);
      }
    }
  }
}
