use std::collections::HashMap;

use crate::{biome::*, builder::BiomeBuilder};

type BiomeFnCategory = &'static [(u32, &'static str, BiomeFn)];

pub type BiomeComposition = Vec<BiomeBuilder>;
type ClimateTable = &'static [&'static [ClimateType]];

macro_rules! b {
  ($rarity:expr, $f:expr) => {
    ($rarity, stringify!($f), $f as BiomeFn)
  };
}

pub struct CompositionLookup {
  pub blank: BiomeComposition,
  pub override_biome: Option<BiomeBuilder>,
  override_variants: HashMap<GeographicType, BiomeBuilder>,
  pub lookup: HashMap<(GeographicType, ClimateType), BiomeComposition>,
}

macro_rules! biome_categories {
  (
    fn build() {
      $(
        let ($geographic:expr, $climate:expr) = &[$($biome:expr),* $(,)?];
      )*
    }
  ) => {
    impl CompositionLookup {
      pub fn new(seed: u64) -> CompositionLookup {
        let mut lookup = HashMap::new();
        $(
          if lookup.insert(($geographic, $climate), composition(seed, &[$($biome),*])).is_some() {
            panic!("Duplicate biome for {:?}, {:?}", $geographic, $climate);
          }
        )*
        if let Some((climate, group)) = crate::feature::MIX_GROUP_OVERRIDE {
          let group_index = lookup
            .get(&(GeographicType::Standard, climate))
            .and_then(|mix| mix.iter().position(|biome| biome.name == group))
            .unwrap_or_else(|| panic!("Unknown mix group '{}' for {:?}", group, climate));

          for geographic in [
            GeographicType::Standard,
            GeographicType::River,
            GeographicType::Canyon,
            GeographicType::Hills,
            GeographicType::Mountains,
            GeographicType::Beach,
            GeographicType::Delta,
          ] {
            if let Some(mix) = lookup.get_mut(&(geographic, climate)) {
              if !mix.is_empty() {
                // Geography tables keep corresponding group rows in order. If a
                // geography combines later groups into fewer variants, use its
                // last available variant for those rows.
                let mut selected = mix.remove(group_index.min(mix.len() - 1));
                selected.rarity = 1;
                *mix = vec![selected];
              }
            }
          }
        }
        CompositionLookup {
          blank: composition(seed, &[b!(1, blank)]),
          override_biome: crate::feature::NAMED_BIOME_OVERRIDE
            .map(|(name, build)| BiomeBuilder::build(seed, name, 1, build)),
          override_variants: crate::feature::NAMED_BIOME_VARIANTS.iter()
            .filter(|(parent, _, _, _)| crate::feature::NAMED_BIOME_OVERRIDE.is_some_and(|(name, _)| name == *parent))
            .map(|(_, geo, name, build)| (*geo, BiomeBuilder::build(seed, name, 1, *build)))
            .collect(),
          lookup,
        }
      }
    }
  };
}

impl CompositionLookup {
  pub fn override_for(&self, geographic: GeographicType) -> Option<&BiomeBuilder> {
    self.override_biome.as_ref().map(|fallback| self.override_variants.get(&geographic).unwrap_or(fallback))
  }

  pub fn choose(&self, geographic: GeographicType, climate: ClimateType) -> &BiomeComposition {
    self.lookup.get(&(geographic, climate)).unwrap_or(&self.blank)
  }
}

