use std::fs::File;
use std::io::Write;
use std::time::SystemTime;

use color::{write, Colour};

use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

mod color;
mod ray;
mod vec3;

fn main() {
    let start = SystemTime::now();

    let mut image = File::create("image.ppm").unwrap();
    image.set_len(0).unwrap();

    // image
    let ideal_aspect_ratio = 16.0 / 9.0;
    let image_width = 100;

    // calculate image height
    let image_height = int(float(image_width) / ideal_aspect_ratio);
    let image_height = if image_height < 1 { 1 } else { image_height };

    let effective_aspect_ratio = float(image_width) / float(image_height);

    // camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * effective_aspect_ratio;
    let camera_center = Point3::origin();

    // calculate vectors across horizontal and vertical edges of viewport
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    //calculate horizontal and vertical delta vectors from px to px
    let pixel_delta_u = viewport_u / (image_width as f64);
    let pixel_delta_v = viewport_v / (image_height as f64);

    // calculate location of upper-left pixel
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - (viewport_u / 2.0) - (viewport_v / 2.0);

    let pixel_00_location = viewport_upper_left + (pixel_delta_u + pixel_delta_v) / 2.0;

    // render

    writeln!(&mut image, "P3\n{image_width} {image_height}\n255").unwrap();

    for j in 0..image_height {
        print!("\rScanlines remaining: {}  ", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                pixel_00_location + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let colour = ray_colour(ray);
            write(&mut image, colour);
        }
    }
    println!("\rDone!                    ");
    println!("took {:?}", start.elapsed().expect("unknown"));
}

const BLUE: Colour = Colour::new(0.5, 0.7, 1.0);
const WHITE: Colour = Colour::new(1.0, 1.0, 1.0);
const RED: Colour = Colour::new(1.0, 0.0, 0.0);

fn hit_sphere(center: Vec3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin() - center;
    let a = ray.direction() * ray.direction();
    let b = 2.0 * (oc * ray.direction());
    let c = (oc * oc) - (radius * radius);

    let discriminant = (b * b) - (4.0 * a * c);
    discriminant > 0.0
}

fn ray_colour(ray: Ray) -> Colour {
    if hit_sphere(Point3::new(0.0, 0.0, -1.0), -0.5, &ray) {
        return RED;
    }

    let unit_direction = ray.direction().unit_vector();

    let a = (unit_direction.y + 1.0) / 2.0;
    WHITE * (1.0 - a) + BLUE * a
}

fn int(f: f64) -> i32 {
    f as i32
}

fn float(i: i32) -> f64 {
    i as f64
}
