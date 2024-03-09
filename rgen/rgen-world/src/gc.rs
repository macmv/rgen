use rgen_base::ChunkPos;

use crate::{CachedWorld, Stage};

impl CachedWorld {
  pub fn gc(&self) {
    let mut chunks = self.chunks.lock();
    let mut base_chunks = self.base_chunks.lock();
    let mut requester_chunks = self.requester.chunks.write();

    let chunks = &mut chunks.chunks;
    let mut gc = vec![];

    for (&pos, chunk) in chunks.iter() {
      match chunk.stage {
        // Base chunks: These can be GC'ed if none of the 8 surrounding chunks are decorated.
        Stage::Base => {
          let mut can_gc = true;
          'outer: for rel_x in -1..=1 {
            for rel_z in -1..=1 {
              if rel_x == 0 && rel_z == 0 {
                continue;
              }
              let pos = pos + ChunkPos::new(rel_x, rel_z);

              if chunks.get(&pos).map(|c| c.stage == Stage::Decorated).unwrap_or(false) {
                can_gc = false;
                break 'outer;
              }
            }
          }

          if can_gc {
            gc.push(pos);
          }
        }

        // Decorated chunks: These can be GC'ed if none of the 8 surrounding chunks are neighbor
        // decorated.
        Stage::Decorated => {
          let mut can_gc = true;
          'outer: for rel_x in -1..=1 {
            for rel_z in -1..=1 {
              if rel_x == 0 && rel_z == 0 {
                continue;
              }
              let pos = pos + ChunkPos::new(rel_x, rel_z);

              if chunks.get(&pos).map(|c| c.stage == Stage::NeighborDecorated).unwrap_or(false) {
                can_gc = false;
                break 'outer;
              }
            }
          }

          if can_gc {
            gc.push(pos);
          }
        }

        Stage::NeighborDecorated => gc.push(pos),
      }
    }

    for pos in gc {
      chunks.remove(&pos);
      base_chunks.remove(&pos);
      requester_chunks.remove(&pos);
    }
  }
}
