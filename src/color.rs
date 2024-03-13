use std::fs::File;

use crate::vec3::Vec3;

type Colour = Vec3;
fn write(mut image: File, colour: Colour) {
    writeln!(
        &mut image,
        "{} {} {}",
        colour.x * 255.99,
        colour.y * 255.99,
        colour.z * 255.99
    )
    .unwrap();
}
