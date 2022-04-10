use crate::{ray::Ray, vec3::Vec3};

pub struct Hit {
    point: Vec3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl Hit {
    pub fn new(ray: &Ray, point: Vec3, outward_normal: Vec3, t: f64) -> Hit {
        let (normal, front_face) = if ray.direction.dot(&outward_normal) > 0.0 {
            (-outward_normal, false)
        } else {
            (outward_normal, true)
        };
        Hit {
            point,
            normal,
            t,
            front_face,
        }
    }

    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> HittableList {
        HittableList { objects }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut result: Option<Hit> = None;
        let mut t_closest_so_far = t_max;

        for hittable in self.objects.iter() {
            if let Some(hit) = hittable.hit(ray, t_min, t_closest_so_far) {
                t_closest_so_far = hit.t;
                result = Some(hit);
            }
        }

        result
    }
}