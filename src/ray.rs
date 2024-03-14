use crate::vec3::{Point3, Vec3};

pub(crate) struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    fn origin(self) -> Point3 {
        self.origin
    }
    fn direction(self) -> Vec3 {
        self.direction
    }

    fn at(self, t: f64) -> Point3 {
        self.origin + (self.direction * t)
    }
}
