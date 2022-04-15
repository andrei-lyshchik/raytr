use rand::random;
use raytr::{
    camera::Camera,
    hittable::{Hittable, HittableList},
    material::{Lambertian, Metal},
    ray::Ray,
    sphere::Sphere,
    vec3::Vec3,
};

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}

fn write_color(color: &Vec3, samples_per_pixel: i32) {
    let scale = 1.0 / (samples_per_pixel as f64);
    let (r, g, b) = (
        (color.x * scale).sqrt(),
        (color.y * scale).sqrt(),
        (color.z * scale).sqrt(),
    );
    let (r_int, g_int, b_int) = (
        (256.0 * clamp(r, 0.0, 0.999)) as i32,
        (256.0 * clamp(g, 0.0, 0.999)) as i32,
        (256.0 * clamp(b, 0.0, 0.999)) as i32,
    );
    println!("{} {} {}", r_int, g_int, b_int);
}

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    if let Some(hit) = world.hit(ray, 0.0, f64::INFINITY) {
        if let Some(scatter) = hit.material().scatter(ray, &hit) {
            return scatter.attenuation() * ray_color(scatter.ray(), world, depth - 1);
        }
        return Vec3::new(0.0, 0.0, 0.0);
    }
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
}

fn main() {
    let camera = Camera::new();
    let image_width = 1024;
    let image_height = ((image_width as f64) / camera.aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let material_ground = Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_center = Box::new(Lambertian::new(Vec3::new(0.7, 0.3, 0.3)));
    let material_left = Box::new(Metal::new(Vec3::new(0.8, 0.8, 0.8)));
    let material_right = Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2)));

    let world = HittableList::new(vec![
        Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, 0.0),
            100.0,
            material_ground,
        )),
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center)),
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left)),
        Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right)),
    ]);

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                let u = ((i as f64) + random::<f64>()) / (image_width as f64 - 1.0);
                let v = ((j as f64) + random::<f64>()) / (image_height as f64 - 1.0);
                let ray = camera.ray(u, v);
                pixel_color += ray_color(&ray, &world, max_depth);
            }
            write_color(&pixel_color, samples_per_pixel);
        }
    }
}
