#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Climate {
  IceCap,
  Tundra,
  SubArctic,
  ColdSwamp,
  DryTemperate,
  CoolTemperate,
  WetTemperate,
  Savanna,
  WarmTemperate,
  HotDesert,
  Mediteranean,
  HotSwamp,
  HighDesert,
  Tropical,
}

#[rustfmt::skip]
const CLIMATE_MAP: [[Climate; 12]; 9] = {
  use Climate::*;

  [
    // Temperature increases downwards, and rainfall increases rightwards.
    [IceCap,       IceCap,       IceCap,       IceCap,        IceCap,        IceCap,        IceCap,        IceCap,        IceCap,        Tundra,       Tundra,       Tundra],
    [Tundra,       Tundra,       Tundra,       Tundra,        Tundra,        Tundra,        Tundra,        Tundra,        Tundra,        SubArctic,    SubArctic,    SubArctic],
    [Tundra,       Tundra,       SubArctic,    SubArctic,     SubArctic,     SubArctic,     SubArctic,     SubArctic,     SubArctic,     ColdSwamp,    ColdSwamp,    ColdSwamp],
    [DryTemperate, DryTemperate, DryTemperate, CoolTemperate, CoolTemperate, CoolTemperate, CoolTemperate, CoolTemperate, WetTemperate,  ColdSwamp,    ColdSwamp,    ColdSwamp],
    [DryTemperate, DryTemperate, DryTemperate, DryTemperate,  CoolTemperate, CoolTemperate, CoolTemperate, CoolTemperate, CoolTemperate, WetTemperate, WetTemperate, ColdSwamp],
    [Savanna,      Savanna,      DryTemperate, WarmTemperate, WarmTemperate, WarmTemperate, WarmTemperate, WarmTemperate, WarmTemperate, WetTemperate, WetTemperate, WetTemperate],
    [HotDesert,    HotDesert,    Savanna,      Mediteranean,  Mediteranean,  WarmTemperate, WarmTemperate, WarmTemperate, WetTemperate,  HotSwamp,     HotSwamp,     HotSwamp],
    [HighDesert,   HotDesert,    HotDesert,    Savanna,       Mediteranean,  Mediteranean,  WarmTemperate, WetTemperate,  HotSwamp,      HotSwamp,     Tropical,     Tropical],
    [HighDesert,   HighDesert,   HotDesert,    HotDesert,     Savanna,       Mediteranean,  Mediteranean,  HotSwamp,      HotSwamp,      Tropical,     Tropical,     Tropical],
  ]
};

// Given a temperature and rainfall from 0 to 1, returns the climate.
pub fn from_temperature_and_rainfall(temperature: f64, rainfall: f64) -> Climate {
  let temperature = temperature.clamp(0.0, 1.0);
  let rainfall = rainfall.clamp(0.0, 1.0);

  let t = (temperature * 9.0) as usize;
  let r = (rainfall * 12.0) as usize;

  CLIMATE_MAP[t][r]
}
