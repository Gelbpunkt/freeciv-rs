use crate::tiles::Tile;

pub struct World {
    width: usize,
    height: usize,
    grid: Vec<Vec<Tile>>,
    wrapping_x: bool,
    wrapping_y: bool,
}

impl World {
    #[must_use]
    pub fn tile_at(&self, x: usize, y: usize) -> Option<&Tile> {
        let x = if self.wrapping_x && x >= self.width {
            x % self.width
        } else {
            x
        };

        let y = if self.wrapping_y && y >= self.height {
            y % self.height
        } else {
            y
        };

        self.grid.get(x).and_then(|row| row.get(y))
    }

    pub fn tile_at_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        let x = if self.wrapping_x && x >= self.width {
            x % self.width
        } else {
            x
        };

        let y = if self.wrapping_y && y >= self.height {
            y % self.height
        } else {
            y
        };

        self.grid.get_mut(x).and_then(|row| row.get_mut(y))
    }
}
