extern crate rand;
pub mod vec;
pub mod materials;
pub mod camera;

use vec::*;
use materials::*;
use rand::{thread_rng, Rng};

pub const RAND_END: f32 = 0.99999;

pub fn refract(v: &Vec3, n: &Vec3, ni_nt: f32, refracted: &mut Vec3) -> bool {
    let uv = unit_vector(v);
    let dt = dot(&uv, n);
    let discriminant = 1.0 - (ni_nt * ni_nt) * (1.0 - (dt * dt));
    if discriminant > 0.0 {
        *refracted = ni_nt * (uv - n * dt) - n * discriminant.sqrt();
        true
    } else {
        false
    }
}

#[inline(always)]
pub fn unit_vector(vec: &Vec3) -> Vec3 {
    vec / vec.length()
}

#[inline(always)]
pub fn dot(rhs: &Vec3, lhs: &Vec3) -> f32 {
    (rhs.x * lhs.x) + (rhs.y * lhs.y) + (rhs.z * lhs.z)
}

#[inline(always)]
pub fn cross(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
    Vec3 {
        x: (lhs.y * rhs.z - lhs.z * rhs.y),
        y: -(lhs.x * rhs.z - lhs.z * rhs.x),
        z: (lhs.x * rhs.y - lhs.y * rhs.x),
    }
}

pub fn rand_in_unit_sphere() -> Vec3 {
    let mut rng = thread_rng();
    let mut p;
    loop {
        p = 2.0
            * Vec3::new(
                rng.gen_range::<f32>(0.0, RAND_END),
                rng.gen_range::<f32>(0.0, RAND_END),
                rng.gen_range::<f32>(0.0, RAND_END),
            ) - Vec3::new(1.0, 1.0, 1.0);
        if !(p.squared_length() >= 1.0) {
            break;
        }
    }
    p
}

#[inline(always)]
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    return v - &(2.0 * dot(v, n) * n);
}

#[inline(always)]
pub fn shlick(cos: f32, idx: f32) -> f32 {
    let mut r0 = (1.0 - idx) / (1.0 + idx);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cos).powf(5.0);
}

#[derive(Debug, Clone)]
pub struct Ray {
    a: Vec3,
    b: Vec3,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Ray {
        Ray { a: a, b: b }
    }

    #[inline(always)]
    pub fn origin(&self) -> Vec3 {
        self.a.clone()
    }
    #[inline(always)]
    pub fn direction(&self) -> Vec3 {
        self.b.clone()
    }
    #[inline(always)]
    pub fn point_at_param(&self, t: f32) -> Vec3 {
        &self.a + &(t * &self.b)
    }
}

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: Option<Box<Material>>,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            t: 0.0,
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            mat: None,
        }
    }
}

pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool;
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Box<Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, mat: Box<Material>) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
            material: mat,
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let oc = &ray.origin() - &self.center;
        let a = dot(&ray.direction(), &ray.direction());
        let b = dot(&oc, &ray.direction());
        let c = dot(&oc, &oc) - (&self.radius * &self.radius);
        let discriminant = (b * b) - (a * c);
        let mut temp = (-b - discriminant.sqrt()) / a;
        if discriminant > 0.0 {
            if temp < t_max && temp > t_min {
                hit_record.t = temp;
                hit_record.p = ray.point_at_param(hit_record.t);
                hit_record.normal = (&hit_record.p - &self.center) / self.radius;
                hit_record.mat = Some(self.material.clone_box());
                return true;
            }
            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                hit_record.t = temp;
                hit_record.p = ray.point_at_param(hit_record.t);
                hit_record.normal = (&hit_record.p - &self.center) / self.radius;
                hit_record.mat = Some(self.material.clone_box());
                return true;
            }
        }
        return false;
    }
}

pub struct HitList<T: Hit> {
    pub list: Vec<T>,
}

impl<T: Hit> HitList<T> {
    pub fn new(size: usize) -> HitList<T> {
        HitList {
            list: Vec::with_capacity(size),
        }
    }
}

impl<T: Hit> Hit for HitList<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest = t_max;
        for item in self.list.iter() {
            let result = item.hit(ray, t_min, closest, hit_record);
            if result {
                hit_anything = true;
                closest = hit_record.t;
            }
        }
        hit_anything
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vec3_vec3_div() {
        let rhs = Vec3::new(4f32, 1f32, 1f32);
        let lhs = Vec3::new(2f32, 1f32, 1f32);
        let new = &rhs / &lhs;
        assert_eq!(new, Vec3::new(2f32, 1f32, 1f32));
    }

    #[test]
    fn vec3_vec3_mul() {
        let rhs = Vec3::new(4f32, 1f32, 1f32);
        let lhs = Vec3::new(2f32, 1f32, 1f32);
        let new = &rhs * &lhs;
        assert_eq!(new, Vec3::new(8f32, 1f32, 1f32));
    }
}
