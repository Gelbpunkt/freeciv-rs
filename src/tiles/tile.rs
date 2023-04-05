use std::hint::unreachable_unchecked;

use bitflags::bitflags;
use image::{imageops, DynamicImage, GenericImage, ImageFormat, Rgba};

use super::images;

pub const TILE_IMAGE_SIZE: u32 = 30;

fn load_png(buf: &'static [u8]) -> DynamicImage {
    // SAFETY: The buffer is known to be a valid PNG.
    unsafe { image::load_from_memory_with_format(buf, ImageFormat::Png).unwrap_unchecked() }
}

/// The FreeCiv map consists of tiles, which are laid out in a grid of squares.
/// Technically, FreeCiv supports other shapes, but we will simplify it to
/// squares.
///
/// Each tile has a terrain type, an optional special resource and might have
/// irrigation, mines, roads, railroads or other infrastructure.
///
/// Tiles can be transformed to other tiles via a [`Transform`].
///
/// TODO: Food, production and trade calculation.
#[derive(Debug, Clone)]
pub struct Tile {
    pub(crate) terrain: Terrain,
    pub(crate) special: Special,
    pub(crate) flags: Flags,
    pub(crate) transform_status: TransformStatus,
}

impl Tile {
    /// Create a new tile with the specified terrain, special resource and
    /// flags.
    #[must_use]
    pub const fn new(terrain: Terrain, special: Special, flags: Flags) -> Self {
        Self {
            terrain,
            special,
            flags,
            transform_status: TransformStatus::NotTransforming,
        }
    }

    /// The cost of moving from this tile (not on it).
    ///
    /// Units in FreeCiv have a specific movement, for example normal units
    /// usually have 1. This means they can go 1 tile each turn. A unit with 3
    /// movement may go 3 tiles far if the cost of movement on each is 1.
    #[must_use]
    pub const fn move_cost(&self) -> u8 {
        self.terrain.move_cost()
    }

    /// Attempts to start transforming this tile.
    ///
    /// Returns [`TransformResult::Impossible`] if the transformation was
    /// already done (for example if a tile is already irrigated and cannot be
    /// irrigated again), or if the transformation is not possible because of
    /// game rules, for example irrigating an ocean.
    ///
    /// TODO: Add support for farmland and railroad, check worker's skill
    /// level and researched technologies. Also, support multiple workers
    /// transforming at once and multiple transformations at once.
    pub fn start_transform(&mut self, transform: Transform) -> TransformResult {
        let turns = match self.terrain.transform(&transform) {
            TransformOutcome::BuildIrrigation(turns) => {
                if self.flags.contains(Flags::HAS_IRRIGATION) {
                    return TransformResult::Impossible;
                }

                turns
            }
            TransformOutcome::BuildMine(turns) => {
                if self.flags.contains(Flags::HAS_MINE) {
                    return TransformResult::Impossible;
                }

                turns
            }
            TransformOutcome::BuildRoad(turns) => {
                if self.flags.contains(Flags::HAS_ROAD) {
                    return TransformResult::Impossible;
                }

                turns
            }
            TransformOutcome::Impossible => return TransformResult::Impossible,
            TransformOutcome::TransformTo(_, turns) => turns,
        };

        self.transform_status = TransformStatus::Transforming {
            turns_remaining: turns,
            transform,
        };

        TransformResult::Possible { turns }
    }

    /// Changes the terrain of this tile and changes special resources and flags
    /// according to game rules.
    fn change_terrain(&mut self, terrain: Terrain) {
        self.terrain = terrain;
        // Special resources always disappear when terraforming.
        self.special = Special::None;

        // On ocean, all flags are removed.
        if terrain == Terrain::Ocean {
            self.flags = Flags::empty();
        } else {
            // Irrigation has to be removed if it cannot be built on the new terrain
            if !matches!(
                self.terrain.transform(&Transform::Irrigation),
                TransformOutcome::BuildIrrigation(_)
            ) {
                self.flags.remove(Flags::HAS_IRRIGATION);
            }

            // Mine has to be removed if it cannot be built on the new terrain
            if !matches!(
                self.terrain.transform(&Transform::Mining),
                TransformOutcome::BuildMine(_)
            ) {
                self.flags.remove(Flags::HAS_MINE);
            }

            // Road has to be removed if it cannot be built on the new terrain
            if !matches!(
                self.terrain.transform(&Transform::Road),
                TransformOutcome::BuildRoad(_)
            ) {
                self.flags.remove(Flags::HAS_ROAD);
            }

            // TODO: Probably missing some behaviour
        }
    }

