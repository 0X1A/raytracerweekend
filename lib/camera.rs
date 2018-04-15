extern crate rand;
extern crate std;

use std::f32;
use super::{cross, dot, unit_vector, Ray, Vec3, RAND_END};

use rand::{thread_rng, Rng};

#[derive(Debug, Clone)]
pub struct Camera {
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Default for Camera {
    fn default() -> Camera {
        Camera {
            lower_left: Vec3::origin(),
            horizontal: Vec3::origin(),
            vertical: Vec3::origin(),
            origin: Vec3::origin(),
            u: Vec3::origin(),
            v: Vec3::origin(),
            w: Vec3::origin(),
            lens_radius: 0.0,
        }
    }
}

fn random_unit_in_disk() -> Vec3 {
    let mut rng = thread_rng();
    let mut p;
    loop {
        p = 2.0
            * Vec3::new(
                rng.gen_range::<f32>(0.0, RAND_END),
                rng.gen_range::<f32>(0.0, RAND_END),
                0.0,
            ) - Vec3::new(1.0, 1.0, 0.0);
        if !(dot(&p, &p) >= 1.0) {
            break;
        }
    }
    p
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus: f32,
    ) -> Camera {
        let mut camera = Camera::default();
        let theta: f32 = vfov * std::f32::consts::PI / 180.0f32;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        camera.lens_radius = aperture / 2.0;
        camera.origin = look_from.clone();
        camera.w = unit_vector(&(look_from - look_at));
        camera.u = unit_vector(&cross(&vup, &camera.w));
        camera.v = cross(&camera.w, &camera.u);
        camera.lower_left = Vec3::new(-half_width, -half_height, -1.0);
        camera.lower_left = &camera.origin - &(half_width * focus * &camera.u)
            - (half_height * focus * &camera.v) - (focus * &camera.w);
        camera.horizontal = 2.0 * half_width * focus * &camera.u;
        camera.vertical = 2.0 * half_height * focus * &camera.v;
        camera
    }

    pub fn ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_unit_in_disk();
        let offset = &self.u * rd.x() + &self.v * rd.y();
        Ray::new(
            &self.origin + &offset,
            &self.lower_left + &(s * &self.horizontal) + (t * &self.vertical) - self.origin.clone()
                - offset,
        )
    }
}
