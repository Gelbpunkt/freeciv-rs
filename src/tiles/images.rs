// See <https://github.com/freeciv/freeciv/blob/main/data/trident/tiles.spec>

use std::{collections::HashMap, sync::LazyLock};

use image::{DynamicImage, ImageFormat};

pub static INACCESSIBLE: &[u8] = include_bytes!("../../assets/imgs/inaccessible.png");
pub static FOG: &[u8] = include_bytes!("../../assets/imgs/fog.png");

// Grassland

pub static GRASSLAND: &[u8] = include_bytes!("../../assets/imgs/grassland.png");

// Hills and whether terrain to north, south, east, west is more hills
pub static HILLS_NOT_EW: &[u8] = include_bytes!("../../assets/imgs/hills_not_ew.png");
pub static HILLS_E: &[u8] = include_bytes!("../../assets/imgs/hills_e.png");
pub static HILLS_EW: &[u8] = include_bytes!("../../assets/imgs/hills_ew.png");
pub static HILLS_W: &[u8] = include_bytes!("../../assets/imgs/hills_w.png");

// Forest and whether terrain to north, south, east, west is more forest
pub static FOREST_NOT_EW: &[u8] = include_bytes!("../../assets/imgs/forest_not_ew.png");
pub static FOREST_E: &[u8] = include_bytes!("../../assets/imgs/forest_e.png");
pub static FOREST_EW: &[u8] = include_bytes!("../../assets/imgs/forest_ew.png");
pub static FOREST_W: &[u8] = include_bytes!("../../assets/imgs/forest_w.png");

// Mountains and whether terrain to north, south, east, west is more mountains
pub static MOUNTAINS_NOT_EW: &[u8] = include_bytes!("../../assets/imgs/mountains_not_ew.png");
pub static MOUNTAINS_E: &[u8] = include_bytes!("../../assets/imgs/mountains_e.png");
pub static MOUNTAINS_EW: &[u8] = include_bytes!("../../assets/imgs/mountains_ew.png");
pub static MOUNTAINS_W: &[u8] = include_bytes!("../../assets/imgs/mountains_w.png");

// Desert and whether terrain to north, south, east, west is more desert
pub static DESERT_NESW: &[u8] = include_bytes!("../../assets/imgs/desert_nesw.png");
pub static DESERT_ESW: &[u8] = include_bytes!("../../assets/imgs/desert_esw.png");
pub static DESERT_NSW: &[u8] = include_bytes!("../../assets/imgs/desert_nsw.png");
pub static DESERT_SW: &[u8] = include_bytes!("../../assets/imgs/desert_sw.png");
pub static DESERT_NEW: &[u8] = include_bytes!("../../assets/imgs/desert_new.png");
pub static DESERT_EW: &[u8] = include_bytes!("../../assets/imgs/desert_ew.png");
pub static DESERT_NW: &[u8] = include_bytes!("../../assets/imgs/desert_nw.png");
pub static DESERT_W: &[u8] = include_bytes!("../../assets/imgs/desert_w.png");
pub static DESERT_NES: &[u8] = include_bytes!("../../assets/imgs/desert_nes.png");
pub static DESERT_ES: &[u8] = include_bytes!("../../assets/imgs/desert_es.png");
pub static DESERT_NS: &[u8] = include_bytes!("../../assets/imgs/desert_ns.png");
pub static DESERT_S: &[u8] = include_bytes!("../../assets/imgs/desert_s.png");
pub static DESERT_NE: &[u8] = include_bytes!("../../assets/imgs/desert_ne.png");
pub static DESERT_E: &[u8] = include_bytes!("../../assets/imgs/desert_e.png");
pub static DESERT_N: &[u8] = include_bytes!("../../assets/imgs/desert_n.png");
pub static DESERT_NONE: &[u8] = include_bytes!("../../assets/imgs/desert_none.png");

// Glacier and whether terrain to north, south, east, west is more glacier
pub static GLACIER_NESW: &[u8] = include_bytes!("../../assets/imgs/glacier_nesw.png");
pub static GLACIER_ESW: &[u8] = include_bytes!("../../assets/imgs/glacier_esw.png");
pub static GLACIER_NSW: &[u8] = include_bytes!("../../assets/imgs/glacier_nsw.png");
pub static GLACIER_SW: &[u8] = include_bytes!("../../assets/imgs/glacier_sw.png");
pub static GLACIER_NEW: &[u8] = include_bytes!("../../assets/imgs/glacier_new.png");
pub static GLACIER_EW: &[u8] = include_bytes!("../../assets/imgs/glacier_ew.png");
pub static GLACIER_NW: &[u8] = include_bytes!("../../assets/imgs/glacier_nw.png");
pub static GLACIER_W: &[u8] = include_bytes!("../../assets/imgs/glacier_w.png");
pub static GLACIER_NES: &[u8] = include_bytes!("../../assets/imgs/glacier_nes.png");
pub static GLACIER_ES: &[u8] = include_bytes!("../../assets/imgs/glacier_es.png");
pub static GLACIER_NS: &[u8] = include_bytes!("../../assets/imgs/glacier_ns.png");
pub static GLACIER_S: &[u8] = include_bytes!("../../assets/imgs/glacier_s.png");
pub static GLACIER_NE: &[u8] = include_bytes!("../../assets/imgs/glacier_ne.png");
pub static GLACIER_E: &[u8] = include_bytes!("../../assets/imgs/glacier_e.png");
pub static GLACIER_N: &[u8] = include_bytes!("../../assets/imgs/glacier_n.png");
pub static GLACIER_NONE: &[u8] = include_bytes!("../../assets/imgs/glacier_none.png");

// Tundra and whether terrain to north, south, east, west is more tundra
pub static TUNDRA_NESW: &[u8] = include_bytes!("../../assets/imgs/tundra_nesw.png");
pub static TUNDRA_ESW: &[u8] = include_bytes!("../../assets/imgs/tundra_esw.png");
pub static TUNDRA_NSW: &[u8] = include_bytes!("../../assets/imgs/tundra_nsw.png");
pub static TUNDRA_SW: &[u8] = include_bytes!("../../assets/imgs/tundra_sw.png");
pub static TUNDRA_NEW: &[u8] = include_bytes!("../../assets/imgs/tundra_new.png");
pub static TUNDRA_EW: &[u8] = include_bytes!("../../assets/imgs/tundra_ew.png");
pub static TUNDRA_NW: &[u8] = include_bytes!("../../assets/imgs/tundra_nw.png");
pub static TUNDRA_W: &[u8] = include_bytes!("../../assets/imgs/tundra_w.png");
pub static TUNDRA_NES: &[u8] = include_bytes!("../../assets/imgs/tundra_nes.png");
pub static TUNDRA_ES: &[u8] = include_bytes!("../../assets/imgs/tundra_es.png");
pub static TUNDRA_NS: &[u8] = include_bytes!("../../assets/imgs/tundra_ns.png");
pub static TUNDRA_S: &[u8] = include_bytes!("../../assets/imgs/tundra_s.png");
pub static TUNDRA_NE: &[u8] = include_bytes!("../../assets/imgs/tundra_ne.png");
pub static TUNDRA_E: &[u8] = include_bytes!("../../assets/imgs/tundra_e.png");
pub static TUNDRA_N: &[u8] = include_bytes!("../../assets/imgs/tundra_n.png");
pub static TUNDRA_NONE: &[u8] = include_bytes!("../../assets/imgs/tundra_none.png");

// Jungle and whether terrain to north, south, east, west is more jungle
pub static JUNGLE_NESW: &[u8] = include_bytes!("../../assets/imgs/jungle_nesw.png");
pub static JUNGLE_ESW: &[u8] = include_bytes!("../../assets/imgs/jungle_esw.png");
pub static JUNGLE_NSW: &[u8] = include_bytes!("../../assets/imgs/jungle_nsw.png");
pub static JUNGLE_SW: &[u8] = include_bytes!("../../assets/imgs/jungle_sw.png");
pub static JUNGLE_NEW: &[u8] = include_bytes!("../../assets/imgs/jungle_new.png");
pub static JUNGLE_EW: &[u8] = include_bytes!("../../assets/imgs/jungle_ew.png");
pub static JUNGLE_NW: &[u8] = include_bytes!("../../assets/imgs/jungle_nw.png");
pub static JUNGLE_W: &[u8] = include_bytes!("../../assets/imgs/jungle_w.png");
pub static JUNGLE_NES: &[u8] = include_bytes!("../../assets/imgs/jungle_nes.png");
pub static JUNGLE_ES: &[u8] = include_bytes!("../../assets/imgs/jungle_es.png");
pub static JUNGLE_NS: &[u8] = include_bytes!("../../assets/imgs/jungle_ns.png");
pub static JUNGLE_S: &[u8] = include_bytes!("../../assets/imgs/jungle_s.png");
pub static JUNGLE_NE: &[u8] = include_bytes!("../../assets/imgs/jungle_ne.png");
pub static JUNGLE_E: &[u8] = include_bytes!("../../assets/imgs/jungle_e.png");
pub static JUNGLE_N: &[u8] = include_bytes!("../../assets/imgs/jungle_n.png");
pub static JUNGLE_NONE: &[u8] = include_bytes!("../../assets/imgs/jungle_none.png");