    /// Tick the ongoing transforms on this tile by one turn. If the required
    /// amount of turns for the ongoing transformation is reached, the tile is
    /// transformed.
    pub fn tick_transform(&mut self) {
        let transform = match &mut self.transform_status {
            TransformStatus::NotTransforming => return,
            TransformStatus::Transforming {
                transform,
                turns_remaining,
            } => {
                *turns_remaining -= 1;

                if *turns_remaining > 0 {
                    return;
                }

                transform
            }
        };

        match self.terrain.transform(transform) {
            TransformOutcome::BuildIrrigation(_) => {
                self.flags |= Flags::HAS_IRRIGATION;
            }
            TransformOutcome::BuildMine(_) => {
                self.flags |= Flags::HAS_MINE;
            }
            TransformOutcome::BuildRoad(_) => {
                self.flags |= Flags::HAS_ROAD;
            }
            TransformOutcome::Impossible => unreachable!(),
            TransformOutcome::TransformTo(terrain, _) => {
                self.change_terrain(terrain);
            }
        }

        self.transform_status = TransformStatus::NotTransforming;
    }

    /// Ticks the tile via [`Tile::tick_transform`] until the transformation is
    /// done.
    ///
    /// This is intended for debugging purposes.
    pub fn tick_until_transform_done(&mut self) {
        while self.transform_status != TransformStatus::NotTransforming {
            self.tick_transform();
        }
    }

    pub fn render<G: GenericImage<Pixel = Rgba<u8>>>(
        &self,
        base: &mut G,
        north: Option<&Self>,
        north_east: Option<&Self>,
        east: Option<&Self>,
        south_east: Option<&Self>,
        south: Option<&Self>,
        south_west: Option<&Self>,
        west: Option<&Self>,
        north_west: Option<&Self>,
    ) {
        // TODO: Some flags render above specials, some below (e.g. hut vs irrigation)
        self.terrain.render(
            base,
            north.map(|t| t.terrain),
            north_east.map(|t| t.terrain),
            east.map(|t| t.terrain),
            south_east.map(|t| t.terrain),
            south.map(|t| t.terrain),
            south_west.map(|t| t.terrain),
            west.map(|t| t.terrain),
            north_west.map(|t| t.terrain),
        );
        self.flags.render(
            base,
            north.map(|t| t.flags),
            east.map(|t| t.flags),
            south.map(|t| t.flags),
            west.map(|t| t.flags),
        );
        self.special.render(base);
    }
}

/// The terrain of a [`Tile`]. Refer to the wiki for more information:
/// <https://freeciv.fandom.com/wiki/Terrain>.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Terrain {
    DeepOcean,
    Desert,
    Forest,
    Glacier,
    Grassland,
    Hills,
    Jungle,
    Lake,
    Mountains,
    Ocean,
    Plains,
    Swamp,
    Tundra,
}

impl Terrain {
    pub const fn is_water(&self) -> bool {
        matches!(self, Self::DeepOcean | Self::Ocean | Self::Lake)
    }

    pub const fn can_build_irrigation(&self) -> bool {
        matches!(
            self.transform(&Transform::Irrigation),
            TransformOutcome::BuildIrrigation(_)
        )
    }

    pub const fn can_build_mine(&self) -> bool {
        matches!(
            self.transform(&Transform::Mining),
            TransformOutcome::BuildMine(_)
        )
    }

    pub const fn can_build_road(&self) -> bool {
        matches!(
            self.transform(&Transform::Road),
            TransformOutcome::BuildRoad(_)
        )
    }

    /// Movement cost on this terrain. See [`Tile::move_cost`].
    #[must_use]
    pub const fn move_cost(self) -> u8 {
        match self {
            Self::DeepOcean
            | Self::Desert
            | Self::Grassland
            | Self::Lake
            | Self::Ocean
            | Self::Plains
            | Self::Tundra => 1,
            Self::Forest | Self::Glacier | Self::Hills | Self::Jungle | Self::Swamp => 2,
            Self::Mountains => 3,
        }
    }