// Climate compositions plus the GeoTest geography visualizer.
biome_categories!(
  fn build() {
    // Badlands groups: Painted Mesa 50%, Desert Scrub 30%, Dry Woods 20%.
    let (GeographicType::Standard, ClimateType::BadLands) = &[
      b!(50, painted_mesa),
      b!(30, red_scrubland),
      b!(20, arid_woodland),
    ];
    let (GeographicType::River, ClimateType::BadLands) = &[
      b!(50, painted_mesa_river),
      b!(30, red_scrubland_river),
      b!(20, arid_woodland_river),
    ];
    let (GeographicType::Canyon, ClimateType::BadLands) = &[
      b!(50, painted_mesa),
      b!(30, red_scrubland),
      b!(20, arid_woodland),
    ];
    let (GeographicType::Hills, ClimateType::BadLands) = &[
      b!(50, painted_mesa),
      b!(30, painted_mesa),
      b!(20, painted_mesa),
    ];
    let (GeographicType::Mountains, ClimateType::BadLands) = &[
      b!(50, painted_mesa),
      b!(30, painted_mesa),
      b!(20, painted_mesa),
    ];
    let (GeographicType::Beach, ClimateType::BadLands) = &[
      b!(50, painted_mesa_beach),
      b!(30, red_rock_shore),
      b!(20, red_rock_shore),
    ];
    let (GeographicType::Delta, ClimateType::BadLands) = &[
      b!(50, painted_mesa_delta),
      b!(30, red_reed_marsh),
      b!(20, red_reed_marsh),
    ];

    // Hot Desert groups: Open Dune 80%, Green Desert 10%, Salt Desert 10%.
    let (GeographicType::Standard, ClimateType::HotDesert) = &[
      b!(80, open_dune),
      b!(10, open_dune),
      b!(10, salt_flats_standard),
    ];
    let (GeographicType::River, ClimateType::HotDesert) = &[
      b!(80, open_dune_river),
      b!(10, lush_desert_river),
      b!(10, salt_flats_river),
    ];
    let (GeographicType::Canyon, ClimateType::HotDesert) = &[
      b!(80, open_dune_canyon),
      b!(10, lush_desert_canyon),
      b!(10, salt_flats_canyon),
    ];
    let (GeographicType::Hills, ClimateType::HotDesert) = &[
      b!(80, desert_mountain_hills),
      b!(10, desert_mountain_hills),
      b!(10, desert_mountain_hills_salt),
    ];
    let (GeographicType::Mountains, ClimateType::HotDesert) = &[
      b!(80, desert_mountain),
      b!(10, desert_mountain_scrub),
      b!(10, desert_mountain_salt),
    ];
    let (GeographicType::Beach, ClimateType::HotDesert) = &[
      b!(80, open_dune_beach),
      b!(10, lush_desert_beach),
      b!(10, salt_flats_beach),
    ];
    let (GeographicType::Delta, ClimateType::HotDesert) = &[
      b!(80, open_dune_delta),
      b!(10, lush_desert_delta),
      b!(10, salt_flats_delta),
    ];

    // Warm Temperate groups from the climate page.
    let (GeographicType::Standard, ClimateType::WarmTemperate) = &[
      b!(55, woodland),
      b!(25, birch_woodland),
      b!(20, cherry_blossom_grove),
    ];
    let (GeographicType::River, ClimateType::WarmTemperate) = &[
      b!(55, woodland_river),
      b!(25, birch_woodland),
      b!(20, cherry_blossom_grove_river),
    ];
    let (GeographicType::Canyon, ClimateType::WarmTemperate) = &[
      b!(55, fern_gully),
      b!(25, fern_gully),
      b!(20, fern_gully),
    ];
    let (GeographicType::Hills, ClimateType::WarmTemperate) = &[
      b!(55, woodland),
      b!(25, birch_woodland),
      b!(20, cherry_blossom_grove),
    ];
    let (GeographicType::Mountains, ClimateType::WarmTemperate) = &[
      b!(55, mountain_oak),
      b!(25, birch_aspen),
      b!(20, cherry_blossom_grove_mountains),
    ];
    let (GeographicType::Beach, ClimateType::WarmTemperate) = &[
      b!(55, woodland_beach),
      b!(25, birch_woodland),
      b!(20, lavender_beach),
    ];
    let (GeographicType::Delta, ClimateType::WarmTemperate) = &[
      b!(55, riparian_woodland),
      b!(25, cypress_swamp),
      b!(20, cypress_swamp),
    ];

    // One solid diagnostic surface color per landform.
    let (GeographicType::MushroomIsland, ClimateType::GeoTest) = &[b!(1, geo_test_mushroom_island)];
    let (GeographicType::Ocean, ClimateType::GeoTest) = &[b!(1, geo_test_ocean)];
    let (GeographicType::Beach, ClimateType::GeoTest) = &[b!(1, geo_test_beach)];
    let (GeographicType::Delta, ClimateType::GeoTest) = &[b!(1, geo_test_delta)];
    let (GeographicType::Canyon, ClimateType::GeoTest) = &[b!(1, geo_test_canyon)];
    let (GeographicType::River, ClimateType::GeoTest) = &[b!(1, geo_test_river)];
    let (GeographicType::Standard, ClimateType::GeoTest) = &[b!(1, geo_test_standard)];
    let (GeographicType::Hills, ClimateType::GeoTest) = &[b!(1, geo_test_hills)];
    let (GeographicType::Mountains, ClimateType::GeoTest) = &[b!(1, geo_test_mountains)];

    let (GeographicType::Standard, ClimateType::Mediterranean) = &[
      b!(20, lebanon_cedar_woodland),
      b!(45, chaparral_flats),
      b!(35, open_plains_standard),
    ];
    let (GeographicType::River, ClimateType::Mediterranean) = &[
      b!(20, lebanon_cedar_woodland_river),
      b!(45, chaparral_flats_river),
      b!(35, open_plains_river),
    ];
    let (GeographicType::Canyon, ClimateType::Mediterranean) = &[
      b!(20, lebanon_cedar_woodland_canyon),
      b!(45, chaparral_flats_canyon),
      b!(35, open_plains_canyon),
    ];
    let (GeographicType::Hills, ClimateType::Mediterranean) = &[
      b!(20, lebanon_cedar_woodland_hills),
      b!(45, chaparral_flats_hills),
      b!(35, open_plains_hills),
    ];
    let (GeographicType::Mountains, ClimateType::Mediterranean) = &[
      b!(20, lebanon_cedar_woodland_mountains),
      b!(45, chaparral_flats_mountains),
      b!(35, open_plains_mountains),
    ];
    let (GeographicType::Beach, ClimateType::Mediterranean) = &[
      b!(20, lebanon_cedar_woodland_beach),
      b!(80, coastal_dunes),
    ];
    let (GeographicType::Delta, ClimateType::Mediterranean) = &[
      b!(20, riparian_woodland),
      b!(80, salt_marsh),
    ];

    // Savanna groups: Wooded Savanna 30%, Savanna Scrub 25%, Open Savanna 45%.
    let (GeographicType::Standard, ClimateType::Savanna) = &[
      b!(30, wooded_savanna),
      b!(25, savanna_scrub),
      b!(45, open_savanna),
    ];
    let (GeographicType::River, ClimateType::Savanna) = &[
      b!(30, riparian_savanna_woodland),
      b!(25, riparian_savanna_woodland),
      b!(45, riparian_savanna_woodland),
    ];
    let (GeographicType::Canyon, ClimateType::Savanna) = &[
      b!(30, wooded_savanna),
      b!(25, savanna_scrub),
      b!(45, open_savanna),
    ];
    let (GeographicType::Hills, ClimateType::Savanna) = &[
      b!(30, wooded_savanna),
      b!(25, rocky_scrubland),
      b!(45, open_savanna),
    ];
    let (GeographicType::Mountains, ClimateType::Savanna) = &[
      b!(30, rocky_scrubland),
      b!(25, rocky_scrubland),
      b!(45, rocky_scrubland),
    ];
    let (GeographicType::Beach, ClimateType::Savanna) = &[
      b!(30, wooded_savanna),
      b!(25, savanna_dunes),
      b!(45, savanna_dunes),
    ];
    let (GeographicType::Delta, ClimateType::Savanna) = &[
      b!(30, flooded_savanna),
      b!(25, flooded_savanna),
      b!(45, flooded_savanna),
    ];
  }
);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GeographicType {
  MushroomIsland,
  Ocean,
  Beach,
  Delta,
  Canyon,
  River,
  Standard,
  Hills,
  Mountains,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ClimateType {
  IceCap,
  Tundra,
  SubArctic,
  CoolTemperate,
  DryTemperate,
  WarmTemperate,
  WetTemperate,
  Mediterranean,
  Monsoon,
  Savanna,
  HotDesert,
  BadLands,
  Tropical,
  DebugClimate,
  GeoTest,
}

use ClimateType::*;
#[rustfmt::skip]
pub const CLIMATE_TABLE: ClimateTable = &[
  &[IceCap, Tundra, Tundra, DryTemperate, Savanna, HotDesert, BadLands, BadLands],
  &[IceCap, Tundra, Tundra, DryTemperate, Savanna, HotDesert, HotDesert, BadLands],
  &[IceCap, Tundra, SubArctic, DryTemperate, DryTemperate, Savanna, HotDesert, HotDesert],
  &[IceCap, Tundra, SubArctic, DryTemperate, WarmTemperate, Mediterranean, Savanna, HotDesert],
  &[IceCap, Tundra, SubArctic, CoolTemperate, WarmTemperate, Mediterranean, Mediterranean, Savanna],
  &[IceCap, Tundra, SubArctic, CoolTemperate, WarmTemperate, WarmTemperate, Mediterranean, Mediterranean],
  &[IceCap, Tundra, SubArctic, CoolTemperate, WarmTemperate, WarmTemperate, WarmTemperate, Mediterranean],
  &[IceCap, Tundra, SubArctic, CoolTemperate, WarmTemperate, WarmTemperate, WetTemperate, WetTemperate],
  &[IceCap, Tundra, SubArctic, CoolTemperate, WarmTemperate, WetTemperate, WetTemperate, Monsoon],
  &[Tundra, SubArctic, CoolTemperate, CoolTemperate, WetTemperate, WetTemperate, Monsoon, Tropical],
  &[Tundra, SubArctic, CoolTemperate, WetTemperate, WetTemperate, Monsoon, Tropical, Tropical],
  &[Tundra, SubArctic, CoolTemperate, WetTemperate, WetTemperate, Monsoon, Tropical, Tropical],
];

fn composition(seed: u64, biome: BiomeFnCategory) -> BiomeComposition {
  biome.iter().map(|(rarity, name, f)| BiomeBuilder::build(seed, name, *rarity, *f)).collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn composition() { CompositionLookup::new(0); }

  #[test]
  fn badlands_variants_are_selected_in_full_mix() {
    let lookup = CompositionLookup::new(0);
    assert_eq!(lookup.choose(GeographicType::River, ClimateType::BadLands)[0].name, "painted_mesa_river");
    assert_eq!(lookup.choose(GeographicType::Standard, ClimateType::BadLands)[0].name, "painted_mesa");
    assert!(lookup.override_for(GeographicType::Standard).is_none());
    let delta = &lookup.choose(GeographicType::Delta, ClimateType::BadLands)[0];
    assert_eq!(delta.name, "painted_mesa_delta");
    assert_eq!(delta.top_block(), rgen_base::block![rgen:packed_mud]);
    assert_eq!(delta.underwater_layers[0].state, rgen_base::block![sand[variant = "red_sand"]]);
    assert_eq!(delta.underwater_layers[1].state, rgen_base::block![stained_hardened_clay[color = "orange"]]);
    assert_eq!((delta.underwater_layers[1].min_depth, delta.underwater_layers[1].max_depth), (2, 3));
    assert!(lookup.override_for(GeographicType::River).is_none());
    assert!(lookup.override_for(GeographicType::Delta).is_none());
  }
}
