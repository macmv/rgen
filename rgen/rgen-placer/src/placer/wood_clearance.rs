use rgen_base::{BlockInfo, Pos};
use rgen_world::PartialWorld;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum WoodSpecies {
  Oak, Spruce, Birch, Jungle, Acacia, DarkOak,
  Fir, Palm, Cherry, Cedar, Mangrove, Lavender, Dead, Burned,
}

pub(crate) fn different_wood_adjacent(world: &PartialWorld, pos: Pos, species: WoodSpecies) -> bool {
  [(1, 0), (-1, 0), (0, 1), (0, -1)].iter().any(|&(x, z)| {
    wood_species(world.get(pos + Pos::new(x, 0, z)))
      .is_some_and(|neighbor| neighbor != species)
  })
}

pub(crate) fn any_wood_adjacent(world: &PartialWorld, pos: Pos) -> bool {
  [(1, 0), (-1, 0), (0, 1), (0, -1)].iter().any(|&(x, z)| {
    is_wood_log(world.get(pos + Pos::new(x, 0, z)))
  })
}

pub(crate) fn is_wood_log(state: BlockInfo<'_>) -> bool {
  wood_species(state).is_some()
}

fn wood_species(state: BlockInfo<'_>) -> Option<WoodSpecies> {
  use WoodSpecies::*;
  let kind = state.block_kind();
  let variant = (state.meta() & 3) as usize;
  if kind == block![log].block {
    return Some([Oak, Spruce, Birch, Jungle][variant]);
  }
  if kind == block![log2].block {
    return Some([Acacia, DarkOak][variant.min(1)]);
  }
  if kind == block![rgen:log].block {
    return Some([Fir, Palm, Cherry, Cedar][variant]);
  }
  if kind == block![rgen:log2].block {
    return Some([Mangrove, Lavender, Dead, Burned][variant]);
  }
  for (block, species) in [
    (block![rgen:covered_oak_log].block, Oak),
    (block![rgen:covered_birch_log].block, Birch),
    (block![rgen:covered_spruce_log].block, Spruce),
    (block![rgen:covered_jungle_log].block, Jungle),
    (block![rgen:covered_acacia_log].block, Acacia),
    (block![rgen:covered_dark_oak_log].block, DarkOak),
    (block![rgen:covered_fir_log].block, Fir),
    (block![rgen:covered_palm_log].block, Palm),
    (block![rgen:covered_cherry_log].block, Cherry),
    (block![rgen:covered_cedar_log].block, Cedar),
    (block![rgen:covered_dead_log].block, Dead),
  ] {
    if kind == block { return Some(species); }
  }
  if kind == block![rgen:mossy_stump].block {
    return Some(if variant & 1 == 0 { Oak } else { Birch });
  }
  None
}
