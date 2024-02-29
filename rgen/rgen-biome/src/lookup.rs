use rgen_base::Pos;
use rgen_placer::noise::NoiseGenerator;

use crate::WorldBiomes;

enum ContinentalnessCategory {
  Sea,
  Coast,
  NearInland,
  MidInland,
  FarInland,
}

enum PeaksValleysCategory {
  Valley,
  LowSlice,
  MidSlice,
  HighSlice,
  Peak,
}

impl WorldBiomes {
  pub fn choose_biome(&self, seed: u64, pos: Pos) {
    let continentalness = self.continentalness_category(seed, pos);

    let _: &str = match continentalness {
      ContinentalnessCategory::Sea => "minecraft:sea",
      ContinentalnessCategory::Coast => {
        // todo: coast table
        "coast"
      }

      // Inland cases
      _ => {
        let peaks_valleys = self.peaks_valleys_category(seed, pos);

        match peaks_valleys {
          PeaksValleysCategory::Valley => {
            let erosion = self.erosion_category(seed, pos);

            if erosion <= 4 {
              // river table
              "river"
            } else {
              // valley table
              "valley"
            }
          }

          PeaksValleysCategory::LowSlice => "",
          PeaksValleysCategory::MidSlice => "",
          PeaksValleysCategory::HighSlice => "",
          PeaksValleysCategory::Peak => "",
        }
      }
    };
  }

  fn continentalness_category(&self, seed: u64, pos: Pos) -> ContinentalnessCategory {
    let continentalness =
      self.continentalness_map.generate(pos.x as f64, pos.z as f64, seed) * 0.5 + 0.5;

    match continentalness {
      x if x < 0.1 => ContinentalnessCategory::Sea,
      x if x < 0.3 => ContinentalnessCategory::Coast,
      x if x < 0.6 => ContinentalnessCategory::NearInland,
      x if x < 0.8 => ContinentalnessCategory::MidInland,
      _ => ContinentalnessCategory::FarInland,
    }
  }

  fn peaks_valleys_category(&self, seed: u64, pos: Pos) -> PeaksValleysCategory {
    let seed = seed.wrapping_add(1);

    let peaks_valleys =
      self.peaks_valleys_map.generate(pos.x as f64, pos.z as f64, seed) * 0.5 + 0.5;

    match peaks_valleys {
      x if x < 0.075 => PeaksValleysCategory::Valley,
      x if x < 0.2 => PeaksValleysCategory::LowSlice,
      x if x < 0.6 => PeaksValleysCategory::MidSlice,
      x if x < 0.85 => PeaksValleysCategory::HighSlice,
      _ => PeaksValleysCategory::Peak,
    }
  }

  fn erosion_category(&self, seed: u64, pos: Pos) -> u8 {
    let seed = seed.wrapping_add(2);

    let erosion = self.peaks_valleys_map.generate(pos.x as f64, pos.z as f64, seed) * 0.5 + 0.5;

    // FIXME: This is dumb
    (erosion * 6.9999) as u8
  }
}
