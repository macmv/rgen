mod biome;

pub struct BiomeBuilder {}

pub enum PlacerStage {
  Sand,
  Sand2,
  Tree,
  Ore,
}

impl BiomeBuilder {
  pub fn new() -> Self { Self {} }

  pub fn place(&mut self, name: &str, stage: PlacerStage, placer: u32 /* todo */) {}
}
