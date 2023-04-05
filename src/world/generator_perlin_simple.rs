use noise::{NoiseFn, Perlin, Seedable};
use rand::{rngs::SmallRng, Rng, SeedableRng};

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
            water_percentage: 0.05,
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
    let mut rng = SmallRng::from_entropy();
    let perlin = Perlin::new(params.seed);
    let mut world =
        vec![
            vec![Tile::new(Terrain::Grassland, Special::None, Flags::empty()); params.width];
            params.height
        ];

    println!("Generating height map");

    // Generate height map using Perlin noise
    let mut heights = vec![vec![0.0; params.width]; params.height];
    let mut max_height = 0.0f32;
    let mut min_height = 1.0f32;
    for y in 0..params.height {
        for x in 0..params.width {
            let height_val = perlin.get([
                (x as f64 / params.width as f64) * 4.0,
                (y as f64 / params.height as f64) * 4.0,
            ]);
            heights[x][y] = height_val as f32;
            max_height = max_height.max(height_val as f32);
            min_height = min_height.min(height_val as f32);
        }
    }

    println!("Normalizing height map");

    // Normalize height map to values between 0 and 1
    for y in 0..params.height {
        for x in 0..params.width {
            heights[x][y] = (heights[x][y] - min_height) / (max_height - min_height);
        }
    }

    println!("Placing water tiles");

    // Set water tiles
    let mut water_threshold = rng.gen_range(0.0..1.0 - params.water_percentage);
    let mut water_count = 0;
    for y in 0..params.height {
        for x in 0..params.width {
            if heights[x][y] < water_threshold {
                world[x][y].terrain = Terrain::Ocean;
                water_count += 1;
            }
        }
    }

    println!("Adjusting water threshold");

    // TODO: This it endless for tiny water percentages
    // Adjust water threshold if needed to reach desired water percentage
    if water_count as f32 / (params.width * params.height) as f32 != params.water_percentage {
        let mut diff = (water_count as f32 / (params.width * params.height) as f32
            - params.water_percentage)
            .abs();

        while diff.abs() > 0.001 {
            if diff > 0.0 {
                water_threshold += 0.0001;
            } else {
                water_threshold -= 0.0001;
            }
            water_count = 0;
            for x in 0..params.width {
                for y in 0..params.height {
                    if heights[x][y] < water_threshold {
                        world[x][y].terrain = Terrain::Ocean;
                        water_count += 1;
                    }
                }
            }
            diff = (water_count as f32 / (params.width * params.height) as f32
                - params.water_percentage)
                .abs();
        }
    }

    println!("Generating other terrain");

    // Generate other terrain types based on height map
    for y in 0..params.height {
        for x in 0..params.width {
            if world[x][y].terrain == Terrain::Grassland {
                if heights[x][y] < 0.3 {
                    world[x][y].terrain = Terrain::Swamp;
                } else if heights[x][y] < 0.4 {
                    world[x][y].terrain = Terrain::Forest;
                } else if heights[x][y] < 0.6 {
                    world[x][y].terrain = Terrain::Hills;
                } else if heights[x][y] < 0.8 {
                    world[x][y].terrain = Terrain::Mountains;
                } else {
                    world[x][y].terrain = Terrain::Glacier;
                }
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
