use std::f64::consts::PI;

use crate::{ray::Ray, vec3::Vec3};

pub struct Camera {
    pub aspect_ratio: f64,
    pub origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov_degrees: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
    ) -> Camera {
        let theta = degrees_to_radians(vfov_degrees);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * aspect_ratio;

        let w = (&lookfrom - lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = &u * viewport_width * focus_distance;
        let vertical = &v * viewport_height * focus_distance;
        let lower_left_corner =
            &origin - &horizontal / 2.0 - &vertical / 2.0 - &w * focus_distance;

        let lens_radius = aperture / 2.0;

        Camera {
            aspect_ratio,
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius,
        }
    }

    pub fn ray(&self, s: f64, t: f64) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = &self.u * rd.x + &self.v * rd.y;
        Ray::new(
            &self.origin + &offset,
            &self.lower_left_corner + &self.horizontal * s + &self.vertical * t - &self.origin - &offset,
        )
    }
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
