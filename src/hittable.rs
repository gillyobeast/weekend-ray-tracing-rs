use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub(crate) struct HitRecord {
    point: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    /// sets the hit record normal vector
    /// outward_normal is assumed to have unit length
    pub(crate) fn new(point: Vec3, t: f64, outward_normal: &Vec3, ray: &Ray) -> Self {
        let front_face = (&ray.direction() * outward_normal) < 0.0;

        let normal = if front_face {
            outward_normal.clone()
        } else {
            -outward_normal
        };

        Self {
            point,
            normal,
            t,
            front_face,
        }
    }
    pub(crate) fn t(&self) -> f64 {
        self.t
    }

    pub(crate) fn normal(&self) -> Vec3 {
        self.normal
    }
}

pub(crate) trait Hittable {
    fn hit(&self, ray: &Ray, ray_t_min: f64, ray_t_max: f64) -> Option<HitRecord>;
}
