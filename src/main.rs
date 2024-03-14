use std::fs::File;
use std::io::Write;
use std::time::SystemTime;

use color::{write, Colour};

mod color;
mod vec3;

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
                (i as f64) / (image_width as f64 - 1.0),
                (j as f64) / (image_width as f64 - 1.0),
                0.0,
            );

            write(&mut image, colour);
        }
    }
    println!("\rDone!                    ");
    println!("took {:?}", start.elapsed().expect("unknown"));
}