    pub const fn allowed_specials(&self) -> &'static [Special] {
        match self {
            Self::DeepOcean => &[],
            Self::Desert => &[Special::Oasis, Special::Oil],
            Self::Forest => &[Special::Pheasant, Special::Silk],
            Self::Glacier => &[Special::Ivory, Special::Oil],
            Self::Grassland => &[Special::Resources],
            Self::Hills => &[Special::Coal, Special::Wine],
            Self::Jungle => &[Special::Gems, Special::Fruit],
            Self::Lake => &[Special::Fish],
            Self::Mountains => &[Special::Gold, Special::Iron],
            Self::Ocean => &[Special::Fish, Special::Whales],
            Self::Plains => &[Special::Buffalo, Special::Wheat],
            Self::Swamp => &[Special::Peat, Special::Spice],
            Self::Tundra => &[Special::Game, Special::Furs],
        }
    }

    /// The result of transforming this terrain with a [`Transform`]. The
    /// outcome returns whether this is a possible transformation, what the new
    /// terrain type or flags will be and how many turns it takes.
    const fn transform(self, transform: &Transform) -> TransformOutcome {
        match self {
            Self::DeepOcean => TransformOutcome::Impossible,
            Self::Desert => match transform {
                Transform::Irrigation => irrigation(5),
                Transform::Mining => mine(5),
                Transform::Road => road(2),
                Transform::Transforming => plains(24),
            },
            Self::Forest => match transform {
                Transform::Irrigation => plains(5),
                Transform::Mining => swamp(15),
                Transform::Road => road(4),
                Transform::Transforming => grassland(24),
            },
            Self::Glacier => match transform {
                Transform::Irrigation => impossible(),
                Transform::Mining => mine(10),
                Transform::Road => road(4),
                Transform::Transforming => tundra(24),
            },
            Self::Grassland => match transform {
                Transform::Irrigation => irrigation(5),
                Transform::Mining => forest(10),
                Transform::Road => road(2),
                Transform::Transforming => hills(24),
            },
            Self::Hills => match transform {
                Transform::Irrigation => irrigation(10),
                Transform::Mining => mine(10),
                Transform::Road => road(4),
                Transform::Transforming => plains(24),
            },
            Self::Jungle => match transform {
                Transform::Irrigation => grassland(15),
                Transform::Mining => forest(15),
                Transform::Road => road(4),
                Transform::Transforming => plains(24),
            },
            Self::Lake => match transform {
                Transform::Irrigation | Transform::Mining | Transform::Road => impossible(),
                Transform::Transforming => swamp(36),
            },
            Self::Mountains => match transform {
                Transform::Irrigation => impossible(),
                Transform::Mining => mine(10),
                Transform::Road => road(6),
                Transform::Transforming => hills(24),
            },
            Self::Ocean => match transform {
                Transform::Irrigation | Transform::Mining | Transform::Road => impossible(),
                Transform::Transforming => swamp(36),
            },
            Self::Plains => match transform {
                Transform::Irrigation => irrigation(5),
                Transform::Mining => forest(15),
                Transform::Road => road(2),
                Transform::Transforming => grassland(24),
            },
            Self::Swamp => match transform {
                Transform::Irrigation => grassland(15),
                Transform::Mining => forest(15),
                Transform::Road => road(4),
                Transform::Transforming => ocean(36),
            },
            Self::Tundra => match transform {
                Transform::Irrigation => irrigation(5),
                Transform::Mining => impossible(),
                Transform::Road => road(2),
                Transform::Transforming => desert(24),
            },
        }
    }

    fn draw_coastline<G: GenericImage<Pixel = Rgba<u8>>>(
        self,
        base: &mut G,
        north: Option<Self>,
        east: Option<Self>,
        south: Option<Self>,
        west: Option<Self>,
    ) {
        let north_water = north.map_or(false, |t| t.is_water());
        let east_water = east.map_or(false, |t| t.is_water());
        let south_water = south.map_or(false, |t| t.is_water());
        let west_water = west.map_or(false, |t| t.is_water());

        // The tileset doesn't support water bordering glacier AND land at the same
        // time.
        let any_is_glacier = north.map_or(false, |t| t == Terrain::Glacier)
            | east.map_or(false, |t| t == Terrain::Glacier)
            | south.map_or(false, |t| t == Terrain::Glacier)
            | west.map_or(false, |t| t == Terrain::Glacier);

        let buf = if any_is_glacier {
            match (north_water, east_water, south_water, west_water) {
                (true, true, true, true) => images::WATER_WITH_ICE_SHELVES_NESW,
                (true, true, true, false) => images::WATER_WITH_ICE_SHELVES_NES,
                (true, true, false, true) => images::WATER_WITH_ICE_SHELVES_NEW,
                (true, false, true, true) => images::WATER_WITH_ICE_SHELVES_NSW,
                (false, true, true, true) => images::WATER_WITH_ICE_SHELVES_ESW,
                (true, true, false, false) => images::WATER_WITH_ICE_SHELVES_NE,
                (true, false, true, false) => images::WATER_WITH_ICE_SHELVES_NS,
                (true, false, false, true) => images::WATER_WITH_ICE_SHELVES_NW,
                (false, true, true, false) => images::WATER_WITH_ICE_SHELVES_ES,
                (false, true, false, true) => images::WATER_WITH_ICE_SHELVES_EW,
                (false, false, true, true) => images::WATER_WITH_ICE_SHELVES_SW,
                (true, false, false, false) => images::WATER_WITH_ICE_SHELVES_N,
                (false, true, false, false) => images::WATER_WITH_ICE_SHELVES_E,
                (false, false, true, false) => images::WATER_WITH_ICE_SHELVES_S,
                (false, false, false, true) => images::WATER_WITH_ICE_SHELVES_W,
                (false, false, false, false) => images::WATER_WITH_ICE_SHELVES_NONE,
            }
        } else {
            match (north_water, east_water, south_water, west_water) {
                (true, true, true, true) => images::WATER_WITH_SHORELINE_NESW,
                (true, true, true, false) => images::WATER_WITH_SHORELINE_NES,
                (true, true, false, true) => images::WATER_WITH_SHORELINE_NEW,
                (true, false, true, true) => images::WATER_WITH_SHORELINE_NSW,
                (false, true, true, true) => images::WATER_WITH_SHORELINE_ESW,
                (true, true, false, false) => images::WATER_WITH_SHORELINE_NE,
                (true, false, true, false) => images::WATER_WITH_SHORELINE_NS,
                (true, false, false, true) => images::WATER_WITH_SHORELINE_NW,
                (false, true, true, false) => images::WATER_WITH_SHORELINE_ES,
                (false, true, false, true) => images::WATER_WITH_SHORELINE_EW,
                (false, false, true, true) => images::WATER_WITH_SHORELINE_SW,
                (true, false, false, false) => images::WATER_WITH_SHORELINE_N,
                (false, true, false, false) => images::WATER_WITH_SHORELINE_E,
                (false, false, true, false) => images::WATER_WITH_SHORELINE_S,
                (false, false, false, true) => images::WATER_WITH_SHORELINE_W,
                (false, false, false, false) => images::WATER_WITH_SHORELINE_NONE,
            }
        };

        let img = load_png(buf);

        imageops::overlay(base, &img, 0, 0);
    }

    fn render<G: GenericImage<Pixel = Rgba<u8>>>(
        self,
        base: &mut G,
        north: Option<Self>,
        north_east: Option<Self>,
        east: Option<Self>,
        south_east: Option<Self>,
        south: Option<Self>,
        south_west: Option<Self>,
        west: Option<Self>,
        north_west: Option<Self>,
    ) {
        // TODO: Refactor
        let north_same = north.is_some() && north == Some(self);
        let east_same = east.is_some() && east == Some(self);
        let south_same = south.is_some() && south == Some(self);
        let west_same = west.is_some() && west == Some(self);

        let buf: &'static [u8] = match self {
            Self::DeepOcean => {
                // TODO: Figure out how the hell this works
                let tl = load_png(images::DEEP_OCEAN_TL_N);
                let tr = load_png(images::DEEP_OCEAN_TR_N);
                let bl = load_png(images::DEEP_OCEAN_BL_N);
                let br = load_png(images::DEEP_OCEAN_BR_N);

                imageops::overlay(base, &tl, 0, 0);
                imageops::overlay(base, &tr, 15, 0);
                imageops::overlay(base, &bl, 0, 15);
                imageops::overlay(base, &br, 15, 15);

                self.draw_coastline(base, north, east, south, west);

                return;
            }
            Self::Desert => match (north_same, east_same, south_same, west_same) {
                (true, true, true, true) => images::DESERT_NESW,
                (true, true, true, false) => images::DESERT_NES,
                (true, true, false, true) => images::DESERT_NEW,
                (true, false, true, true) => images::DESERT_NSW,
                (false, true, true, true) => images::DESERT_ESW,
                (true, true, false, false) => images::DESERT_NE,
                (true, false, true, false) => images::DESERT_NS,
                (true, false, false, true) => images::DESERT_NW,
                (false, true, true, false) => images::DESERT_ES,
                (false, true, false, true) => images::DESERT_EW,
                (false, false, true, true) => images::DESERT_SW,
                (true, false, false, false) => images::DESERT_N,
                (false, true, false, false) => images::DESERT_E,
                (false, false, true, false) => images::DESERT_S,
                (false, false, false, true) => images::DESERT_W,
                (false, false, false, false) => images::DESERT_NONE,
            },
            Self::Forest => match (east_same, west_same) {
                (true, true) => images::FOREST_EW,
                (true, false) => images::FOREST_E,
                (false, true) => images::FOREST_W,
                (false, false) => images::FOREST_NOT_EW,
            },
            Self::Glacier => match (north_same, east_same, south_same, west_same) {
                (true, true, true, true) => images::GLACIER_NESW,
                (true, true, true, false) => images::GLACIER_NES,
                (true, true, false, true) => images::GLACIER_NEW,
                (true, false, true, true) => images::GLACIER_NSW,
                (false, true, true, true) => images::GLACIER_ESW,
                (true, true, false, false) => images::GLACIER_NE,
                (true, false, true, false) => images::GLACIER_NS,
                (true, false, false, true) => images::GLACIER_NW,
                (false, true, true, false) => images::GLACIER_ES,
                (false, true, false, true) => images::GLACIER_EW,
                (false, false, true, true) => images::GLACIER_SW,
                (true, false, false, false) => images::GLACIER_N,
                (false, true, false, false) => images::GLACIER_E,
                (false, false, true, false) => images::GLACIER_S,
                (false, false, false, true) => images::GLACIER_W,
                (false, false, false, false) => images::GLACIER_NONE,
            },
            Self::Grassland => images::GRASSLAND,
            Self::Hills => match (east_same, west_same) {
                (true, true) => images::HILLS_EW,
                (true, false) => images::HILLS_E,
                (false, true) => images::HILLS_W,
                (false, false) => images::HILLS_NOT_EW,
            },
            Self::Jungle => match (north_same, east_same, south_same, west_same) {
                (true, true, true, true) => images::JUNGLE_NESW,
                (true, true, true, false) => images::JUNGLE_NES,
                (true, true, false, true) => images::JUNGLE_NEW,
                (true, false, true, true) => images::JUNGLE_NSW,
                (false, true, true, true) => images::JUNGLE_ESW,
                (true, true, false, false) => images::JUNGLE_NE,
                (true, false, true, false) => images::JUNGLE_NS,
                (true, false, false, true) => images::JUNGLE_NW,
                (false, true, true, false) => images::JUNGLE_ES,
                (false, true, false, true) => images::JUNGLE_EW,
                (false, false, true, true) => images::JUNGLE_SW,
                (true, false, false, false) => images::JUNGLE_N,
                (false, true, false, false) => images::JUNGLE_E,
                (false, false, true, false) => images::JUNGLE_S,
                (false, false, false, true) => images::JUNGLE_W,
                (false, false, false, false) => images::JUNGLE_NONE,
            },
            Self::Lake => {
                let tl = load_png(images::LAKE_TL_N);
                let tr = load_png(images::LAKE_TR_N);
                let bl = load_png(images::LAKE_BL_N);
                let br = load_png(images::LAKE_BR_N);

                imageops::overlay(base, &tl, 0, 0);
                imageops::overlay(base, &tr, 15, 0);
                imageops::overlay(base, &bl, 0, 15);
                imageops::overlay(base, &br, 15, 15);

                self.draw_coastline(base, north, east, south, west);

                return;
            }
            Self::Mountains => match (east_same, west_same) {
                (true, true) => images::MOUNTAINS_EW,
                (true, false) => images::MOUNTAINS_E,
                (false, true) => images::MOUNTAINS_W,
                (false, false) => images::MOUNTAINS_NOT_EW,
            },
            Self::Ocean => {
                let tl = load_png(images::OCEAN_TL_N);
                let tr = load_png(images::OCEAN_TR_N);
                let bl = load_png(images::OCEAN_BL_N);
                let br = load_png(images::OCEAN_BR_N);

                imageops::overlay(base, &tl, 0, 0);
                imageops::overlay(base, &tr, 15, 0);
                imageops::overlay(base, &bl, 0, 15);
                imageops::overlay(base, &br, 15, 15);

                self.draw_coastline(base, north, east, south, west);

                return;
            }
            Self::Plains => match (north_same, east_same, south_same, west_same) {
                (true, true, true, true) => images::PLAINS_NESW,
                (true, true, true, false) => images::PLAINS_NES,
                (true, true, false, true) => images::PLAINS_NEW,
                (true, false, true, true) => images::PLAINS_NSW,
                (false, true, true, true) => images::PLAINS_ESW,
                (true, true, false, false) => images::PLAINS_NE,
                (true, false, true, false) => images::PLAINS_NS,
                (true, false, false, true) => images::PLAINS_NW,
                (false, true, true, false) => images::PLAINS_ES,
                (false, true, false, true) => images::PLAINS_EW,
                (false, false, true, true) => images::PLAINS_SW,
                (true, false, false, false) => images::PLAINS_N,
                (false, true, false, false) => images::PLAINS_E,
                (false, false, true, false) => images::PLAINS_S,
                (false, false, false, true) => images::PLAINS_W,
                (false, false, false, false) => images::PLAINS_NONE,
            },
            Self::Swamp => match (north_same, east_same, south_same, west_same) {
                (true, true, true, true) => images::SWAMP_NESW,
                (true, true, true, false) => images::SWAMP_NES,
                (true, true, false, true) => images::SWAMP_NEW,
                (true, false, true, true) => images::SWAMP_NSW,
                (false, true, true, true) => images::SWAMP_ESW,
                (true, true, false, false) => images::SWAMP_NE,
                (true, false, true, false) => images::SWAMP_NS,
                (true, false, false, true) => images::SWAMP_NW,
                (false, true, true, false) => images::SWAMP_ES,
                (false, true, false, true) => images::SWAMP_EW,
                (false, false, true, true) => images::SWAMP_SW,
                (true, false, false, false) => images::SWAMP_N,
                (false, true, false, false) => images::SWAMP_E,
                (false, false, true, false) => images::SWAMP_S,
                (false, false, false, true) => images::SWAMP_W,
                (false, false, false, false) => images::SWAMP_NONE,
            },
            Self::Tundra => match (north_same, east_same, south_same, west_same) {
                (true, true, true, true) => images::TUNDRA_NESW,
                (true, true, true, false) => images::TUNDRA_NES,
                (true, true, false, true) => images::TUNDRA_NEW,
                (true, false, true, true) => images::TUNDRA_NSW,
                (false, true, true, true) => images::TUNDRA_ESW,
                (true, true, false, false) => images::TUNDRA_NE,
                (true, false, true, false) => images::TUNDRA_NS,
                (true, false, false, true) => images::TUNDRA_NW,
                (false, true, true, false) => images::TUNDRA_ES,
                (false, true, false, true) => images::TUNDRA_EW,
                (false, false, true, true) => images::TUNDRA_SW,
                (true, false, false, false) => images::TUNDRA_N,
                (false, true, false, false) => images::TUNDRA_E,
                (false, false, true, false) => images::TUNDRA_S,
                (false, false, false, true) => images::TUNDRA_W,
                (false, false, false, false) => images::TUNDRA_NONE,
            },
        };

        let img = load_png(buf);

        imageops::overlay(base, &img, 0, 0);
    }

    pub(crate) fn random() -> Self {
        match fastrand::u8(..13) {
            0 => Self::DeepOcean,
            1 => Self::Desert,
            2 => Self::Forest,
            3 => Self::Glacier,
            4 => Self::Grassland,
            5 => Self::Hills,
            6 => Self::Jungle,
            7 => Self::Lake,
            8 => Self::Mountains,
            9 => Self::Ocean,
            10 => Self::Plains,
            11 => Self::Swamp,
            12 => Self::Tundra,
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

#[inline]
const fn irrigation(turns: u8) -> TransformOutcome {
    TransformOutcome::BuildIrrigation(turns)
}

#[inline]
const fn mine(turns: u8) -> TransformOutcome {
    TransformOutcome::BuildMine(turns)
}

#[inline]
const fn road(turns: u8) -> TransformOutcome {
    TransformOutcome::BuildRoad(turns)
}

#[inline]
const fn railroad() -> TransformOutcome {
    road(4)
}

#[inline]
const fn desert(turns: u8) -> TransformOutcome {
    TransformOutcome::TransformTo(Terrain::Desert, turns)
}

#[inline]
const fn forest(turns: u8) -> TransformOutcome {
    TransformOutcome::TransformTo(Terrain::Forest, turns)
}

#[inline]
const fn grassland(turns: u8) -> TransformOutcome {
    TransformOutcome::TransformTo(Terrain::Grassland, turns)
}

#[inline]
const fn hills(turns: u8) -> TransformOutcome {
    TransformOutcome::TransformTo(Terrain::Hills, turns)
}

#[inline]
const fn ocean(turns: u8) -> TransformOutcome {
    TransformOutcome::TransformTo(Terrain::Ocean, turns)
}

#[inline]
const fn plains(turns: u8) -> TransformOutcome {
    TransformOutcome::TransformTo(Terrain::Plains, turns)
}

#[inline]
const fn swamp(turns: u8) -> TransformOutcome {
    TransformOutcome::TransformTo(Terrain::Swamp, turns)
}

#[inline]
const fn tundra(turns: u8) -> TransformOutcome {
    TransformOutcome::TransformTo(Terrain::Tundra, turns)
}

#[inline]
const fn impossible() -> TransformOutcome {
    TransformOutcome::Impossible
}

/// The outcome of a transform from a [`Terrain`] with a [`Transform`].
#[derive(PartialEq, Eq)]
enum TransformOutcome {
    BuildIrrigation(u8),
    BuildMine(u8),
    BuildRoad(u8),
    TransformTo(Terrain, u8),
    Impossible,
}

/// Special resource that may be present on a [`Tile`]. This grants additional
/// food, production or trade points.
#[derive(Debug, Clone, Copy)]
pub enum Special {
    None,
    Oasis,
    Oil,
    Pheasant,
    Silk,
    Ivory,
    Resources,
    Coal,
    Wine,
    Gems,
    Fruit,
    Fish,
    Gold,
    Iron,
    Whales,
    Buffalo,
    Wheat,
    Peat,
    Spice,
    Game,
    Furs,
}

impl Special {
    fn render<G: GenericImage<Pixel = Rgba<u8>>>(&self, base: &mut G) {
        let buf = match self {
            Self::None => return,
            Self::Oasis => images::OASIS,
            Self::Oil => images::OIL,
            Self::Pheasant => images::PHEASANT,
            Self::Silk => images::SILK,
            Self::Ivory => images::IVORY,
            Self::Resources => images::GRASSLAND_RESOURCES, // TODO: Dynamic on river
            Self::Coal => images::COAL,
            Self::Wine => images::WINE,
            Self::Gems => images::GEMS,
            Self::Fruit => images::FRUIT,
            Self::Fish => images::FISH,
            Self::Gold => images::GOLD,
            Self::Iron => images::IRON,
            Self::Whales => images::WHALES,
            Self::Buffalo => images::BUFFALO,
            Self::Wheat => images::WHEAT,
            Self::Peat => images::PEAT,
            Self::Spice => images::SPICE,
            Self::Game => images::TUNDRA_GAME, // TODO? Does our version support game on forest?
            Self::Furs => images::FURS,
        };

        let img = load_png(buf);

        imageops::overlay(base, &img, 0, 0);
    }
}

bitflags! {
    /// Flags for possible modifications to a [`Tile`] that include player-made
    /// things such as roads, irrigation or mines as well as game-made modifications
    /// like rivers or pollution.
    #[derive(Clone, Copy, Debug)]
    pub struct Flags: u16 {
        const HAS_RIVER =           0b0000_0000_0001;
        const HAS_ROAD  =           0b0000_0000_0010;
        const HAS_IRRIGATION =      0b0000_0000_0100;
        const HAS_MINE =            0b0000_0000_1000;
        const HAS_RAILROAD =        0b0000_0001_0000;
        const HAS_RUINS =           0b0000_0010_0000;
        const HAS_POLLUTION =       0b0000_0100_0000;
        const HAS_FORT =            0b0000_1000_0000;
        const HAS_NUCLEAR_FALLOUT = 0b0001_0000_0000;
        const HAS_HUT =             0b0010_0000_0000;
        const HAS_FARMLAND =        0b0100_0000_0000;
        // TODO: Consider removing this and storing it differently.
        const HAS_CITY =            0b1000_0000_0000;
    }
}

impl Flags {
    fn render<G: GenericImage<Pixel = Rgba<u8>>>(
        &self,
        base: &mut G,
        north: Option<Self>,
        east: Option<Self>,
        south: Option<Self>,
        west: Option<Self>,
    ) {
        if self.contains(Self::HAS_RIVER) {
            let river_north = north.map_or(false, |f| f.contains(Flags::HAS_RIVER));
            let river_east = east.map_or(false, |f| f.contains(Flags::HAS_RIVER));
            let river_south = south.map_or(false, |f| f.contains(Flags::HAS_RIVER));
            let river_west = west.map_or(false, |f| f.contains(Flags::HAS_RIVER));

            let buf = match (river_north, river_east, river_south, river_west) {
                (true, true, true, true) => images::RIVER_NESW,
                (true, true, true, false) => images::RIVER_NES,
                (true, true, false, true) => images::RIVER_NEW,
                (true, false, true, true) => images::RIVER_NSW,
                (false, true, true, true) => images::RIVER_ESW,
                (true, true, false, false) => images::RIVER_NE,
                (true, false, true, false) => images::RIVER_NS,
                (true, false, false, true) => images::RIVER_NW,
                (false, true, true, false) => images::RIVER_ES,
                (false, true, false, true) => images::RIVER_EW,
                (false, false, true, true) => images::RIVER_SW,
                (true, false, false, false) => images::RIVER_N,
                (false, true, false, false) => images::RIVER_E,
                (false, false, true, false) => images::RIVER_S,
                (false, false, false, true) => images::RIVER_W,
                (false, false, false, false) => images::RIVER,
            };

            let img = load_png(buf);
            imageops::overlay(base, &img, 0, 0);
        }

        if self.contains(Self::HAS_ROAD) {
            // TODO: When we have ported the road files
            todo!()
        }

        if self.contains(Self::HAS_IRRIGATION) {
            let img = load_png(images::IRRIGATION);
            imageops::overlay(base, &img, 0, 0);
        }

        if self.contains(Self::HAS_MINE) {
            let img = load_png(images::MINE);
            imageops::overlay(base, &img, 0, 0);
        }

        if self.contains(Self::HAS_RAILROAD) {
            // TODO: Seems to be missing in the tileset
            todo!()
        }

        if self.contains(Self::HAS_RUINS) {
            let img = load_png(images::RUINS);
            imageops::overlay(base, &img, 0, 0);
        }

        if self.contains(Self::HAS_POLLUTION) {
            let img = load_png(images::POLLUTION);
            imageops::overlay(base, &img, 0, 0);
        }

        if self.contains(Self::HAS_FORT) {
            // TODO: We seem to have the images, but idk which ones are which
            todo!()
        }

        if self.contains(Self::HAS_NUCLEAR_FALLOUT) {
            // TODO: We might have the images, but idk which ones are which
        }

        if self.contains(Self::HAS_HUT) {
            let img = load_png(images::VILLAGE);
            imageops::overlay(base, &img, 0, 0);
        }

        if self.contains(Self::HAS_FARMLAND) {
            let img = load_png(images::FARMLAND);
            imageops::overlay(base, &img, 0, 0);
        }

        // TODO: City definitely shouldn't be a flag
    }
}

/// A possibly ongoing transformation on a [`Tile`].
#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum TransformStatus {
    Transforming {
        transform: Transform,
        turns_remaining: u8,
    },
    NotTransforming,
}

/// The result of starting a transform via [`Tile::start_transform`].
pub enum TransformResult {
    Possible { turns: u8 },
    Impossible,
}

/// There are four possible transforms in FreeCiv: Irrigation (I), mining (M),
/// road (R) and transforming (O).
///
/// TODO: Clean pollution is another transform.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Transform {
    Irrigation,
    Mining,
    Road,
    Transforming,
}

/// Skill level of a worker unit. Workers have basic skill, engineers have
/// advanced skill. Units with advanced skill can perform some transforms that
/// basic skilled units cannot.
pub enum WorkerSkill {
    Basic,
    Advanced,
}
