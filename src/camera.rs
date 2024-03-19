use std::{f64::INFINITY, fs::File, io::Write};

use crate::{
    colour::{write, Colour, BLUE, WHITE},
    hittable::Hittable,
    ray::Ray,
    vec3::{Point3, Vec3},
};
pub(crate) struct Camera {
    // pub aspect_ratio: f64,     // ratio of image width over height
    pub image_width: u32,      // rendered image width in px
    image_height: u32,         // rendered image height
    center: Point3,            // camera center
    pixel_00_location: Point3, // location of px 0,0
    pixel_delta_u: Vec3,       // offset to pixel →
    pixel_delta_v: Vec3,       // offset to pixel ↓
}

impl Camera {
    // private fns for supporting render
    pub(crate) fn new(ideal_aspect_ratio: f64, image_width: u32) -> Self {
        let image_height = (image_width as f64 / ideal_aspect_ratio) as u32;
        let image_height = if image_height < 1 { 1 } else { image_height };
        let center = Point3::origin();

        let aspect_ratio = image_width as f64 / image_height as f64;
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;

        // calculate vectors across horizontal and vertical edges of viewport
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        //calculate horizontal and vertical delta vectors from px to px
        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);

        // calculate location of upper-left pixel
        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - (viewport_u / 2.0) - (viewport_v / 2.0);

        let pixel_00_location = viewport_upper_left + (pixel_delta_u + pixel_delta_v) / 2.0;

        Self {
            // aspect_ratio,
            image_width,
            image_height,
            center,
            pixel_00_location,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    fn ray_colour(&self, ray: &Ray, world: &dyn Hittable) -> Colour {
        if let Some(hit_record) = world.hit_range(ray, 0.0..INFINITY) {
            return (hit_record.normal() + WHITE) / 2.0;
        }

        let unit_direction = ray.direction().unit_vector();

        let a = (unit_direction.y + 1.0) / 2.0;
        WHITE * (1.0 - a) + BLUE * a
    }
}

pub(crate) trait Renderer {
    fn render(&self, world: &dyn Hittable);
}

impl Renderer for Camera {
    fn render(&self, world: &dyn Hittable) {
        let mut image = File::create("image.ppm").unwrap();

        writeln!(
            &mut image,
            "P3\n{} {}\n255",
            self.image_width, self.image_height
        )
        .unwrap();

        for j in 0..self.image_height {
            print!("\rScanlines remaining: {}  ", self.image_height - j);
            for i in 0..self.image_width {
                let pixel_center = self.pixel_00_location
                    + (self.pixel_delta_u * i as f64)
                    + (self.pixel_delta_v * j as f64);
                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction);

                let colour = self.ray_colour(&ray, world);
                write(&mut image, colour);
            }
        }
        println!("\rDone!                    ");
    }
}
