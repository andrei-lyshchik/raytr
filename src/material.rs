use crate::{hittable::Hit, ray::Ray, vec3::Vec3};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter>;
}

pub struct Scatter {
    ray: Ray,
    attenuation: Vec3,
}

impl Scatter {
    pub fn new(scattered: Ray, attenuation: Vec3) -> Scatter {
        Scatter {
            ray: scattered,
            attenuation,
        }
    }

    pub fn ray(&self) -> &Ray {
        &self.ray
    }

    pub fn attenuation(&self) -> &Vec3 {
        &self.attenuation
    }
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let scatter_direction = hit.normal() + Vec3::random_in_hemisphere(hit.normal());
        let scatter_direction = if scatter_direction.near_zero() {
            hit.normal().clone()
        } else {
            scatter_direction
        };
        Some(Scatter::new(
            Ray::new(hit.point().clone(), scatter_direction),
            self.albedo.clone(),
        ))
    }
}

pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Metal {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let reflected = ray.direction.unit_vector().reflect(hit.normal());
        let scattered = Ray::new(hit.point().clone(), reflected);
        if scattered.direction.dot(hit.normal()) > 0.0 {
            Some(Scatter::new(scattered, self.albedo.clone()))
        } else {
            None
        }
    }
}
