use std::hint::unreachable_unchecked;

use bitflags::bitflags;
use image::{imageops, GenericImage, Rgba};

use super::images::get_image;

pub const TILE_IMAGE_SIZE: u32 = 30;

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

        let img = if any_is_glacier {
            match (north_water, east_water, south_water, west_water) {
                (true, true, true, true) => get_image("water_with_ice_shelves_nesw"),
                (true, true, true, false) => get_image("water_with_ice_shelves_nes"),
                (true, true, false, true) => get_image("water_with_ice_shelves_new"),
                (true, false, true, true) => get_image("water_with_ice_shelves_nsw"),
                (false, true, true, true) => get_image("water_with_ice_shelves_esw"),
                (true, true, false, false) => get_image("water_with_ice_shelves_ne"),
                (true, false, true, false) => get_image("water_with_ice_shelves_ns"),
                (true, false, false, true) => get_image("water_with_ice_shelves_nw"),
                (false, true, true, false) => get_image("water_with_ice_shelves_es"),
                (false, true, false, true) => get_image("water_with_ice_shelves_ew"),
                (false, false, true, true) => get_image("water_with_ice_shelves_sw"),
                (true, false, false, false) => get_image("water_with_ice_shelves_n"),
                (false, true, false, false) => get_image("water_with_ice_shelves_e"),
                (false, false, true, false) => get_image("water_with_ice_shelves_s"),
                (false, false, false, true) => get_image("water_with_ice_shelves_w"),
                (false, false, false, false) => get_image("water_with_ice_shelves_none"),
            }
        } else {
            match (north_water, east_water, south_water, west_water) {
                (true, true, true, true) => get_image("water_with_shoreline_nesw"),
                (true, true, true, false) => get_image("water_with_shoreline_nes"),
                (true, true, false, true) => get_image("water_with_shoreline_new"),
                (true, false, true, true) => get_image("water_with_shoreline_nsw"),
                (false, true, true, true) => get_image("water_with_shoreline_esw"),
                (true, true, false, false) => get_image("water_with_shoreline_ne"),
                (true, false, true, false) => get_image("water_with_shoreline_ns"),
                (true, false, false, true) => get_image("water_with_shoreline_nw"),
                (false, true, true, false) => get_image("water_with_shoreline_es"),
                (false, true, false, true) => get_image("water_with_shoreline_ew"),
                (false, false, true, true) => get_image("water_with_shoreline_sw"),
                (true, false, false, false) => get_image("water_with_shoreline_n"),
                (false, true, false, false) => get_image("water_with_shoreline_e"),
                (false, false, true, false) => get_image("water_with_shoreline_s"),
                (false, false, false, true) => get_image("water_with_shoreline_w"),
                (false, false, false, false) => get_image("water_with_shoreline_none"),
            }
        };

        imageops::overlay(base, img, 0, 0);
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

        let img = match self {
            Self::DeepOcean => {
                // TODO: Figure out how the hell this works
                let tl = get_image("deep_ocean_tl_n");
                let tr = get_image("deep_ocean_tr_n");
                let bl = get_image("deep_ocean_bl_n");
                let br = get_image("deep_ocean_br_n");

                imageops::overlay(base, tl, 0, 0);
                imageops::overlay(base, tr, 15, 0);
                imageops::overlay(base, bl, 0, 15);
                imageops::overlay(base, br, 15, 15);

                self.draw_coastline(base, north, east, south, west);

                return;
            }
            Self::Desert => match (north_same, east_same, south_same, west_same) {
                (true, true, true, true) => get_image("desert_nesw"),
                (true, true, true, false) => get_image("desert_nes"),
                (true, true, false, true) => get_image("desert_new"),
                (true, false, true, true) => get_image("desert_nsw"),
                (false, true, true, true) => get_image("desert_esw"),
                (true, true, false, false) => get_image("desert_ne"),
                (true, false, true, false) => get_image("desert_ns"),
                (true, false, false, true) => get_image("desert_nw"),
                (false, true, true, false) => get_image("desert_es"),
                (false, true, false, true) => get_image("desert_ew"),
                (false, false, true, true) => get_image("desert_sw"),
                (true, false, false, false) => get_image("desert_n"),
                (false, true, false, false) => get_image("desert_e"),
                (false, false, true, false) => get_image("desert_s"),
                (false, false, false, true) => get_image("desert_w"),
                (false, false, false, false) => get_image("desert_none"),
            },
            Self::Forest => match (east_same, west_same) {
                (true, true) => get_image("forest_ew"),
                (true, false) => get_image("forest_e"),
                (false, true) => get_image("forest_w"),
                (false, false) => get_image("forest_not_ew"),
            },
            Self::Glacier => match (north_same, east_same, south_same, west_same) {
                (true, true, true, true) => get_image("glacier_nesw"),
                (true, true, true, false) => get_image("glacier_nes"),
                (true, true, false, true) => get_image("glacier_new"),
                (true, false, true, true) => get_image("glacier_nsw"),
                (false, true, true, true) => get_image("glacier_esw"),
                (true, true, false, false) => get_image("glacier_ne"),
                (true, false, true, false) => get_image("glacier_ns"),
                (true, false, false, true) => get_image("glacier_nw"),
                (false, true, true, false) => get_image("glacier_es"),
                (false, true, false, true) => get_image("glacier_ew"),
                (false, false, true, true) => get_image("glacier_sw"),
                (true, false, false, false) => get_image("glacier_n"),
                (false, true, false, false) => get_image("glacier_e"),
                (false, false, true, false) => get_image("glacier_s"),
                (false, false, false, true) => get_image("glacier_w"),
                (false, false, false, false) => get_image("glacier_none"),
            },
            Self::Grassland => get_image("grassland"),
            Self::Hills => match (east_same, west_same) {
                (true, true) => get_image("hills_ew"),
                (true, false) => get_image("hills_e"),
                (false, true) => get_image("hills_w"),
                (false, false) => get_image("hills_not_ew"),
            },
            Self::Jungle => match (north_same, east_same, south_same, west_same) {
                (true, true, true, true) => get_image("jungle_nesw"),
                (true, true, true, false) => get_image("jungle_nes"),
                (true, true, false, true) => get_image("jungle_new"),
                (true, false, true, true) => get_image("jungle_nsw"),
                (false, true, true, true) => get_image("jungle_esw"),
                (true, true, false, false) => get_image("jungle_ne"),
                (true, false, true, false) => get_image("jungle_ns"),
                (true, false, false, true) => get_image("jungle_nw"),
                (false, true, true, false) => get_image("jungle_es"),
                (false, true, false, true) => get_image("jungle_ew"),
                (false, false, true, true) => get_image("jungle_sw"),
                (true, false, false, false) => get_image("jungle_n"),
                (false, true, false, false) => get_image("jungle_e"),
                (false, false, true, false) => get_image("jungle_s"),
                (false, false, false, true) => get_image("jungle_w"),
                (false, false, false, false) => get_image("jungle_none"),
            },
            Self::Lake => {
                let tl = get_image("lake_tl_n");
                let tr = get_image("lake_tr_n");
                let bl = get_image("lake_bl_n");
                let br = get_image("lake_br_n");

                imageops::overlay(base, tl, 0, 0);
                imageops::overlay(base, tr, 15, 0);
                imageops::overlay(base, bl, 0, 15);
                imageops::overlay(base, br, 15, 15);

                self.draw_coastline(base, north, east, south, west);

                return;
            }
            Self::Mountains => match (east_same, west_same) {
                (true, true) => get_image("mountains_ew"),
                (true, false) => get_image("mountains_e"),
                (false, true) => get_image("mountains_w"),
                (false, false) => get_image("mountains_not_ew"),
            },
            Self::Ocean => {
                let tl = get_image("ocean_tl_n");
                let tr = get_image("ocean_tr_n");
                let bl = get_image("ocean_bl_n");
                let br = get_image("ocean_br_n");

                imageops::overlay(base, tl, 0, 0);
                imageops::overlay(base, tr, 15, 0);
                imageops::overlay(base, bl, 0, 15);
                imageops::overlay(base, br, 15, 15);

                self.draw_coastline(base, north, east, south, west);

                return;
            }
            Self::Plains => match (north_same, east_same, south_same, west_same) {
                (true, true, true, true) => get_image("plains_nesw"),
                (true, true, true, false) => get_image("plains_nes"),
                (true, true, false, true) => get_image("plains_new"),
                (true, false, true, true) => get_image("plains_nsw"),
                (false, true, true, true) => get_image("plains_esw"),
                (true, true, false, false) => get_image("plains_ne"),
                (true, false, true, false) => get_image("plains_ns"),
                (true, false, false, true) => get_image("plains_nw"),
                (false, true, true, false) => get_image("plains_es"),
                (false, true, false, true) => get_image("plains_ew"),
                (false, false, true, true) => get_image("plains_sw"),
                (true, false, false, false) => get_image("plains_n"),
                (false, true, false, false) => get_image("plains_e"),
                (false, false, true, false) => get_image("plains_s"),
                (false, false, false, true) => get_image("plains_w"),
                (false, false, false, false) => get_image("plains_none"),
            },
            Self::Swamp => match (north_same, east_same, south_same, west_same) {
                (true, true, true, true) => get_image("swamp_nesw"),
                (true, true, true, false) => get_image("swamp_nes"),
                (true, true, false, true) => get_image("swamp_new"),
                (true, false, true, true) => get_image("swamp_nsw"),
                (false, true, true, true) => get_image("swamp_esw"),
                (true, true, false, false) => get_image("swamp_ne"),
                (true, false, true, false) => get_image("swamp_ns"),
                (true, false, false, true) => get_image("swamp_nw"),
                (false, true, true, false) => get_image("swamp_es"),
                (false, true, false, true) => get_image("swamp_ew"),
                (false, false, true, true) => get_image("swamp_sw"),
                (true, false, false, false) => get_image("swamp_n"),
                (false, true, false, false) => get_image("swamp_e"),
                (false, false, true, false) => get_image("swamp_s"),
                (false, false, false, true) => get_image("swamp_w"),
                (false, false, false, false) => get_image("swamp_none"),
            },
            Self::Tundra => match (north_same, east_same, south_same, west_same) {
                (true, true, true, true) => get_image("tundra_nesw"),
                (true, true, true, false) => get_image("tundra_nes"),
                (true, true, false, true) => get_image("tundra_new"),
                (true, false, true, true) => get_image("tundra_nsw"),
                (false, true, true, true) => get_image("tundra_esw"),
                (true, true, false, false) => get_image("tundra_ne"),
                (true, false, true, false) => get_image("tundra_ns"),
                (true, false, false, true) => get_image("tundra_nw"),
                (false, true, true, false) => get_image("tundra_es"),
                (false, true, false, true) => get_image("tundra_ew"),
                (false, false, true, true) => get_image("tundra_sw"),
                (true, false, false, false) => get_image("tundra_n"),
                (false, true, false, false) => get_image("tundra_e"),
                (false, false, true, false) => get_image("tundra_s"),
                (false, false, false, true) => get_image("tundra_w"),
                (false, false, false, false) => get_image("tundra_none"),
            },
        };

        imageops::overlay(base, img, 0, 0);
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
        let img = match self {
            Self::None => return,
            Self::Oasis => get_image("oasis"),
            Self::Oil => get_image("oil"),
            Self::Pheasant => get_image("pheasant"),
            Self::Silk => get_image("silk"),
            Self::Ivory => get_image("ivory"),
            Self::Resources => get_image("grassland_resources"), // TODO: Dynamic on river
            Self::Coal => get_image("coal"),
            Self::Wine => get_image("wine"),
            Self::Gems => get_image("gems"),
            Self::Fruit => get_image("fruit"),
            Self::Fish => get_image("fish"),
            Self::Gold => get_image("gold"),
            Self::Iron => get_image("iron"),
            Self::Whales => get_image("whales"),
            Self::Buffalo => get_image("buffalo"),
            Self::Wheat => get_image("wheat"),
            Self::Peat => get_image("peat"),
            Self::Spice => get_image("spice"),
            Self::Game => get_image("tundra_game"), // TODO? Does our version support game on
            // forest?
            Self::Furs => get_image("furs"),
        };

        imageops::overlay(base, img, 0, 0);
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

            let img = match (river_north, river_east, river_south, river_west) {
                (true, true, true, true) => get_image("river_nesw"),
                (true, true, true, false) => get_image("river_nes"),
                (true, true, false, true) => get_image("river_new"),
                (true, false, true, true) => get_image("river_nsw"),
                (false, true, true, true) => get_image("river_esw"),
                (true, true, false, false) => get_image("river_ne"),
                (true, false, true, false) => get_image("river_ns"),
                (true, false, false, true) => get_image("river_nw"),
                (false, true, true, false) => get_image("river_es"),
                (false, true, false, true) => get_image("river_ew"),
                (false, false, true, true) => get_image("river_sw"),
                (true, false, false, false) => get_image("river_n"),
                (false, true, false, false) => get_image("river_e"),
                (false, false, true, false) => get_image("river_s"),
                (false, false, false, true) => get_image("river_w"),
                (false, false, false, false) => get_image("river"),
            };

            imageops::overlay(base, img, 0, 0);
        }

        if self.contains(Self::HAS_ROAD) {
            // TODO: When we have ported the road files
            todo!()
        }

        if self.contains(Self::HAS_IRRIGATION) {
            let img = get_image("irrigation");
            imageops::overlay(base, img, 0, 0);
        }

        if self.contains(Self::HAS_MINE) {
            let img = get_image("mine");
            imageops::overlay(base, img, 0, 0);
        }

        if self.contains(Self::HAS_RAILROAD) {
            // TODO: Seems to be missing in the tileset
            todo!()
        }

        if self.contains(Self::HAS_RUINS) {
            let img = get_image("ruins");
            imageops::overlay(base, img, 0, 0);
        }

        if self.contains(Self::HAS_POLLUTION) {
            let img = get_image("pollution");
            imageops::overlay(base, img, 0, 0);
        }

        if self.contains(Self::HAS_FORT) {
            // TODO: We seem to have the images, but idk which ones are which
            todo!()
        }

        if self.contains(Self::HAS_NUCLEAR_FALLOUT) {
            // TODO: We might have the images, but idk which ones are which
        }

        if self.contains(Self::HAS_HUT) {
            let img = get_image("village");
            imageops::overlay(base, img, 0, 0);
        }

        if self.contains(Self::HAS_FARMLAND) {
            let img = get_image("farmland");
            imageops::overlay(base, img, 0, 0);
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
