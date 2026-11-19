use rgen_placer::{Random, Rng};

const TILE_BLOCKS: i32 = 4;
const SEED_SPACING: usize = 32;
const SIDE: usize = SEED_SPACING * 64;
const UNCLAIMED: u16 = u16::MAX;

/// World-seeded regions grown on a wrapping grid. The wrap keeps growth
/// continuous across the edges of the repeating sample.
pub(crate) struct OrganicVoronoi {
  owners: Vec<u16>,
  rolls:  Vec<u32>,
}

impl OrganicVoronoi {
  pub(crate) fn new(seed: u64) -> Self {
    let mut growth = Rng::new(seed);
    let mut rolls = Rng::new(seed ^ 0xB108_EA73_05C9_246D);
    let mut map = Self { owners: vec![UNCLAIMED; SIDE * SIDE], rolls: Vec::new() };
    let mut frontier = Vec::new();

    for z in (0..SIDE).step_by(SEED_SPACING) {
      for x in (0..SIDE).step_by(SEED_SPACING) {
        let x = x + growth.range(0..SEED_SPACING as i32) as usize;
        let z = z + growth.range(0..SEED_SPACING as i32) as usize;
        let tile = z * SIDE + x;
        map.owners[tile] = map.rolls.len() as u16;
        map.rolls.push(rolls.next() as u32);
        frontier.push(tile);
      }
    }

    while !frontier.is_empty() {
      let index = growth.range(0..frontier.len() as i32) as usize;
      let tile = frontier.swap_remove(index);
      let owner = map.owners[tile];
      let x = tile % SIDE;
      let row = tile - x;
      let neighbors = [
        row + ((x + SIDE - 1) % SIDE),
        row + ((x + 1) % SIDE),
        (tile + SIDE * SIDE - SIDE) % (SIDE * SIDE),
        (tile + SIDE) % (SIDE * SIDE),
      ];

      // A claim always extends from an owned tile, so a region can never
      // leave a disconnected island of its own tiles.
      for neighbor in neighbors {
        if map.owners[neighbor] == UNCLAIMED {
          map.owners[neighbor] = owner;
          frontier.push(neighbor);
        }
      }
    }

    map
  }

  pub(crate) fn sample(&self, x: i32, z: i32) -> u32 {
    let tile = |block: i32| block.div_euclid(TILE_BLOCKS).rem_euclid(SIDE as i32) as usize;
    self.rolls[self.owners[tile(z) * SIDE + tile(x)] as usize]
  }
}
