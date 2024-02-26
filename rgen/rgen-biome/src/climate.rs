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
