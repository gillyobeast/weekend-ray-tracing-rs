use std::fs::File;
use std::io::Write;

use crate::vec3::Vec3;

pub(crate) type Colour = Vec3;
pub(crate) fn write(image: &mut File, colour: Vec3) {
    let r = (255.99 * colour.x) as i32;
    let g = (255.99 * colour.y) as i32;
    let b = (255.99 * colour.z) as i32;

    writeln!(image, "{r} {g} {b}").unwrap();
}
