use crate::{camera::Camera, hittable::Hittable};

pub struct Scene {
    pub camera: Camera,
    pub world: Box<dyn Hittable + Send + Sync>,
}

impl Scene {
    pub fn new(camera: Camera, world: Box<dyn Hittable + Send + Sync>) -> Scene {
        Scene { camera, world }
    }
}