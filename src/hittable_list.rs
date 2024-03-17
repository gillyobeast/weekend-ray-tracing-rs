use std::collections::LinkedList;

use crate::{hittable::Hittable, sphere::Sphere};

pub(crate) struct HittableList {
    hittables: LinkedList<Sphere>,
}

impl HittableList {
    pub(crate) fn new(hittables: LinkedList<Sphere>) -> Self {
        Self { hittables }
    }
}

impl Hittable for HittableList {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        ray_t_min: f64,
        ray_t_max: f64,
    ) -> Option<crate::hittable::HitRecord> {
        // hmm this is imperative as hell, i hate it...
        let mut temp_rec = None;
        let mut closest_t = ray_t_max;

        for hittable in &self.hittables {
            if let Some(hit_record) = hittable.hit(ray, ray_t_min, closest_t) {
                closest_t = hit_record.t();
                temp_rec = Some(hit_record);
            }
        }

        temp_rec
    }
}
