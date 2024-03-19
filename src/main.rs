use std::collections::LinkedList;
use std::time::SystemTime;

use crate::camera::Camera;
use crate::camera::Renderer;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

mod camera;
mod colour;
mod common;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

fn main() {
    let start = SystemTime::now();

    // image
    let ideal_aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let camera = Camera::new(ideal_aspect_ratio, image_width);
    // world

    let mut hittables = LinkedList::new();
    hittables.push_front(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    hittables.push_front(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));
    let world = HittableList::new(hittables);

    camera.render(&world);

    println!("took {:?}", start.elapsed().expect("unknown"));
}

