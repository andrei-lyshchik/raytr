use std::f64::consts::PI;

use crate::{ray::Ray, vec3::Vec3};

pub struct Camera {
    pub aspect_ratio: f64,
    pub origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov_degrees: f64,
        aspect_ratio: f64,
    ) -> Camera {
        let theta = degrees_to_radians(vfov_degrees);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * aspect_ratio;

        let w = (&lookfrom - lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);
        let origin = lookfrom;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let lower_left_corner =
            &origin - &horizontal / 2.0 - &vertical / 2.0 - w;

        Camera {
            aspect_ratio,
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin.clone(),
            &self.lower_left_corner + &self.horizontal * s + &self.vertical * t - &self.origin,
        )
    }
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