// Plains and whether terrain to north, south, east, west is more plains
pub static PLAINS_NESW: &[u8] = include_bytes!("../../assets/imgs/plains_nesw.png");
pub static PLAINS_ESW: &[u8] = include_bytes!("../../assets/imgs/plains_esw.png");
pub static PLAINS_NSW: &[u8] = include_bytes!("../../assets/imgs/plains_nsw.png");
pub static PLAINS_SW: &[u8] = include_bytes!("../../assets/imgs/plains_sw.png");
pub static PLAINS_NEW: &[u8] = include_bytes!("../../assets/imgs/plains_new.png");
pub static PLAINS_EW: &[u8] = include_bytes!("../../assets/imgs/plains_ew.png");
pub static PLAINS_NW: &[u8] = include_bytes!("../../assets/imgs/plains_nw.png");
pub static PLAINS_W: &[u8] = include_bytes!("../../assets/imgs/plains_w.png");
pub static PLAINS_NES: &[u8] = include_bytes!("../../assets/imgs/plains_nes.png");
pub static PLAINS_ES: &[u8] = include_bytes!("../../assets/imgs/plains_es.png");
pub static PLAINS_NS: &[u8] = include_bytes!("../../assets/imgs/plains_ns.png");
pub static PLAINS_S: &[u8] = include_bytes!("../../assets/imgs/plains_s.png");
pub static PLAINS_NE: &[u8] = include_bytes!("../../assets/imgs/plains_ne.png");
pub static PLAINS_E: &[u8] = include_bytes!("../../assets/imgs/plains_e.png");
pub static PLAINS_N: &[u8] = include_bytes!("../../assets/imgs/plains_n.png");
pub static PLAINS_NONE: &[u8] = include_bytes!("../../assets/imgs/plains_none.png");

// Swamp and whether terrain to north, south, east, west is more swamp
pub static SWAMP_NESW: &[u8] = include_bytes!("../../assets/imgs/swamp_nesw.png");
pub static SWAMP_ESW: &[u8] = include_bytes!("../../assets/imgs/swamp_esw.png");
pub static SWAMP_NSW: &[u8] = include_bytes!("../../assets/imgs/swamp_nsw.png");
pub static SWAMP_SW: &[u8] = include_bytes!("../../assets/imgs/swamp_sw.png");
pub static SWAMP_NEW: &[u8] = include_bytes!("../../assets/imgs/swamp_new.png");
pub static SWAMP_EW: &[u8] = include_bytes!("../../assets/imgs/swamp_ew.png");
pub static SWAMP_NW: &[u8] = include_bytes!("../../assets/imgs/swamp_nw.png");
pub static SWAMP_W: &[u8] = include_bytes!("../../assets/imgs/swamp_w.png");
pub static SWAMP_NES: &[u8] = include_bytes!("../../assets/imgs/swamp_nes.png");
pub static SWAMP_ES: &[u8] = include_bytes!("../../assets/imgs/swamp_es.png");
pub static SWAMP_NS: &[u8] = include_bytes!("../../assets/imgs/swamp_ns.png");
pub static SWAMP_S: &[u8] = include_bytes!("../../assets/imgs/swamp_s.png");
pub static SWAMP_NE: &[u8] = include_bytes!("../../assets/imgs/swamp_ne.png");
pub static SWAMP_E: &[u8] = include_bytes!("../../assets/imgs/swamp_e.png");
pub static SWAMP_N: &[u8] = include_bytes!("../../assets/imgs/swamp_n.png");
pub static SWAMP_NONE: &[u8] = include_bytes!("../../assets/imgs/swamp_none.png");

// Water with shoreline and whether terrain to north, south, east, west is more
// water
pub static WATER_WITH_SHORELINE_NESW: &[u8] =
    include_bytes!("../../assets/imgs/water_with_shoreline_nesw.png");
pub static WATER_WITH_SHORELINE_ESW: &[u8] =
    include_bytes!("../../assets/imgs/water_with_shoreline_esw.png");
pub static WATER_WITH_SHORELINE_NSW: &[u8] =
    include_bytes!("../../assets/imgs/water_with_shoreline_nsw.png");
pub static WATER_WITH_SHORELINE_SW: &[u8] =
    include_bytes!("../../assets/imgs/water_with_shoreline_sw.png");
pub static WATER_WITH_SHORELINE_NEW: &[u8] =
    include_bytes!("../../assets/imgs/water_with_shoreline_new.png");
pub static WATER_WITH_SHORELINE_EW: &[u8] =
    include_bytes!("../../assets/imgs/water_with_shoreline_ew.png");
pub static WATER_WITH_SHORELINE_NW: &[u8] =
    include_bytes!("../../assets/imgs/water_with_shoreline_nw.png");
pub static WATER_WITH_SHORELINE_W: &[u8] =
    include_bytes!("../../assets/imgs/water_with_shoreline_w.png");
pub static WATER_WITH_SHORELINE_NES: &[u8] =
    include_bytes!("../../assets/imgs/water_with_shoreline_nes.png");
pub static WATER_WITH_SHORELINE_ES: &[u8] =
    include_bytes!("../../assets/imgs/water_with_shoreline_es.png");
pub static WATER_WITH_SHORELINE_NS: &[u8] =
    include_bytes!("../../assets/imgs/water_with_shoreline_ns.png");
pub static WATER_WITH_SHORELINE_S: &[u8] =
    include_bytes!("../../assets/imgs/water_with_shoreline_s.png");
pub static WATER_WITH_SHORELINE_NE: &[u8] =
    include_bytes!("../../assets/imgs/water_with_shoreline_ne.png");
pub static WATER_WITH_SHORELINE_E: &[u8] =
    include_bytes!("../../assets/imgs/water_with_shoreline_e.png");
pub static WATER_WITH_SHORELINE_N: &[u8] =
    include_bytes!("../../assets/imgs/water_with_shoreline_n.png");
pub static WATER_WITH_SHORELINE_NONE: &[u8] =
    include_bytes!("../../assets/imgs/water_with_shoreline_none.png");

// Water with ice shelves and whether terrain to north, south, east, west is
// more water
pub static WATER_WITH_ICE_SHELVES_NESW: &[u8] =
    include_bytes!("../../assets/imgs/water_with_ice_shelves_nesw.png");
pub static WATER_WITH_ICE_SHELVES_ESW: &[u8] =
    include_bytes!("../../assets/imgs/water_with_ice_shelves_esw.png");
pub static WATER_WITH_ICE_SHELVES_NSW: &[u8] =
    include_bytes!("../../assets/imgs/water_with_ice_shelves_nsw.png");
pub static WATER_WITH_ICE_SHELVES_SW: &[u8] =
    include_bytes!("../../assets/imgs/water_with_ice_shelves_sw.png");
pub static WATER_WITH_ICE_SHELVES_NEW: &[u8] =
    include_bytes!("../../assets/imgs/water_with_ice_shelves_new.png");
pub static WATER_WITH_ICE_SHELVES_EW: &[u8] =
    include_bytes!("../../assets/imgs/water_with_ice_shelves_ew.png");
pub static WATER_WITH_ICE_SHELVES_NW: &[u8] =
    include_bytes!("../../assets/imgs/water_with_ice_shelves_nw.png");
pub static WATER_WITH_ICE_SHELVES_W: &[u8] =
    include_bytes!("../../assets/imgs/water_with_ice_shelves_w.png");
pub static WATER_WITH_ICE_SHELVES_NES: &[u8] =
    include_bytes!("../../assets/imgs/water_with_ice_shelves_nes.png");
pub static WATER_WITH_ICE_SHELVES_ES: &[u8] =
    include_bytes!("../../assets/imgs/water_with_ice_shelves_es.png");
pub static WATER_WITH_ICE_SHELVES_NS: &[u8] =
    include_bytes!("../../assets/imgs/water_with_ice_shelves_ns.png");
pub static WATER_WITH_ICE_SHELVES_S: &[u8] =
    include_bytes!("../../assets/imgs/water_with_ice_shelves_s.png");
pub static WATER_WITH_ICE_SHELVES_NE: &[u8] =
    include_bytes!("../../assets/imgs/water_with_ice_shelves_ne.png");
pub static WATER_WITH_ICE_SHELVES_E: &[u8] =
    include_bytes!("../../assets/imgs/water_with_ice_shelves_e.png");
pub static WATER_WITH_ICE_SHELVES_N: &[u8] =
    include_bytes!("../../assets/imgs/water_with_ice_shelves_n.png");
pub static WATER_WITH_ICE_SHELVES_NONE: &[u8] =
    include_bytes!("../../assets/imgs/water_with_ice_shelves_none.png");

// Darkness (unexplored) to north, south, east, west
pub static DARKNESS: &[u8] = include_bytes!("../../assets/imgs/darkness.png");
pub static DARKNESS_N: &[u8] = include_bytes!("../../assets/imgs/darkness_n.png");
pub static DARKNESS_E: &[u8] = include_bytes!("../../assets/imgs/darkness_e.png");
pub static DARKNESS_NE: &[u8] = include_bytes!("../../assets/imgs/darkness_ne.png");
pub static DARKNESS_S: &[u8] = include_bytes!("../../assets/imgs/darkness_s.png");
pub static DARKNESS_NS: &[u8] = include_bytes!("../../assets/imgs/darkness_ns.png");
pub static DARKNESS_ES: &[u8] = include_bytes!("../../assets/imgs/darkness_es.png");
pub static DARKNESS_NES: &[u8] = include_bytes!("../../assets/imgs/darkness_nes.png");
pub static DARKNESS_W: &[u8] = include_bytes!("../../assets/imgs/darkness_w.png");
pub static DARKNESS_NW: &[u8] = include_bytes!("../../assets/imgs/darkness_nw.png");
pub static DARKNESS_EW: &[u8] = include_bytes!("../../assets/imgs/darkness_ew.png");
pub static DARKNESS_NEW: &[u8] = include_bytes!("../../assets/imgs/darkness_new.png");
pub static DARKNESS_SW: &[u8] = include_bytes!("../../assets/imgs/darkness_sw.png");
pub static DARKNESS_NSW: &[u8] = include_bytes!("../../assets/imgs/darkness_nsw.png");
pub static DARKNESS_ESW: &[u8] = include_bytes!("../../assets/imgs/darkness_esw.png");
pub static DARKNESS_NESW: &[u8] = include_bytes!("../../assets/imgs/darkness_nesw.png");

