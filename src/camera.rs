use crate::{color::Colour, hittable::Hittable, ray::Ray};

struct Camera {}

impl Camera {
    // private fns for supporting render
    fn initialise(self) {
        todo!()
    }
    fn ray_colour(ray: &Ray, world: &dyn Hittable) -> Colour {
        todo!()
    }
}

trait Renderer {
    fn render(world: &dyn Hittable);
}

impl Renderer for Camera{
    fn render(world: &dyn Hittable) {
        todo!()
    }
}
