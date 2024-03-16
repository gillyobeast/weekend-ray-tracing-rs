use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub(crate) struct HitRecord {
    pub(crate) point: Point3,
    pub(crate) normal: Vec3,
    pub(crate) t: f64,
}

pub(crate) trait Hittable {
    fn hit(self, ray: &Ray, ray_t_min: f64, ray_t_max: f64) -> Option<HitRecord>;
}
