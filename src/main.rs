use std::fs::File;
use std::io::Write;
use std::time::SystemTime;

use color::{write, Colour};

mod color;
mod ray;
mod vec3;

fn main() {
    let start = SystemTime::now();

    let mut image = File::create("image.ppm").unwrap();
    image.set_len(0).unwrap();

    // image
    let ideal_aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // calculate image height
    let image_height = int(float(image_width) / ideal_aspect_ratio);
    let image_height = if image_height < 1 { 1 } else { image_height };

    let effective_aspect_ratio = float(image_width) / float(image_height);

    // viewport
    let viewport_height = 2.0;
    let viewport_width = viewport_height * effective_aspect_ratio;

    // render

    writeln!(&mut image, "P3\n{image_width} {image_height}\n255").unwrap();

    for j in 0..image_height {
        print!("\rScanlines remaining: {}  ", image_height - j);
        for i in 0..image_width {
            let colour = Colour::new(
                float(i) / float(image_width - 1),
                float(j) / float(image_height - 1),
                0.0,
            );

            write(&mut image, colour);
        }
    }
    println!("\rDone!                    ");
    println!("took {:?}", start.elapsed().expect("unknown"));
}

fn int(f: f64) -> i32 {
    f as i32
}

fn float(i: i32) -> f64 {
    i as f64
}
