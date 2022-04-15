use crate::{
    hittable::{Hit, Hittable},
    material::Material,
    ray::Ray,
    vec3::Vec3,
};

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Box<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl<'a> Hittable<'a> for Sphere {
    fn hit(self: & Sphere, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let oc = &ray.origin - &self.center;
        let a = ray.direction.dot(&ray.direction);
        let half_b = ray.direction.dot(&oc);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }
        let discriminant_sqrt = discriminant.sqrt();
        let left_root = (-half_b - discriminant_sqrt) / a;
        let t = if left_root < t_min || left_root > t_max {
            let right_root = (-half_b + discriminant_sqrt) / a;
            if right_root < t_min || right_root > t_max {
                return None;
            }
            right_root
        } else {
            left_root
        };

        let point = ray.at(t);
        let outward_normal = (&point - &self.center) / self.radius;
        Some(Hit::new(
            ray,
            point,
            outward_normal,
            t,
            self.material.as_ref(),
        ))
    }
}
