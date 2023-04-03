use bitflags::bitflags;

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
#[derive(Debug)]
pub struct Tile {
    terrain: Terrain,
    special: Special,
    flags: Flags,
    transform_status: TransformStatus,
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
#[derive(Debug)]
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

/// A possibly ongoing transformation on a [`Tile`].
#[derive(Debug, PartialEq, Eq)]
enum TransformStatus {
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
#[derive(Debug, PartialEq, Eq)]
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