// Rivers (as special type) and whether north, south, east, west also has river
// or ocean
pub static RIVER: &[u8] = include_bytes!("../../assets/imgs/river.png");
pub static RIVER_N: &[u8] = include_bytes!("../../assets/imgs/river_n.png");
pub static RIVER_E: &[u8] = include_bytes!("../../assets/imgs/river_e.png");
pub static RIVER_NE: &[u8] = include_bytes!("../../assets/imgs/river_ne.png");
pub static RIVER_S: &[u8] = include_bytes!("../../assets/imgs/river_s.png");
pub static RIVER_NS: &[u8] = include_bytes!("../../assets/imgs/river_ns.png");
pub static RIVER_ES: &[u8] = include_bytes!("../../assets/imgs/river_es.png");
pub static RIVER_NES: &[u8] = include_bytes!("../../assets/imgs/river_nes.png");
pub static RIVER_W: &[u8] = include_bytes!("../../assets/imgs/river_w.png");
pub static RIVER_NW: &[u8] = include_bytes!("../../assets/imgs/river_nw.png");
pub static RIVER_EW: &[u8] = include_bytes!("../../assets/imgs/river_ew.png");
pub static RIVER_NEW: &[u8] = include_bytes!("../../assets/imgs/river_new.png");
pub static RIVER_SW: &[u8] = include_bytes!("../../assets/imgs/river_sw.png");
pub static RIVER_NSW: &[u8] = include_bytes!("../../assets/imgs/river_nsw.png");
pub static RIVER_ESW: &[u8] = include_bytes!("../../assets/imgs/river_esw.png");
pub static RIVER_NESW: &[u8] = include_bytes!("../../assets/imgs/river_nesw.png");

// River outlets, river to north, south, east, west
pub static RIVER_OUTLET_N: &[u8] = include_bytes!("../../assets/imgs/river_outlet_n.png");
pub static RIVER_OUTLET_W: &[u8] = include_bytes!("../../assets/imgs/river_outlet_w.png");
pub static RIVER_OUTLET_S: &[u8] = include_bytes!("../../assets/imgs/river_outlet_s.png");
pub static RIVER_OUTLET_E: &[u8] = include_bytes!("../../assets/imgs/river_outlet_e.png");

// Terrain special resources
pub static SPICE: &[u8] = include_bytes!("../../assets/imgs/spice.png");
pub static FURS: &[u8] = include_bytes!("../../assets/imgs/furs.png");
pub static PEAT: &[u8] = include_bytes!("../../assets/imgs/peat.png");
pub static IVORY: &[u8] = include_bytes!("../../assets/imgs/ivory.png");
pub static FRUIT: &[u8] = include_bytes!("../../assets/imgs/fruit.png");
pub static IRON: &[u8] = include_bytes!("../../assets/imgs/iron.png");
pub static WHALES: &[u8] = include_bytes!("../../assets/imgs/whales.png");
pub static WHEAT: &[u8] = include_bytes!("../../assets/imgs/wheat.png");
pub static PHEASANT: &[u8] = include_bytes!("../../assets/imgs/pheasant.png");
pub static BUFFALO: &[u8] = include_bytes!("../../assets/imgs/buffalo.png");
pub static SILK: &[u8] = include_bytes!("../../assets/imgs/silk.png");
pub static WINE: &[u8] = include_bytes!("../../assets/imgs/wine.png");

pub static SEALS: &[u8] = include_bytes!("../../assets/imgs/seals.png");
pub static OASIS: &[u8] = include_bytes!("../../assets/imgs/oasis.png");
pub static FOREST_GAME: &[u8] = include_bytes!("../../assets/imgs/forest_game.png");
pub static GRASSLAND_RESOURCES: &[u8] = include_bytes!("../../assets/imgs/grassland_resources.png");
pub static COAL: &[u8] = include_bytes!("../../assets/imgs/coal.png");
pub static GEMS: &[u8] = include_bytes!("../../assets/imgs/gems.png");
pub static GOLD: &[u8] = include_bytes!("../../assets/imgs/gold.png");
pub static FISH: &[u8] = include_bytes!("../../assets/imgs/fish.png");
pub static HORSES: &[u8] = include_bytes!("../../assets/imgs/horses.png");
pub static RIVER_RESOURCES: &[u8] = include_bytes!("../../assets/imgs/river_resources.png");
pub static OIL: &[u8] = include_bytes!("../../assets/imgs/oil.png");
pub static TUNDRA_GAME: &[u8] = include_bytes!("../../assets/imgs/tundra_game.png");

// Terrain strategic resources
pub static ALUMINUM: &[u8] = include_bytes!("../../assets/imgs/aluminum.png");
pub static URANIUM: &[u8] = include_bytes!("../../assets/imgs/uranium.png");
pub static SALTPETER: &[u8] = include_bytes!("../../assets/imgs/saltpeter.png");
pub static ELEPHANT: &[u8] = include_bytes!("../../assets/imgs/elephant.png");

// Terrain improvements and similar
pub static FARMLAND: &[u8] = include_bytes!("../../assets/imgs/farmland.png");
pub static IRRIGATION: &[u8] = include_bytes!("../../assets/imgs/irrigation.png");
pub static MINE: &[u8] = include_bytes!("../../assets/imgs/mine.png");
pub static OIL_MINE: &[u8] = include_bytes!("../../assets/imgs/oil_mine.png");
pub static POLLUTION: &[u8] = include_bytes!("../../assets/imgs/pollution.png");
pub static FALLOUT: &[u8] = include_bytes!("../../assets/imgs/fallout.png");
pub static OIL_RIG: &[u8] = include_bytes!("../../assets/imgs/oil_rig.png");

// Bases
pub static BUOY: &[u8] = include_bytes!("../../assets/imgs/buoy.png");
pub static RUINS: &[u8] = include_bytes!("../../assets/imgs/ruins.png");
pub static VILLAGE: &[u8] = include_bytes!("../../assets/imgs/village.png");
pub static AIRSTRIP: &[u8] = include_bytes!("../../assets/imgs/airstrip.png");
pub static AIRBASE: &[u8] = include_bytes!("../../assets/imgs/airbase.png");
pub static OUTPOST: &[u8] = include_bytes!("../../assets/imgs/outpost.png");
pub static FORTRESS: &[u8] = include_bytes!("../../assets/imgs/fortress.png");

// Numbers: city size (also used for goto)
pub static CITY_SIZE_0: &[u8] = include_bytes!("../../assets/imgs/city_size_0.png");
pub static CITY_SIZE_1: &[u8] = include_bytes!("../../assets/imgs/city_size_1.png");
pub static CITY_SIZE_2: &[u8] = include_bytes!("../../assets/imgs/city_size_2.png");
pub static CITY_SIZE_3: &[u8] = include_bytes!("../../assets/imgs/city_size_3.png");
pub static CITY_SIZE_4: &[u8] = include_bytes!("../../assets/imgs/city_size_4.png");
pub static CITY_SIZE_5: &[u8] = include_bytes!("../../assets/imgs/city_size_5.png");
pub static CITY_SIZE_6: &[u8] = include_bytes!("../../assets/imgs/city_size_6.png");
pub static CITY_SIZE_7: &[u8] = include_bytes!("../../assets/imgs/city_size_7.png");
pub static CITY_SIZE_8: &[u8] = include_bytes!("../../assets/imgs/city_size_8.png");
pub static CITY_SIZE_9: &[u8] = include_bytes!("../../assets/imgs/city_size_9.png");
pub static CITY_SIZE_00: &[u8] = include_bytes!("../../assets/imgs/city_size_00.png");
pub static CITY_SIZE_10: &[u8] = include_bytes!("../../assets/imgs/city_size_10.png");
pub static CITY_SIZE_20: &[u8] = include_bytes!("../../assets/imgs/city_size_20.png");
pub static CITY_SIZE_30: &[u8] = include_bytes!("../../assets/imgs/city_size_30.png");
pub static CITY_SIZE_40: &[u8] = include_bytes!("../../assets/imgs/city_size_40.png");
pub static CITY_SIZE_50: &[u8] = include_bytes!("../../assets/imgs/city_size_50.png");
pub static CITY_SIZE_60: &[u8] = include_bytes!("../../assets/imgs/city_size_60.png");
pub static CITY_SIZE_70: &[u8] = include_bytes!("../../assets/imgs/city_size_70.png");
pub static CITY_SIZE_80: &[u8] = include_bytes!("../../assets/imgs/city_size_80.png");
pub static CITY_SIZE_90: &[u8] = include_bytes!("../../assets/imgs/city_size_90.png");
pub static CITY_SIZE_100: &[u8] = include_bytes!("../../assets/imgs/city_size_100.png");
pub static CITY_SIZE_200: &[u8] = include_bytes!("../../assets/imgs/city_size_200.png");
pub static CITY_SIZE_300: &[u8] = include_bytes!("../../assets/imgs/city_size_300.png");
pub static CITY_SIZE_400: &[u8] = include_bytes!("../../assets/imgs/city_size_400.png");
pub static CITY_SIZE_500: &[u8] = include_bytes!("../../assets/imgs/city_size_500.png");
pub static CITY_SIZE_600: &[u8] = include_bytes!("../../assets/imgs/city_size_600.png");
pub static CITY_SIZE_700: &[u8] = include_bytes!("../../assets/imgs/city_size_700.png");
pub static CITY_SIZE_800: &[u8] = include_bytes!("../../assets/imgs/city_size_800.png");
pub static CITY_SIZE_900: &[u8] = include_bytes!("../../assets/imgs/city_size_900.png");

