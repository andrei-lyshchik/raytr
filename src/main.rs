use rand::{random, thread_rng, Rng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use raytr::{
    camera::Camera,
    hittable::{Hittable, HittableList},
    material::{Dielectric, Lambertian, Metal, Material},
    ray::Ray,
    scene::Scene,
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
    if let Some(hit) = world.hit(ray, 0.0001, f64::INFINITY) {
        if let Some(scatter) = hit.material().scatter(ray, &hit) {
            return scatter.attenuation() * ray_color(scatter.ray(), world, depth - 1);
        }
        return Vec3::new(0.0, 0.0, 0.0);
    }
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
}

fn initial_scene() -> Scene {
    let camera = Camera::new(
        Vec3::new(-2.0, 2.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        50.0,
        16.0 / 9.0,
        0.1,
        (Vec3::new(-2.0, 2.0, 1.0) - Vec3::new(0.0, 0.0, -1.0)).length(),
    );
    let material_ground = Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_center = Box::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let material_left = Box::new(Dielectric::new(2.4));
    let material_right = Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0));

    let world = HittableList::new(vec![
        Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, 0.0),
            100.0,
            material_ground,
        )),
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center)),
        Box::new(Sphere::new(Vec3::new(-0.7, -0.3, -1.0), 0.2, material_left)),
        Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right)),
    ]);
    Scene::new(camera, Box::new(world))
}

fn random_material() -> Box<dyn Material + Send + Sync> {
    let random: f64 = random();

    if random < 0.8 {
        let albedo = &Vec3::random(0.0, 1.0) * Vec3::random(0.0, 1.0);
        Box::new(Lambertian::new(albedo))
    } else if random < 0.95 {
        let albedo = Vec3::random(0.5, 1.0);
        let fuzz = thread_rng().gen_range(0.0..0.5);
        Box::new(Metal::new(albedo, fuzz))
    } else {
        Box::new(Dielectric::new(1.5))
    }
}

fn random_world() -> Box<dyn Hittable + Send + Sync> {
    let mut objects: Vec<Box<dyn Hittable + Send + Sync>> = Vec::new();

    let ground_material = Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    objects.push(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new((a as f64) + 0.9 * random::<f64>(), 0.2, (b as f64) + 0.9 * random::<f64>());
            let material = random_material();
            objects.push(Box::new(Sphere::new(center, 0.2, material)));
        }
    }

    let material_1 = Box::new(Dielectric::new(1.5));
    objects.push(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material_1)));

    let material_2 = Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    objects.push(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material_2)));

    let material_3 = Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    objects.push(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material_3)));

    Box::new(HittableList::new(objects))
}

fn random_scene() -> Scene {
    let aspect_ratio = 3.0 / 2.0;
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let focus_distance = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture, focus_distance);

    Scene::new(camera, random_world())
}

fn main() {
    let scene = random_scene();

    let image_width = 1024;
    let image_height = ((image_width as f64) / &scene.camera.aspect_ratio) as i32;
    let samples_per_pixel = 400;
    let max_depth = 50;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let pixel_color = (0..samples_per_pixel).into_par_iter()
                .map(|_| {
                    let s = ((i as f64) + random::<f64>()) / (image_width as f64 - 1.0);
                    let t = ((j as f64) + random::<f64>()) / (image_height as f64 - 1.0);
                    let ray = scene.camera.ray(s, t);
                    ray_color(&ray, scene.world.as_ref(), max_depth)
                })
                .reduce(|| Vec3::new(0.0, 0.0, 0.0), |a, b| a + b);
            write_color(&pixel_color, samples_per_pixel);
        }
    }
}
