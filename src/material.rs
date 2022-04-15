use rand::random;

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
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let reflected = ray.direction.unit_vector().reflect(hit.normal());
        let scattered = Ray::new(
            hit.point().clone(),
            reflected + Vec3::random_in_unit_sphere() * self.fuzz,
        );
        if scattered.direction.dot(hit.normal()) > 0.0 {
            Some(Scatter::new(scattered, self.albedo.clone()))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    index_refraction: f64,
}

impl Dielectric {
    pub fn new(index_refraction: f64) -> Dielectric {
        Dielectric { index_refraction }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let refraction_ratio = if hit.front_face {
            1.0 / self.index_refraction
        } else {
            self.index_refraction
        };
        let unit_direction = ray.direction.unit_vector();
        let cos_theta = f64::min(hit.normal().dot(&-&unit_direction), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_refract || reflectance(cos_theta, refraction_ratio) > random() {
            unit_direction.reflect(hit.normal())
        } else {
            Vec3::refract(&unit_direction, hit.normal(), refraction_ratio)
        };

        Some(Scatter::new(
            Ray::new(hit.point().clone(), direction),
            Vec3::new(1.0, 1.0, 1.0),
        ))
    }
}

// Schlick approximation for reflectance
fn reflectance(cosine: f64, refraction_ratio: f64) -> f64 {
    let r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
    let r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
}
