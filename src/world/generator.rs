use noise::{NoiseFn, OpenSimplex};
use rand::{rngs::StdRng, Rng, SeedableRng};

use super::World;
use crate::tiles::{Flags, Special, Terrain, Tile};

pub struct Parameters {
    pub width: usize,
    pub height: usize,
    pub wrapping_x: bool,
    pub wrapping_y: bool,
    pub water_percentage: f32,
    pub seed: u32,
    pub land_distribution: LandDistribution,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            width: 64,
            height: 64,
            wrapping_x: true,
            wrapping_y: false,
            water_percentage: 0.6,
            seed: 0,
            land_distribution: LandDistribution::Spread,
        }
    }
}

pub enum LandDistribution {
    Spread,
    Continguous,
}

#[must_use]
pub fn generate(params: Parameters) -> World {
    let mut rng = StdRng::from_entropy();
    let noise = OpenSimplex::new(params.seed);

    // Generate height map
    let mut height_map = vec![vec![0.0; params.width]; params.height];
    for octave in 0..5 {
        let frequency = 2.0_f64.powi(octave as i32);
        let amplitude = 0.5_f64.powi(octave as i32);
        for y in 0..params.height {
            for x in 0..params.width {
                height_map[y][x] +=
                    noise.get([(x as f64) * frequency, (y as f64) * frequency]) * amplitude;
            }
        }
    }

    // Normalize height map
    let mut min_height = f64::INFINITY;
    let mut max_height = f64::NEG_INFINITY;
    for y in 0..params.height {
        for x in 0..params.width {
            min_height = min_height.min(height_map[y][x]);
            max_height = max_height.max(height_map[y][x]);
        }
    }
    for y in 0..params.height {
        for x in 0..params.width {
            height_map[y][x] = (height_map[y][x] - min_height) / (max_height - min_height);
        }
    }

    // Add islands
    let mut island_count = 0;
    let total_count = params.width * params.height;
    let island_target = ((1.0 - params.water_percentage) * total_count as f32).round() as usize;
    while island_count < island_target {
        let x = rng.gen_range(0..params.width);
        let y = rng.gen_range(0..params.height);
        if height_map[y][x] > 0.4 && height_map[y][x] < 0.8 {
            height_map[y][x] = 0.0;
            island_count += 1;
        }
    }

    // Add terrain types
    let mut world =
        vec![
            vec![Tile::new(Terrain::Ocean, Special::None, Flags::empty()); params.width];
            params.height
        ];
    for y in 0..params.height {
        for x in 0..params.width {
            let feature_value = height_map[y][x];
            let terrain_type = if feature_value < 0.1 {
                Terrain::Ocean
            } else if feature_value < 0.2 {
                Terrain::Plains
            } else if feature_value < 0.3 {
                Terrain::Grassland
            } else if feature_value < 0.4 {
                Terrain::Hills
            } else if feature_value < 0.5 {
                Terrain::Forest
            } else if feature_value < 0.6 {
                Terrain::Swamp
            } else if feature_value < 0.7 {
                Terrain::Jungle
            } else if feature_value < 0.8 {
                Terrain::Mountains
            } else {
                Terrain::Desert
            };
            world[y][x].terrain = terrain_type;
        }
    }

    // Flood fill to ensure contiguous oceans
    let mut visited = vec![vec![false; params.width]; params.height];
    for y in 0..params.height {
        for x in 0..params.width {
            if !visited[y][x] && world[y][x].terrain.is_water() {
                let mut queue = vec![(x, y)];
                visited[y][x] = true;
                let mut island_size = 0;
                let mut ocean_size = 1;
                while let Some((x, y)) = queue.pop() {
                    island_size += 1;
                    if x > 0 && !visited[y][x - 1] && world[y][x - 1].terrain.is_water() {
                        queue.push((x - 1, y));
                        visited[y][x - 1] = true;
                        ocean_size += 1;
                    }
                    if x < params.width - 1
                        && !visited[y][x + 1]
                        && world[y][x + 1].terrain.is_water()
                    {
                        queue.push((x + 1, y));
                        visited[y][x + 1] = true;
                        ocean_size += 1;
                    }
                    if y > 0 && !visited[y - 1][x] && world[y - 1][x].terrain.is_water() {
                        queue.push((x, y - 1));
                        visited[y - 1][x] = true;
                        ocean_size += 1;
                    }
                    if y < params.height - 1
                        && !visited[y + 1][x]
                        && world[y + 1][x].terrain.is_water()
                    {
                        queue.push((x, y + 1));
                        visited[y + 1][x] = true;
                        ocean_size += 1;
                    }
                }
                if island_size < ocean_size {
                    for y in 0..params.height {
                        for x in 0..params.width {
                            if visited[y][x] {
                                world[y][x].terrain = Terrain::Ocean;
                            }
                        }
                    }
                }
            }
        }
    }

    // Add features
    let feature_map = {
        let mut feature_map = vec![vec![0.0; params.width]; params.height];
        for octave in 0..3 {
            let frequency = 8.0_f64.powi(octave as i32);
            let amplitude = 0.5_f64.powi(octave as i32);
            for y in 0..params.height {
                for x in 0..params.width {
                    feature_map[y][x] +=
                        noise.get([(x as f64) * frequency, (y as f64) * frequency]) * amplitude;
                }
            }
        }
        feature_map
    };

    for y in 0..params.height {
        for x in 0..params.width {
            let terrain_type = world[y][x].terrain;
            let feature_value = feature_map[y][x];
            match terrain_type {
                Terrain::Ocean | Terrain::Swamp => {
                    if feature_value > 0.3 {
                        world[x][y].terrain = Terrain::Forest;
                    }
                }
                Terrain::Desert | Terrain::Plains | Terrain::Grassland => {
                    if feature_value > 0.5 {
                        world[x][y].terrain = Terrain::Forest;
                    }
                }
                Terrain::Hills | Terrain::Jungle | Terrain::Mountains => {
                    if feature_value > 0.4 {
                        world[x][y].terrain = Terrain::Forest;
                    }
                }
                Terrain::Forest => {
                    if feature_value > 0.5 {
                        let mut adjacent_types = vec![];
                        if x > 0 {
                            adjacent_types.push(world[y][x - 1].terrain);
                        }
                        if x < params.width - 1 {
                            adjacent_types.push(world[y][x + 1].terrain);
                        }
                        if y > 0 {
                            adjacent_types.push(world[y - 1][x].terrain);
                        }
                        if y < params.height - 1 {
                            adjacent_types.push(world[y + 1][x].terrain);
                        }
                        if adjacent_types.iter().any(|&t| t.is_water()) {
                            world[y][x].terrain = Terrain::Swamp;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    let world = World {
        width: params.width,
        height: params.height,
        grid: world,
        wrapping_x: params.wrapping_x,
        wrapping_y: params.wrapping_y,
    };

    world
}
