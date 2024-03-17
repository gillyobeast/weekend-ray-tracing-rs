use crate::{
    hittable::{HitRecord, Hittable},
    vec3::Point3,
};

pub(crate) struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub(crate) fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit_range(&self, ray: &crate::ray::Ray, range: std::ops::Range<f64>) -> Option<HitRecord> {
        let r_squared = self.radius * self.radius;

        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = oc * ray.direction();
        let c = oc.length_squared() - r_squared;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }
        let sqrt_d = discriminant.sqrt();

        // find the nearest root that lies in the acceptable range:

        let root = (-half_b - sqrt_d) / a;
        if !range.contains(&root) {
            let root = (-half_b + sqrt_d) / a;
            if !range.contains(&root) {
                return None;
            }
        }

        let point = ray.at(root);
        let normal = (point - self.center) / self.radius;
        Some(HitRecord::new(point, root, &normal, ray))
    }
}
