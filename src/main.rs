use std::fs::File;
use std::io::Write;
use std::time::SystemTime;

use color::{write, Colour};

mod color;
mod vec3;
mod ray;

fn main() {
    let start = SystemTime::now();

    let mut image = File::create("image.ppm").unwrap();
    image.set_len(0).unwrap();

    // image
    let image_width = 256;
    let image_height = 256;

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

fn float(i: i32) -> f64 {
    i as f64
}