// Numbers: city tile food/shields/trade y/g/b
pub static FOOD_0: &[u8] = include_bytes!("../../assets/imgs/food_0.png");
pub static FOOD_1: &[u8] = include_bytes!("../../assets/imgs/food_1.png");
pub static FOOD_2: &[u8] = include_bytes!("../../assets/imgs/food_2.png");
pub static FOOD_3: &[u8] = include_bytes!("../../assets/imgs/food_3.png");
pub static FOOD_4: &[u8] = include_bytes!("../../assets/imgs/food_4.png");
pub static FOOD_5: &[u8] = include_bytes!("../../assets/imgs/food_5.png");
pub static FOOD_6: &[u8] = include_bytes!("../../assets/imgs/food_6.png");
pub static FOOD_7: &[u8] = include_bytes!("../../assets/imgs/food_7.png");
pub static FOOD_8: &[u8] = include_bytes!("../../assets/imgs/food_8.png");
pub static FOOD_9: &[u8] = include_bytes!("../../assets/imgs/food_9.png");

pub static SHIELDS_0: &[u8] = include_bytes!("../../assets/imgs/shields_0.png");
pub static SHIELDS_1: &[u8] = include_bytes!("../../assets/imgs/shields_1.png");
pub static SHIELDS_2: &[u8] = include_bytes!("../../assets/imgs/shields_2.png");
pub static SHIELDS_3: &[u8] = include_bytes!("../../assets/imgs/shields_3.png");
pub static SHIELDS_4: &[u8] = include_bytes!("../../assets/imgs/shields_4.png");
pub static SHIELDS_5: &[u8] = include_bytes!("../../assets/imgs/shields_5.png");
pub static SHIELDS_6: &[u8] = include_bytes!("../../assets/imgs/shields_6.png");
pub static SHIELDS_7: &[u8] = include_bytes!("../../assets/imgs/shields_7.png");
pub static SHIELDS_8: &[u8] = include_bytes!("../../assets/imgs/shields_8.png");
pub static SHIELDS_9: &[u8] = include_bytes!("../../assets/imgs/shields_9.png");

pub static TRADE_0: &[u8] = include_bytes!("../../assets/imgs/trade_0.png");
pub static TRADE_1: &[u8] = include_bytes!("../../assets/imgs/trade_1.png");
pub static TRADE_2: &[u8] = include_bytes!("../../assets/imgs/trade_2.png");
pub static TRADE_3: &[u8] = include_bytes!("../../assets/imgs/trade_3.png");
pub static TRADE_4: &[u8] = include_bytes!("../../assets/imgs/trade_4.png");
pub static TRADE_5: &[u8] = include_bytes!("../../assets/imgs/trade_5.png");
pub static TRADE_6: &[u8] = include_bytes!("../../assets/imgs/trade_6.png");
pub static TRADE_7: &[u8] = include_bytes!("../../assets/imgs/trade_7.png");
pub static TRADE_8: &[u8] = include_bytes!("../../assets/imgs/trade_8.png");
pub static TRADE_9: &[u8] = include_bytes!("../../assets/imgs/trade_9.png");

// Unit Misc
pub static UNIT_TIRED: &[u8] = include_bytes!("../../assets/imgs/unit_tired.png"); // Also lowfuel
pub static UNIT_LOADED: &[u8] = include_bytes!("../../assets/imgs/unit_loaded.png");
pub static UNIT_ATTENTION: &[u8] = include_bytes!("../../assets/imgs/unit_attention.png"); // Variously crosshair/red-square/arrows
pub static UNIT_STACK: &[u8] = include_bytes!("../../assets/imgs/unit_stack.png");

// Goto path
pub static PATH_STEP: &[u8] = include_bytes!("../../assets/imgs/path_step.png");
pub static PATH_EXHAUSTED: &[u8] = include_bytes!("../../assets/imgs/path_step.png");
pub static PATH_NORMAL: &[u8] = include_bytes!("../../assets/imgs/path_step.png");
pub static PATH_WAYPOINT: &[u8] = include_bytes!("../../assets/imgs/path_step.png");

// Unit activity letters
pub static UNIT_AUTO_ATTACK: &[u8] = include_bytes!("../../assets/imgs/unit_auto_attack.png"); // Also auto_settler
pub static UNIT_CONNECT: &[u8] = include_bytes!("../../assets/imgs/unit_connect.png");
pub static UNIT_AUTO_EXPLORE: &[u8] = include_bytes!("../../assets/imgs/unit_auto_explore.png");

pub static UNIT_FORTIFYING: &[u8] = include_bytes!("../../assets/imgs/unit_fortifying.png");
pub static UNIT_FORTIFIED: &[u8] = include_bytes!("../../assets/imgs/unit_fortified.png");
pub static UNIT_SENTRY: &[u8] = include_bytes!("../../assets/imgs/unit_sentry.png");
pub static UNIT_PATROL: &[u8] = include_bytes!("../../assets/imgs/unit_patrol.png");

pub static UNIT_MINE: &[u8] = include_bytes!("../../assets/imgs/unit_mine.png"); // Also plant
pub static UNIT_IRRIGATE: &[u8] = include_bytes!("../../assets/imgs/unit_irrigate.png"); // Also cultivate
pub static UNIT_TRANSFORM: &[u8] = include_bytes!("../../assets/imgs/unit_transform.png");
pub static UNIT_PILLAGE: &[u8] = include_bytes!("../../assets/imgs/unit_pillage.png");

pub static UNIT_POLLUTION: &[u8] = include_bytes!("../../assets/imgs/unit_pollution.png");
pub static UNIT_FALLOUT: &[u8] = include_bytes!("../../assets/imgs/unit_fallout.png");
pub static UNIT_CONVERT: &[u8] = include_bytes!("../../assets/imgs/unit_convert.png");
pub static UNIT_GOTO: &[u8] = include_bytes!("../../assets/imgs/unit_goto.png");

// Unit activities
pub static UNIT_AIRSTRIP: &[u8] = include_bytes!("../../assets/imgs/unit_airstrip.png");
pub static UNIT_OUTPOST: &[u8] = include_bytes!("../../assets/imgs/unit_outpost.png");
pub static UNIT_AIRBASE: &[u8] = include_bytes!("../../assets/imgs/unit_airbase.png");
pub static UNIT_FORTRESS: &[u8] = include_bytes!("../../assets/imgs/unit_fortress.png");
pub static UNIT_BUOY: &[u8] = include_bytes!("../../assets/imgs/unit_buoy.png");

// Road activities
pub static UNIT_ROAD: &[u8] = include_bytes!("../../assets/imgs/unit_road.png");
pub static UNIT_RAIL: &[u8] = include_bytes!("../../assets/imgs/unit_rail.png");
pub static UNIT_MAGLEV: &[u8] = include_bytes!("../../assets/imgs/unit_maglev.png");

// Unit hit-point bars: approx percent of hp remaining
pub static UNIT_HP_100: &[u8] = include_bytes!("../../assets/imgs/unit_hp_100.png");
pub static UNIT_HP_90: &[u8] = include_bytes!("../../assets/imgs/unit_hp_90.png");
pub static UNIT_HP_80: &[u8] = include_bytes!("../../assets/imgs/unit_hp_80.png");
pub static UNIT_HP_70: &[u8] = include_bytes!("../../assets/imgs/unit_hp_70.png");
pub static UNIT_HP_60: &[u8] = include_bytes!("../../assets/imgs/unit_hp_60.png");
pub static UNIT_HP_50: &[u8] = include_bytes!("../../assets/imgs/unit_hp_50.png");
pub static UNIT_HP_40: &[u8] = include_bytes!("../../assets/imgs/unit_hp_40.png");
pub static UNIT_HP_30: &[u8] = include_bytes!("../../assets/imgs/unit_hp_30.png");
pub static UNIT_HP_20: &[u8] = include_bytes!("../../assets/imgs/unit_hp_20.png");
pub static UNIT_HP_10: &[u8] = include_bytes!("../../assets/imgs/unit_hp_10.png");
pub static UNIT_HP_0: &[u8] = include_bytes!("../../assets/imgs/unit_hp_0.png");

// Veteran levels: up to 9 military honors for experienced units
pub static UNIT_VET_1: &[u8] = include_bytes!("../../assets/imgs/unit_vet_1.png");
pub static UNIT_VET_2: &[u8] = include_bytes!("../../assets/imgs/unit_vet_2.png");
pub static UNIT_VET_3: &[u8] = include_bytes!("../../assets/imgs/unit_vet_3.png");
pub static UNIT_VET_4: &[u8] = include_bytes!("../../assets/imgs/unit_vet_4.png");
pub static UNIT_VET_5: &[u8] = include_bytes!("../../assets/imgs/unit_vet_5.png");
pub static UNIT_VET_6: &[u8] = include_bytes!("../../assets/imgs/unit_vet_6.png");
pub static UNIT_VET_7: &[u8] = include_bytes!("../../assets/imgs/unit_vet_7.png");
pub static UNIT_VET_8: &[u8] = include_bytes!("../../assets/imgs/unit_vet_8.png");
pub static UNIT_VET_9: &[u8] = include_bytes!("../../assets/imgs/unit_vet_9.png");

