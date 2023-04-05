use std::ops::{Deref, DerefMut};

use image::{imageops, DynamicImage};

use crate::tiles::{Tile, TILE_IMAGE_SIZE};

pub struct World {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) grid: Vec<Vec<Tile>>,
    pub(crate) wrapping_x: bool,
    pub(crate) wrapping_y: bool,
}

pub struct TileRef<'a> {
    x: usize,
    y: usize,
    world: &'a World,
}

impl TileRef<'_> {
    pub fn north(&self) -> Option<Self> {
        let y = if self.y == 0 && self.world.wrapping_y {
            self.world.height - 1
        } else if self.y == 0 {
            return None;
        } else {
            self.y - 1
        };

        self.world.tile_at(self.x, y)
    }

    pub fn west(&self) -> Option<Self> {
        let x = if self.x == 0 && self.world.wrapping_x {
            self.world.width - 1
        } else if self.x == 0 {
            return None;
        } else {
            self.x - 1
        };

        self.world.tile_at(x, self.y)
    }

    pub fn east(&self) -> Option<Self> {
        let x = if self.x == self.world.width - 1 && self.world.wrapping_x {
            0
        } else if self.x == self.world.width - 1 {
            return None;
        } else {
            self.x + 1
        };

        self.world.tile_at(x, self.y)
    }

    pub fn south(&self) -> Option<Self> {
        let y = if self.y == self.world.height - 1 && self.world.wrapping_y {
            0
        } else if self.y == self.world.height - 1 {
            return None;
        } else {
            self.y + 1
        };

        self.world.tile_at(self.x, y)
    }

    pub fn north_east(&self) -> Option<Self> {
        self.north().and_then(|t| t.east())
    }

    pub fn south_east(&self) -> Option<Self> {
        self.south().and_then(|t| t.east())
    }

    pub fn south_west(&self) -> Option<Self> {
        self.south().and_then(|t| t.west())
    }

    pub fn north_west(&self) -> Option<Self> {
        self.north().and_then(|t| t.west())
    }
}

impl<'a> Deref for TileRef<'a> {
    type Target = Tile;

    fn deref(&self) -> &Self::Target {
        // SAFETY: Bounds are checked by `tile_at` methods.
        unsafe { self.world.grid.get_unchecked(self.y).get_unchecked(self.x) }
    }
}

pub struct TileRefMut<'a> {
    x: usize,
    y: usize,
    world: &'a mut World,
}

impl<'a> TileRefMut<'a> {
    pub fn into_immutable(self) -> TileRef<'a> {
        TileRef {
            x: self.x,
            y: self.y,
            world: self.world,
        }
    }
}

impl<'a> Deref for TileRefMut<'a> {
    type Target = Tile;

    fn deref(&self) -> &Self::Target {
        // SAFETY: Bounds are checked by `tile_at` methods.
        unsafe { self.world.grid.get_unchecked(self.y).get_unchecked(self.x) }
    }
}

impl<'a> DerefMut for TileRefMut<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: Bounds are checked by `tile_at` methods.
        unsafe {
            self.world
                .grid
                .get_unchecked_mut(self.y)
                .get_unchecked_mut(self.x)
        }
    }
}

impl World {
    #[must_use]
    pub fn tile_at<'a>(&'a self, x: usize, y: usize) -> Option<TileRef<'a>> {
        let x = if self.wrapping_x && x >= self.width {
            x % self.width
        } else if x >= self.width {
            return None;
        } else {
            x
        };

        let y = if self.wrapping_y && y >= self.height {
            y % self.height
        } else if y >= self.height {
            return None;
        } else {
            y
        };

        Some(TileRef { x, y, world: self })
    }

    pub fn tile_at_mut<'a>(&'a mut self, x: usize, y: usize) -> Option<TileRefMut<'a>> {
        let x = if self.wrapping_x && x >= self.width {
            x % self.width
        } else if x >= self.width {
            return None;
        } else {
            x
        };

        let y = if self.wrapping_y && y >= self.height {
            y % self.height
        } else if y >= self.height {
            return None;
        } else {
            y
        };

        Some(TileRefMut { x, y, world: self })
    }

    pub fn render(&self) -> DynamicImage {
        let mut image = DynamicImage::new_rgba8(
            TILE_IMAGE_SIZE * self.width as u32,
            TILE_IMAGE_SIZE * self.height as u32,
        );

        for y in 0..self.height {
            for x in 0..self.width {
                // SAFETY: It is always within height and width.
                let tile = unsafe { self.tile_at(x, y).unwrap_unchecked() };
                let north = tile.north();
                let east = tile.east();
                let south = tile.south();
                let west = tile.west();
                let north_east = tile.north_west();
                let south_east = tile.south_east();
                let south_west = tile.south_west();
                let north_west = tile.north_west();

                let px_x = x as u32 * TILE_IMAGE_SIZE;
                let px_y = y as u32 * TILE_IMAGE_SIZE;

                let mut tile_section =
                    imageops::crop(&mut image, px_x, px_y, TILE_IMAGE_SIZE, TILE_IMAGE_SIZE);

                tile.render(
                    tile_section.deref_mut(),
                    north.as_deref(),
                    north_east.as_deref(),
                    east.as_deref(),
                    south_east.as_deref(),
                    south.as_deref(),
                    south_west.as_deref(),
                    west.as_deref(),
                    north_west.as_deref(),
                );
            }
        }

        image
    }
}
