use crate::{material::Material, ray::Ray, vec3::Vec3};

pub struct Hit<'a> {
    point: Vec3,
    normal: Vec3,
    t: f64,
    pub front_face: bool,
    material: &'a dyn Material,
}

impl<'a> Hit<'a> {
    pub fn new(
        ray: &Ray,
        point: Vec3,
        outward_normal: Vec3,
        t: f64,
        material: &'a dyn Material,
    ) -> Hit<'a> {
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
            material,
        }
    }

    pub fn point(&self) -> &Vec3 {
        &self.point
    }

    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }

    pub fn material(&'a self) -> &'a (dyn Material + 'a) {
        self.material
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable + Send + Sync>>,
}

impl HittableList {
    pub fn new(objects: Vec<Box<dyn Hittable + Send + Sync>>) -> HittableList {
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
