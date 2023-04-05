#![deny(
    clippy::pedantic,
    clippy::missing_const_for_fn,
    rustdoc::broken_intra_doc_links
)]

use image::ImageFormat;
use world::{generate, Parameters};

pub mod research;
pub mod tiles;
pub mod world;

fn main() {
    let params = Parameters::default();
    let world = generate(params);
    let rendered = world.render();
    rendered
        .save_with_format("map.png", ImageFormat::Png)
        .unwrap();
}
