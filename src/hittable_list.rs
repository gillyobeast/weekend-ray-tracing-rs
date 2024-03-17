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
    fn hit_range(
        &self,
        ray: &crate::ray::Ray,
        range: std::ops::Range<f64>,
    ) -> Option<crate::hittable::HitRecord> {
        // hmm this is imperative as hell, i hate it...
        let mut temp_rec = None;
        let mut closest_t = range.end;

        for hittable in &self.hittables {
            if let Some(hit_record) = hittable.hit_range(ray, range.start..closest_t) {
                closest_t = hit_record.t();
                temp_rec = Some(hit_record);
            }
        }

        temp_rec
    }
}