// Unit upkeep in city dialog:
// These should probably be handled differently and have a different size
pub static UPKEEP_SHIELD_1: &[u8] = include_bytes!("../../assets/imgs/upkeep_shield_1.png");
pub static UPKEEP_SHIELD_2: &[u8] = include_bytes!("../../assets/imgs/upkeep_shield_2.png");
pub static UPKEEP_SHIELD_3: &[u8] = include_bytes!("../../assets/imgs/upkeep_shield_3.png");
pub static UPKEEP_SHIELD_4: &[u8] = include_bytes!("../../assets/imgs/upkeep_shield_4.png");
pub static UPKEEP_SHIELD_5: &[u8] = include_bytes!("../../assets/imgs/upkeep_shield_5.png");
pub static UPKEEP_SHIELD_6: &[u8] = include_bytes!("../../assets/imgs/upkeep_shield_6.png");
pub static UPKEEP_SHIELD_7: &[u8] = include_bytes!("../../assets/imgs/upkeep_shield_7.png");
pub static UPKEEP_SHIELD_8: &[u8] = include_bytes!("../../assets/imgs/upkeep_shield_8.png");
pub static UPKEEP_SHIELD_9: &[u8] = include_bytes!("../../assets/imgs/upkeep_shield_9.png");
pub static UPKEEP_SHIELD_10: &[u8] = include_bytes!("../../assets/imgs/upkeep_shield_10.png");

pub static UPKEEP_UNHAPPY_1: &[u8] = include_bytes!("../../assets/imgs/upkeep_unhappy_1.png");
pub static UPKEEP_UNHAPPY_2: &[u8] = include_bytes!("../../assets/imgs/upkeep_unhappy_2.png");
pub static UPKEEP_UNHAPPY_3: &[u8] = include_bytes!("../../assets/imgs/upkeep_unhappy_3.png");
pub static UPKEEP_UNHAPPY_4: &[u8] = include_bytes!("../../assets/imgs/upkeep_unhappy_4.png");
pub static UPKEEP_UNHAPPY_5: &[u8] = include_bytes!("../../assets/imgs/upkeep_unhappy_5.png");
pub static UPKEEP_UNHAPPY_6: &[u8] = include_bytes!("../../assets/imgs/upkeep_unhappy_6.png");
pub static UPKEEP_UNHAPPY_7: &[u8] = include_bytes!("../../assets/imgs/upkeep_unhappy_7.png");
pub static UPKEEP_UNHAPPY_8: &[u8] = include_bytes!("../../assets/imgs/upkeep_unhappy_8.png");
pub static UPKEEP_UNHAPPY_9: &[u8] = include_bytes!("../../assets/imgs/upkeep_unhappy_9.png");
pub static UPKEEP_UNHAPPY_10: &[u8] = include_bytes!("../../assets/imgs/upkeep_unhappy_10.png");

pub static UPKEEP_FOOD_1: &[u8] = include_bytes!("../../assets/imgs/upkeep_food_1.png");
pub static UPKEEP_FOOD_2: &[u8] = include_bytes!("../../assets/imgs/upkeep_food_2.png");
pub static UPKEEP_FOOD_3: &[u8] = include_bytes!("../../assets/imgs/upkeep_food_3.png");
pub static UPKEEP_FOOD_4: &[u8] = include_bytes!("../../assets/imgs/upkeep_food_4.png");
pub static UPKEEP_FOOD_5: &[u8] = include_bytes!("../../assets/imgs/upkeep_food_5.png");
pub static UPKEEP_FOOD_6: &[u8] = include_bytes!("../../assets/imgs/upkeep_food_6.png");
pub static UPKEEP_FOOD_7: &[u8] = include_bytes!("../../assets/imgs/upkeep_food_7.png");
pub static UPKEEP_FOOD_8: &[u8] = include_bytes!("../../assets/imgs/upkeep_food_8.png");
pub static UPKEEP_FOOD_9: &[u8] = include_bytes!("../../assets/imgs/upkeep_food_9.png");
pub static UPKEEP_FOOD_10: &[u8] = include_bytes!("../../assets/imgs/upkeep_food_10.png");

pub static UPKEEP_GOLD_1: &[u8] = include_bytes!("../../assets/imgs/upkeep_gold_1.png");
pub static UPKEEP_GOLD_2: &[u8] = include_bytes!("../../assets/imgs/upkeep_gold_2.png");
pub static UPKEEP_GOLD_3: &[u8] = include_bytes!("../../assets/imgs/upkeep_gold_3.png");
pub static UPKEEP_GOLD_4: &[u8] = include_bytes!("../../assets/imgs/upkeep_gold_4.png");
pub static UPKEEP_GOLD_5: &[u8] = include_bytes!("../../assets/imgs/upkeep_gold_5.png");
pub static UPKEEP_GOLD_6: &[u8] = include_bytes!("../../assets/imgs/upkeep_gold_6.png");
pub static UPKEEP_GOLD_7: &[u8] = include_bytes!("../../assets/imgs/upkeep_gold_7.png");
pub static UPKEEP_GOLD_8: &[u8] = include_bytes!("../../assets/imgs/upkeep_gold_8.png");
pub static UPKEEP_GOLD_9: &[u8] = include_bytes!("../../assets/imgs/upkeep_gold_9.png");
pub static UPKEEP_GOLD_10: &[u8] = include_bytes!("../../assets/imgs/upkeep_gold_10.png");

pub static NUKE: &[u8] = include_bytes!("../../assets/imgs/nuke.png");

// For matched terrains that have cell_type "rect",
// 32 different sprites are needed. Each sprite is
// a rectangle corresponding to one cell, and there are
// 8 different sprites per cell. Each sprite has
// a name like "t.ocean_cell_u110" where "ocean" is the
// terrain, "u" means up (north on the map) and
// 110 indicates which of the adjacent tiles are
// mismatched. For instance u110 means
//
//              /\
//             /B \
//            /\ 1/\
//           / A\/C \
//           \1 /\ 0/
//            \/D \/
//             \  /
//              \/
//
// a matching terrain exists at C but not at A or B. In
// this case D is the current tile.
pub static OCEAN_TL_N: &[u8] = include_bytes!("../../assets/imgs/ocean_tl_n.png");
pub static OCEAN_TL_Y: &[u8] = include_bytes!("../../assets/imgs/ocean_tl_y.png");
pub static OCEAN_TR_N: &[u8] = include_bytes!("../../assets/imgs/ocean_tr_n.png");
pub static OCEAN_TR_Y: &[u8] = include_bytes!("../../assets/imgs/ocean_tr_y.png");
pub static OCEAN_BL_N: &[u8] = include_bytes!("../../assets/imgs/ocean_bl_n.png");
pub static OCEAN_BL_Y: &[u8] = include_bytes!("../../assets/imgs/ocean_bl_y.png");
pub static OCEAN_BR_N: &[u8] = include_bytes!("../../assets/imgs/ocean_br_n.png");
pub static OCEAN_BR_Y: &[u8] = include_bytes!("../../assets/imgs/ocean_br_y.png");

// Deep ocean cornering ocean
pub static DEEP_OCEAN_TL_N: &[u8] = include_bytes!("../../assets/imgs/deep_ocean_tl_n.png");
pub static DEEP_OCEAN_TL_Y: &[u8] = include_bytes!("../../assets/imgs/deep_ocean_tl_y.png");
pub static DEEP_OCEAN_TR_N: &[u8] = include_bytes!("../../assets/imgs/deep_ocean_tr_n.png");
pub static DEEP_OCEAN_TR_Y: &[u8] = include_bytes!("../../assets/imgs/deep_ocean_tr_y.png");
pub static DEEP_OCEAN_BL_N: &[u8] = include_bytes!("../../assets/imgs/deep_ocean_bl_n.png");
pub static DEEP_OCEAN_BL_Y: &[u8] = include_bytes!("../../assets/imgs/deep_ocean_bl_y.png");
pub static DEEP_OCEAN_BR_N: &[u8] = include_bytes!("../../assets/imgs/deep_ocean_br_n.png");
pub static DEEP_OCEAN_BR_Y: &[u8] = include_bytes!("../../assets/imgs/deep_ocean_br_y.png");

// Lake cornering ocean
pub static LAKE_TL_N: &[u8] = include_bytes!("../../assets/imgs/lake_tl_n.png");
pub static LAKE_TL_Y: &[u8] = include_bytes!("../../assets/imgs/lake_tl_y.png");
pub static LAKE_TR_N: &[u8] = include_bytes!("../../assets/imgs/lake_tr_n.png");
pub static LAKE_TR_Y: &[u8] = include_bytes!("../../assets/imgs/lake_tr_y.png");
pub static LAKE_BL_N: &[u8] = include_bytes!("../../assets/imgs/lake_bl_n.png");
pub static LAKE_BL_Y: &[u8] = include_bytes!("../../assets/imgs/lake_bl_y.png");
pub static LAKE_BR_N: &[u8] = include_bytes!("../../assets/imgs/lake_br_n.png");
pub static LAKE_BR_Y: &[u8] = include_bytes!("../../assets/imgs/lake_br_y.png");

