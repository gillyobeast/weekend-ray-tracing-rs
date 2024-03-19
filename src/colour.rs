use std::fs::File;
use std::io::Write;

use crate::vec3::Vec3;

pub(crate) const BLUE: Colour = Colour::new(0.5, 0.7, 1.0);
pub(crate) const WHITE: Colour = Colour::new(1.0, 1.0, 1.0);

const MIN: f64 = 0.0;
const MAX: f64 = 0.999;
const COLOUR_DEPTH: f64 = 255.0;

pub(crate) type Colour = Vec3;
pub(crate) fn write(image: &mut File, colour: Vec3, samples_per_pixel: u32) {
    let scale = 1.0 / samples_per_pixel as f64;

    let colour_value = |value: f64| ((value * scale).clamp(MIN, MAX) * COLOUR_DEPTH) as u32;

    let r = colour_value(colour.x);
    let g = colour_value(colour.y);
    let b = colour_value(colour.z);

    writeln!(image, "{r} {g} {b}").unwrap();
}
