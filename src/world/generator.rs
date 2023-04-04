use super::World;

pub struct Parameters {
    pub width: usize,
    pub height: usize,
    pub wrapping_x: bool,
    pub wrapping_y: bool,
    pub water_percentage: f32,
    pub land_distribution: LandDistribution,
}

pub enum LandDistribution {
    Spread,
    Continguous,
}

#[must_use] pub fn generate(_params: Parameters) -> World {
    todo!()
}