// Inaccessible bordering ocean
pub static INACCESSIBLE_TL_N: &[u8] = include_bytes!("../../assets/imgs/inaccessible_tl_n.png");
pub static INACCESSIBLE_TL_Y: &[u8] = include_bytes!("../../assets/imgs/inaccessible_tl_y.png");
pub static INACCESSIBLE_TR_N: &[u8] = include_bytes!("../../assets/imgs/inaccessible_tr_n.png");
pub static INACCESSIBLE_TR_Y: &[u8] = include_bytes!("../../assets/imgs/inaccessible_tr_y.png");
pub static INACCESSIBLE_BL_N: &[u8] = include_bytes!("../../assets/imgs/inaccessible_bl_n.png");
pub static INACCESSIBLE_BL_Y: &[u8] = include_bytes!("../../assets/imgs/inaccessible_bl_y.png");
pub static INACCESSIBLE_BR_N: &[u8] = include_bytes!("../../assets/imgs/inaccessible_br_n.png");
pub static INACCESSIBLE_BR_Y: &[u8] = include_bytes!("../../assets/imgs/inaccessible_br_y.png");

static ALL_IMAGES: LazyLock<HashMap<&'static str, DynamicImage>> =
    LazyLock::new(|| load_all_images());

/// Gets an image by its name. The image must exist, otherwise this causes UB.
pub fn get_image(identifier: &'static str) -> &DynamicImage {
    unsafe { ALL_IMAGES.get(identifier).unwrap_unchecked() }
}

fn load_png(buf: &'static [u8]) -> DynamicImage {
    // SAFETY: The buffer is known to be a valid PNG.
    unsafe { image::load_from_memory_with_format(buf, ImageFormat::Png).unwrap_unchecked() }
}

