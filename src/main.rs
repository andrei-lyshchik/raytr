use raytr::{vec3::Vec3, ray::Ray};

fn write_color(color: &Vec3) {
    let (r, g, b) = ((color.x * 255.999) as i32, (color.y * 255.999) as i32, (color.z * 255.999) as i32);
    println!("{} {} {}", r, g, b);
}

fn hit_sphere(center: &Vec3, radius: f64, ray: &Ray) -> Option<f64> {
    let oc = &ray.origin - center;
    let a = ray.direction.dot(&ray.direction);
    let half_b = ray.direction.dot(&oc);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = half_b * half_b - a * c;
    
    if discriminant < 0.0 {
        None
    } else {
        Some((-half_b - discriminant.sqrt()) / a)
    }
}

fn ray_color(ray: &Ray) -> Vec3 {
    if let Some(t) = hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, &ray) {
        let n = (ray.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
        return Vec3::new(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5;
    }
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    return Vec3::new(0.5, 0.7, 1.0) * (1.0 - t) + Vec3::new(1.0, 1.0, 1.0) * t
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1000;
    let image_height = ((image_width as f64) / aspect_ratio) as i32;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        &origin - &horizontal / 2.0 - &vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in 0 .. image_height {
        for i in 0 .. image_width {
            let u = (i as f64) / (image_width as f64 - 1.0);
            let v = (j as f64) / (image_height as f64 - 1.0);
            let ray = Ray::new(&origin, &(&lower_left_corner + &horizontal * u + &vertical * v));
            write_color(&ray_color(&ray));
        }
    }
}