fn load_all_images() -> HashMap<&'static str, DynamicImage> {
    HashMap::from([
        ("inaccessible", load_png(INACCESSIBLE)),
        ("fog", load_png(FOG)),
        ("grassland", load_png(GRASSLAND)),
        ("hills_not_ew", load_png(HILLS_NOT_EW)),
        ("hills_e", load_png(HILLS_E)),
        ("hills_ew", load_png(HILLS_EW)),
        ("hills_w", load_png(HILLS_W)),
        ("forest_not_ew", load_png(FOREST_NOT_EW)),
        ("forest_e", load_png(FOREST_E)),
        ("forest_ew", load_png(FOREST_EW)),
        ("forest_w", load_png(FOREST_W)),
        ("mountains_not_ew", load_png(MOUNTAINS_NOT_EW)),
        ("mountains_e", load_png(MOUNTAINS_E)),
        ("mountains_ew", load_png(MOUNTAINS_EW)),
        ("mountains_w", load_png(MOUNTAINS_W)),
        ("desert_nesw", load_png(DESERT_NESW)),
        ("desert_esw", load_png(DESERT_ESW)),
        ("desert_nsw", load_png(DESERT_NSW)),
        ("desert_sw", load_png(DESERT_SW)),
        ("desert_new", load_png(DESERT_NEW)),
        ("desert_ew", load_png(DESERT_EW)),
        ("desert_nw", load_png(DESERT_NW)),
        ("desert_w", load_png(DESERT_W)),
        ("desert_nes", load_png(DESERT_NES)),
        ("desert_es", load_png(DESERT_ES)),
        ("desert_ns", load_png(DESERT_NS)),
        ("desert_s", load_png(DESERT_S)),
        ("desert_ne", load_png(DESERT_NE)),
        ("desert_e", load_png(DESERT_E)),
        ("desert_n", load_png(DESERT_N)),
        ("desert_none", load_png(DESERT_NONE)),
        ("glacier_nesw", load_png(GLACIER_NESW)),
        ("glacier_esw", load_png(GLACIER_ESW)),
        ("glacier_nsw", load_png(GLACIER_NSW)),
        ("glacier_sw", load_png(GLACIER_SW)),
        ("glacier_new", load_png(GLACIER_NEW)),
        ("glacier_ew", load_png(GLACIER_EW)),
        ("glacier_nw", load_png(GLACIER_NW)),
        ("glacier_w", load_png(GLACIER_W)),
        ("glacier_nes", load_png(GLACIER_NES)),
        ("glacier_es", load_png(GLACIER_ES)),
        ("glacier_ns", load_png(GLACIER_NS)),
        ("glacier_s", load_png(GLACIER_S)),
        ("glacier_ne", load_png(GLACIER_NE)),
        ("glacier_e", load_png(GLACIER_E)),
        ("glacier_n", load_png(GLACIER_N)),
        ("glacier_none", load_png(GLACIER_NONE)),
        ("tundra_nesw", load_png(TUNDRA_NESW)),
        ("tundra_esw", load_png(TUNDRA_ESW)),
        ("tundra_nsw", load_png(TUNDRA_NSW)),
        ("tundra_sw", load_png(TUNDRA_SW)),
        ("tundra_new", load_png(TUNDRA_NEW)),
        ("tundra_ew", load_png(TUNDRA_EW)),
        ("tundra_nw", load_png(TUNDRA_NW)),
        ("tundra_w", load_png(TUNDRA_W)),
        ("tundra_nes", load_png(TUNDRA_NES)),
        ("tundra_es", load_png(TUNDRA_ES)),
        ("tundra_ns", load_png(TUNDRA_NS)),
        ("tundra_s", load_png(TUNDRA_S)),
        ("tundra_ne", load_png(TUNDRA_NE)),
        ("tundra_e", load_png(TUNDRA_E)),
        ("tundra_n", load_png(TUNDRA_N)),
        ("tundra_none", load_png(TUNDRA_NONE)),
        ("jungle_nesw", load_png(JUNGLE_NESW)),
        ("jungle_esw", load_png(JUNGLE_ESW)),
        ("jungle_nsw", load_png(JUNGLE_NSW)),
        ("jungle_sw", load_png(JUNGLE_SW)),
        ("jungle_new", load_png(JUNGLE_NEW)),
        ("jungle_ew", load_png(JUNGLE_EW)),
        ("jungle_nw", load_png(JUNGLE_NW)),
        ("jungle_w", load_png(JUNGLE_W)),
        ("jungle_nes", load_png(JUNGLE_NES)),
        ("jungle_es", load_png(JUNGLE_ES)),
        ("jungle_ns", load_png(JUNGLE_NS)),
        ("jungle_s", load_png(JUNGLE_S)),
        ("jungle_ne", load_png(JUNGLE_NE)),
        ("jungle_e", load_png(JUNGLE_E)),
        ("jungle_n", load_png(JUNGLE_N)),
        ("jungle_none", load_png(JUNGLE_NONE)),
        ("plains_nesw", load_png(PLAINS_NESW)),
        ("plains_esw", load_png(PLAINS_ESW)),
        ("plains_nsw", load_png(PLAINS_NSW)),
        ("plains_sw", load_png(PLAINS_SW)),
        ("plains_new", load_png(PLAINS_NEW)),
        ("plains_ew", load_png(PLAINS_EW)),
        ("plains_nw", load_png(PLAINS_NW)),
        ("plains_w", load_png(PLAINS_W)),
        ("plains_nes", load_png(PLAINS_NES)),
        ("plains_es", load_png(PLAINS_ES)),
        ("plains_ns", load_png(PLAINS_NS)),
        ("plains_s", load_png(PLAINS_S)),
        ("plains_ne", load_png(PLAINS_NE)),
        ("plains_e", load_png(PLAINS_E)),
        ("plains_n", load_png(PLAINS_N)),
        ("plains_none", load_png(PLAINS_NONE)),
        ("swamp_nesw", load_png(SWAMP_NESW)),
        ("swamp_esw", load_png(SWAMP_ESW)),
        ("swamp_nsw", load_png(SWAMP_NSW)),
        ("swamp_sw", load_png(SWAMP_SW)),
        ("swamp_new", load_png(SWAMP_NEW)),
        ("swamp_ew", load_png(SWAMP_EW)),
        ("swamp_nw", load_png(SWAMP_NW)),
        ("swamp_w", load_png(SWAMP_W)),
        ("swamp_nes", load_png(SWAMP_NES)),
        ("swamp_es", load_png(SWAMP_ES)),
        ("swamp_ns", load_png(SWAMP_NS)),
        ("swamp_s", load_png(SWAMP_S)),
        ("swamp_ne", load_png(SWAMP_NE)),
        ("swamp_e", load_png(SWAMP_E)),
        ("swamp_n", load_png(SWAMP_N)),
        ("swamp_none", load_png(SWAMP_NONE)),
        (
            "water_with_shoreline_nesw",
            load_png(WATER_WITH_SHORELINE_NESW),
        ),
        (
            "water_with_shoreline_esw",
            load_png(WATER_WITH_SHORELINE_ESW),
        ),
        (
            "water_with_shoreline_nsw",
            load_png(WATER_WITH_SHORELINE_NSW),
        ),
        ("water_with_shoreline_sw", load_png(WATER_WITH_SHORELINE_SW)),
        (
            "water_with_shoreline_new",
            load_png(WATER_WITH_SHORELINE_NEW),
        ),
        ("water_with_shoreline_ew", load_png(WATER_WITH_SHORELINE_EW)),
        ("water_with_shoreline_nw", load_png(WATER_WITH_SHORELINE_NW)),
        ("water_with_shoreline_w", load_png(WATER_WITH_SHORELINE_W)),
        (
            "water_with_shoreline_nes",
            load_png(WATER_WITH_SHORELINE_NES),
        ),
        ("water_with_shoreline_es", load_png(WATER_WITH_SHORELINE_ES)),
        ("water_with_shoreline_ns", load_png(WATER_WITH_SHORELINE_NS)),
        ("water_with_shoreline_s", load_png(WATER_WITH_SHORELINE_S)),
        ("water_with_shoreline_ne", load_png(WATER_WITH_SHORELINE_NE)),
        ("water_with_shoreline_e", load_png(WATER_WITH_SHORELINE_E)),
        ("water_with_shoreline_n", load_png(WATER_WITH_SHORELINE_N)),
        (
            "water_with_shoreline_none",
            load_png(WATER_WITH_SHORELINE_NONE),
        ),
        (
            "water_with_ice_shelves_nesw",
            load_png(WATER_WITH_ICE_SHELVES_NESW),
        ),
        (
            "water_with_ice_shelves_esw",
            load_png(WATER_WITH_ICE_SHELVES_ESW),
        ),
        (
            "water_with_ice_shelves_nsw",
            load_png(WATER_WITH_ICE_SHELVES_NSW),
        ),
        (
            "water_with_ice_shelves_sw",
            load_png(WATER_WITH_ICE_SHELVES_SW),
        ),
        (
            "water_with_ice_shelves_new",
            load_png(WATER_WITH_ICE_SHELVES_NEW),
        ),
        (
            "water_with_ice_shelves_ew",
            load_png(WATER_WITH_ICE_SHELVES_EW),
        ),
        (
            "water_with_ice_shelves_nw",
            load_png(WATER_WITH_ICE_SHELVES_NW),
        ),
        (
            "water_with_ice_shelves_w",
            load_png(WATER_WITH_ICE_SHELVES_W),
        ),
        (
            "water_with_ice_shelves_nes",
            load_png(WATER_WITH_ICE_SHELVES_NES),
        ),
        (
            "water_with_ice_shelves_es",
            load_png(WATER_WITH_ICE_SHELVES_ES),
        ),
        (
            "water_with_ice_shelves_ns",
            load_png(WATER_WITH_ICE_SHELVES_NS),
        ),
        (
            "water_with_ice_shelves_s",
            load_png(WATER_WITH_ICE_SHELVES_S),
        ),
        (
            "water_with_ice_shelves_ne",
            load_png(WATER_WITH_ICE_SHELVES_NE),
        ),
        (
            "water_with_ice_shelves_e",
            load_png(WATER_WITH_ICE_SHELVES_E),
        ),
        (
            "water_with_ice_shelves_n",
            load_png(WATER_WITH_ICE_SHELVES_N),
        ),
        (
            "water_with_ice_shelves_none",
            load_png(WATER_WITH_ICE_SHELVES_NONE),
        ),
        ("darkness", load_png(DARKNESS)),
        ("darkness_n", load_png(DARKNESS_N)),
        ("darkness_e", load_png(DARKNESS_E)),
        ("darkness_ne", load_png(DARKNESS_NE)),
        ("darkness_s", load_png(DARKNESS_S)),
        ("darkness_ns", load_png(DARKNESS_NS)),
        ("darkness_es", load_png(DARKNESS_ES)),
        ("darkness_nes", load_png(DARKNESS_NES)),
        ("darkness_w", load_png(DARKNESS_W)),
        ("darkness_nw", load_png(DARKNESS_NW)),
        ("darkness_ew", load_png(DARKNESS_EW)),
        ("darkness_new", load_png(DARKNESS_NEW)),
        ("darkness_sw", load_png(DARKNESS_SW)),
        ("darkness_nsw", load_png(DARKNESS_NSW)),
        ("darkness_esw", load_png(DARKNESS_ESW)),
        ("darkness_nesw", load_png(DARKNESS_NESW)),
        ("river", load_png(RIVER)),
        ("river_n", load_png(RIVER_N)),
        ("river_e", load_png(RIVER_E)),
        ("river_ne", load_png(RIVER_NE)),
        ("river_s", load_png(RIVER_S)),
        ("river_ns", load_png(RIVER_NS)),
        ("river_es", load_png(RIVER_ES)),
        ("river_nes", load_png(RIVER_NES)),
        ("river_w", load_png(RIVER_W)),
        ("river_nw", load_png(RIVER_NW)),
        ("river_ew", load_png(RIVER_EW)),
        ("river_new", load_png(RIVER_NEW)),
        ("river_sw", load_png(RIVER_SW)),
        ("river_nsw", load_png(RIVER_NSW)),
        ("river_esw", load_png(RIVER_ESW)),
        ("river_nesw", load_png(RIVER_NESW)),
        ("river_outlet_n", load_png(RIVER_OUTLET_N)),
        ("river_outlet_w", load_png(RIVER_OUTLET_W)),
        ("river_outlet_s", load_png(RIVER_OUTLET_S)),
        ("river_outlet_e", load_png(RIVER_OUTLET_E)),
        ("spice", load_png(SPICE)),
        ("furs", load_png(FURS)),
        ("peat", load_png(PEAT)),
        ("ivory", load_png(IVORY)),
        ("fruit", load_png(FRUIT)),
        ("iron", load_png(IRON)),
        ("whales", load_png(WHALES)),
        ("wheat", load_png(WHEAT)),
        ("pheasant", load_png(PHEASANT)),
        ("buffalo", load_png(BUFFALO)),
        ("silk", load_png(SILK)),
        ("wine", load_png(WINE)),
        ("seals", load_png(SEALS)),
        ("oasis", load_png(OASIS)),
        ("forest_game", load_png(FOREST_GAME)),
        ("grassland_resources", load_png(GRASSLAND_RESOURCES)),
        ("coal", load_png(COAL)),
        ("gems", load_png(GEMS)),
        ("gold", load_png(GOLD)),
        ("fish", load_png(FISH)),
        ("horses", load_png(HORSES)),
        ("river_resources", load_png(RIVER_RESOURCES)),
        ("oil", load_png(OIL)),
        ("tundra_game", load_png(TUNDRA_GAME)),
        ("aluminum", load_png(ALUMINUM)),
        ("uranium", load_png(URANIUM)),
        ("saltpeter", load_png(SALTPETER)),
        ("elephant", load_png(ELEPHANT)),
        ("farmland", load_png(FARMLAND)),
        ("irrigation", load_png(IRRIGATION)),
        ("mine", load_png(MINE)),
        ("oil_mine", load_png(OIL_MINE)),
        ("pollution", load_png(POLLUTION)),
        ("fallout", load_png(FALLOUT)),
        ("oil_rig", load_png(OIL_RIG)),
        ("buoy", load_png(BUOY)),
        ("ruins", load_png(RUINS)),
        ("village", load_png(VILLAGE)),
        ("airstrip", load_png(AIRSTRIP)),
        ("airbase", load_png(AIRBASE)),
        ("outpost", load_png(OUTPOST)),
        ("fortress", load_png(FORTRESS)),
        ("city_size_0", load_png(CITY_SIZE_0)),
        ("city_size_1", load_png(CITY_SIZE_1)),
        ("city_size_2", load_png(CITY_SIZE_2)),
        ("city_size_3", load_png(CITY_SIZE_3)),
        ("city_size_4", load_png(CITY_SIZE_4)),
        ("city_size_5", load_png(CITY_SIZE_5)),
        ("city_size_6", load_png(CITY_SIZE_6)),
        ("city_size_7", load_png(CITY_SIZE_7)),
        ("city_size_8", load_png(CITY_SIZE_8)),
        ("city_size_9", load_png(CITY_SIZE_9)),
        ("city_size_00", load_png(CITY_SIZE_00)),
        ("city_size_10", load_png(CITY_SIZE_10)),
        ("city_size_20", load_png(CITY_SIZE_20)),
        ("city_size_30", load_png(CITY_SIZE_30)),
        ("city_size_40", load_png(CITY_SIZE_40)),
        ("city_size_50", load_png(CITY_SIZE_50)),
        ("city_size_60", load_png(CITY_SIZE_60)),
        ("city_size_70", load_png(CITY_SIZE_70)),
        ("city_size_80", load_png(CITY_SIZE_80)),
        ("city_size_90", load_png(CITY_SIZE_90)),
        ("city_size_100", load_png(CITY_SIZE_100)),
        ("city_size_200", load_png(CITY_SIZE_200)),
        ("city_size_300", load_png(CITY_SIZE_300)),
        ("city_size_400", load_png(CITY_SIZE_400)),
        ("city_size_500", load_png(CITY_SIZE_500)),
        ("city_size_600", load_png(CITY_SIZE_600)),
        ("city_size_700", load_png(CITY_SIZE_700)),
        ("city_size_800", load_png(CITY_SIZE_800)),
        ("city_size_900", load_png(CITY_SIZE_900)),
        ("food_0", load_png(FOOD_0)),
        ("food_1", load_png(FOOD_1)),
        ("food_2", load_png(FOOD_2)),
        ("food_3", load_png(FOOD_3)),
        ("food_4", load_png(FOOD_4)),
        ("food_5", load_png(FOOD_5)),
        ("food_6", load_png(FOOD_6)),
        ("food_7", load_png(FOOD_7)),
        ("food_8", load_png(FOOD_8)),
        ("food_9", load_png(FOOD_9)),
        ("shields_0", load_png(SHIELDS_0)),
        ("shields_1", load_png(SHIELDS_1)),
        ("shields_2", load_png(SHIELDS_2)),
        ("shields_3", load_png(SHIELDS_3)),
        ("shields_4", load_png(SHIELDS_4)),
        ("shields_5", load_png(SHIELDS_5)),
        ("shields_6", load_png(SHIELDS_6)),
        ("shields_7", load_png(SHIELDS_7)),
        ("shields_8", load_png(SHIELDS_8)),
        ("shields_9", load_png(SHIELDS_9)),
        ("trade_0", load_png(TRADE_0)),
        ("trade_1", load_png(TRADE_1)),
        ("trade_2", load_png(TRADE_2)),
        ("trade_3", load_png(TRADE_3)),
        ("trade_4", load_png(TRADE_4)),
        ("trade_5", load_png(TRADE_5)),
        ("trade_6", load_png(TRADE_6)),
        ("trade_7", load_png(TRADE_7)),
        ("trade_8", load_png(TRADE_8)),
        ("trade_9", load_png(TRADE_9)),
        ("unit_tired", load_png(UNIT_TIRED)),
        ("unit_loaded", load_png(UNIT_LOADED)),
        ("unit_attention", load_png(UNIT_ATTENTION)),
        ("unit_stack", load_png(UNIT_STACK)),
        ("path_step", load_png(PATH_STEP)),
        ("path_step", load_png(PATH_EXHAUSTED)),
        ("path_step", load_png(PATH_NORMAL)),
        ("path_step", load_png(PATH_WAYPOINT)),
        ("unit_auto_attack", load_png(UNIT_AUTO_ATTACK)),
        ("unit_connect", load_png(UNIT_CONNECT)),
        ("unit_auto_explore", load_png(UNIT_AUTO_EXPLORE)),
        ("unit_fortifying", load_png(UNIT_FORTIFYING)),
        ("unit_fortified", load_png(UNIT_FORTIFIED)),
        ("unit_sentry", load_png(UNIT_SENTRY)),
        ("unit_patrol", load_png(UNIT_PATROL)),
        ("unit_mine", load_png(UNIT_MINE)),
        ("unit_irrigate", load_png(UNIT_IRRIGATE)),
        ("unit_transform", load_png(UNIT_TRANSFORM)),
        ("unit_pillage", load_png(UNIT_PILLAGE)),
        ("unit_pollution", load_png(UNIT_POLLUTION)),
        ("unit_fallout", load_png(UNIT_FALLOUT)),
        ("unit_convert", load_png(UNIT_CONVERT)),
        ("unit_goto", load_png(UNIT_GOTO)),
        ("unit_airstrip", load_png(UNIT_AIRSTRIP)),
        ("unit_outpost", load_png(UNIT_OUTPOST)),
        ("unit_airbase", load_png(UNIT_AIRBASE)),
        ("unit_fortress", load_png(UNIT_FORTRESS)),
        ("unit_buoy", load_png(UNIT_BUOY)),
        ("unit_road", load_png(UNIT_ROAD)),
        ("unit_rail", load_png(UNIT_RAIL)),
        ("unit_maglev", load_png(UNIT_MAGLEV)),
        ("unit_hp_100", load_png(UNIT_HP_100)),
        ("unit_hp_90", load_png(UNIT_HP_90)),
        ("unit_hp_80", load_png(UNIT_HP_80)),
        ("unit_hp_70", load_png(UNIT_HP_70)),
        ("unit_hp_60", load_png(UNIT_HP_60)),
        ("unit_hp_50", load_png(UNIT_HP_50)),
        ("unit_hp_40", load_png(UNIT_HP_40)),
        ("unit_hp_30", load_png(UNIT_HP_30)),
        ("unit_hp_20", load_png(UNIT_HP_20)),
        ("unit_hp_10", load_png(UNIT_HP_10)),
        ("unit_hp_0", load_png(UNIT_HP_0)),
        ("unit_vet_1", load_png(UNIT_VET_1)),
        ("unit_vet_2", load_png(UNIT_VET_2)),
        ("unit_vet_3", load_png(UNIT_VET_3)),
        ("unit_vet_4", load_png(UNIT_VET_4)),
        ("unit_vet_5", load_png(UNIT_VET_5)),
        ("unit_vet_6", load_png(UNIT_VET_6)),
        ("unit_vet_7", load_png(UNIT_VET_7)),
        ("unit_vet_8", load_png(UNIT_VET_8)),
        ("unit_vet_9", load_png(UNIT_VET_9)),
        ("upkeep_shield_1", load_png(UPKEEP_SHIELD_1)),
        ("upkeep_shield_2", load_png(UPKEEP_SHIELD_2)),
        ("upkeep_shield_3", load_png(UPKEEP_SHIELD_3)),
        ("upkeep_shield_4", load_png(UPKEEP_SHIELD_4)),
        ("upkeep_shield_5", load_png(UPKEEP_SHIELD_5)),
        ("upkeep_shield_6", load_png(UPKEEP_SHIELD_6)),
        ("upkeep_shield_7", load_png(UPKEEP_SHIELD_7)),
        ("upkeep_shield_8", load_png(UPKEEP_SHIELD_8)),
        ("upkeep_shield_9", load_png(UPKEEP_SHIELD_9)),
        ("upkeep_shield_10", load_png(UPKEEP_SHIELD_10)),
        ("upkeep_unhappy_1", load_png(UPKEEP_UNHAPPY_1)),
        ("upkeep_unhappy_2", load_png(UPKEEP_UNHAPPY_2)),
        ("upkeep_unhappy_3", load_png(UPKEEP_UNHAPPY_3)),
        ("upkeep_unhappy_4", load_png(UPKEEP_UNHAPPY_4)),
        ("upkeep_unhappy_5", load_png(UPKEEP_UNHAPPY_5)),
        ("upkeep_unhappy_6", load_png(UPKEEP_UNHAPPY_6)),
        ("upkeep_unhappy_7", load_png(UPKEEP_UNHAPPY_7)),
        ("upkeep_unhappy_8", load_png(UPKEEP_UNHAPPY_8)),
        ("upkeep_unhappy_9", load_png(UPKEEP_UNHAPPY_9)),
        ("upkeep_unhappy_10", load_png(UPKEEP_UNHAPPY_10)),
        ("upkeep_food_1", load_png(UPKEEP_FOOD_1)),
        ("upkeep_food_2", load_png(UPKEEP_FOOD_2)),
        ("upkeep_food_3", load_png(UPKEEP_FOOD_3)),
        ("upkeep_food_4", load_png(UPKEEP_FOOD_4)),
        ("upkeep_food_5", load_png(UPKEEP_FOOD_5)),
        ("upkeep_food_6", load_png(UPKEEP_FOOD_6)),
        ("upkeep_food_7", load_png(UPKEEP_FOOD_7)),
        ("upkeep_food_8", load_png(UPKEEP_FOOD_8)),
        ("upkeep_food_9", load_png(UPKEEP_FOOD_9)),
        ("upkeep_food_10", load_png(UPKEEP_FOOD_10)),
        ("upkeep_gold_1", load_png(UPKEEP_GOLD_1)),
        ("upkeep_gold_2", load_png(UPKEEP_GOLD_2)),
        ("upkeep_gold_3", load_png(UPKEEP_GOLD_3)),
        ("upkeep_gold_4", load_png(UPKEEP_GOLD_4)),
        ("upkeep_gold_5", load_png(UPKEEP_GOLD_5)),
        ("upkeep_gold_6", load_png(UPKEEP_GOLD_6)),
        ("upkeep_gold_7", load_png(UPKEEP_GOLD_7)),
        ("upkeep_gold_8", load_png(UPKEEP_GOLD_8)),
        ("upkeep_gold_9", load_png(UPKEEP_GOLD_9)),
        ("upkeep_gold_10", load_png(UPKEEP_GOLD_10)),
        ("nuke", load_png(NUKE)),
        ("ocean_tl_n", load_png(OCEAN_TL_N)),
        ("ocean_tl_y", load_png(OCEAN_TL_Y)),
        ("ocean_tr_n", load_png(OCEAN_TR_N)),
        ("ocean_tr_y", load_png(OCEAN_TR_Y)),
        ("ocean_bl_n", load_png(OCEAN_BL_N)),
        ("ocean_bl_y", load_png(OCEAN_BL_Y)),
        ("ocean_br_n", load_png(OCEAN_BR_N)),
        ("ocean_br_y", load_png(OCEAN_BR_Y)),
        ("deep_ocean_tl_n", load_png(DEEP_OCEAN_TL_N)),
        ("deep_ocean_tl_y", load_png(DEEP_OCEAN_TL_Y)),
        ("deep_ocean_tr_n", load_png(DEEP_OCEAN_TR_N)),
        ("deep_ocean_tr_y", load_png(DEEP_OCEAN_TR_Y)),
        ("deep_ocean_bl_n", load_png(DEEP_OCEAN_BL_N)),
        ("deep_ocean_bl_y", load_png(DEEP_OCEAN_BL_Y)),
        ("deep_ocean_br_n", load_png(DEEP_OCEAN_BR_N)),
        ("deep_ocean_br_y", load_png(DEEP_OCEAN_BR_Y)),
        ("lake_tl_n", load_png(LAKE_TL_N)),
        ("lake_tl_y", load_png(LAKE_TL_Y)),
        ("lake_tr_n", load_png(LAKE_TR_N)),
        ("lake_tr_y", load_png(LAKE_TR_Y)),
        ("lake_bl_n", load_png(LAKE_BL_N)),
        ("lake_bl_y", load_png(LAKE_BL_Y)),
        ("lake_br_n", load_png(LAKE_BR_N)),
        ("lake_br_y", load_png(LAKE_BR_Y)),
        ("inaccessible_tl_n", load_png(INACCESSIBLE_TL_N)),
        ("inaccessible_tl_y", load_png(INACCESSIBLE_TL_Y)),
        ("inaccessible_tr_n", load_png(INACCESSIBLE_TR_N)),
        ("inaccessible_tr_y", load_png(INACCESSIBLE_TR_Y)),
        ("inaccessible_bl_n", load_png(INACCESSIBLE_BL_N)),
        ("inaccessible_bl_y", load_png(INACCESSIBLE_BL_Y)),
        ("inaccessible_br_n", load_png(INACCESSIBLE_BR_N)),
        ("inaccessible_br_y", load_png(INACCESSIBLE_BR_Y)),
    ])
}
