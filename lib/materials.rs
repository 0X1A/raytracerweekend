extern crate rand;

use super::{dot, rand_in_unit_sphere, reflect, refract, shlick, unit_vector, HitRecord,
            Ray, Vec3};
use rand::{thread_rng, Rng};

pub trait Material: MaterialClone {
    fn scatter(
        &self,
        rays: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

pub trait MaterialClone {
    fn clone_box(&self) -> Box<Material>;
}

impl<T> MaterialClone for T
where
    T: 'static + Material + Clone,
{
    fn clone_box(&self) -> Box<Material> {
        Box::new(self.clone())
    }
}

impl Clone for Box<Material> {
    fn clone(&self) -> Box<Material> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(vec: Vec3) -> Lambertian {
        Lambertian { albedo: vec }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let target = &hit_record.p + &hit_record.normal + rand_in_unit_sphere();
        *scattered = Ray::new(hit_record.p.clone(), target - hit_record.p.clone());
        *attenuation = self.albedo.clone();
        return true;
    }
}

#[derive(Clone)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(vec: Vec3, fuzz: f32) -> Metal {
        Metal {
            albedo: vec,
            fuzz: fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflect = reflect(&unit_vector(&ray.direction()), &hit_record.normal);
        *scattered = Ray::new(
            hit_record.p.clone(),
            reflect + self.fuzz * rand_in_unit_sphere(),
        );
        *attenuation = self.albedo.clone();
        dot(&scattered.direction(), &hit_record.normal) > 0.0
    }
}

#[derive(Clone)]
pub struct Dielectric {
    index: f32,
}

impl Dielectric {
    pub fn new(idx: f32) -> Dielectric {
        Dielectric { index: idx }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut rng = thread_rng();
        let outward_normal;
        let reflected = reflect(&ray.direction(), &hit_record.normal);
        let ni_nt;
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let mut refracted = Vec3::default();
        let reflect_prob;
        let cos;
        if dot(&ray.direction(), &hit_record.normal) > 0.0 {
            outward_normal = &hit_record.normal * -1.0;
            ni_nt = self.index;
            cos = self.index + dot(&ray.direction(), &hit_record.normal) / ray.direction().length();
        } else {
            outward_normal = hit_record.normal.clone();
            ni_nt = 1.0 / self.index;
            cos = -dot(&ray.direction(), &hit_record.normal) / ray.direction().length();
        }
        if refract(&ray.direction(), &outward_normal, ni_nt, &mut refracted) {
            reflect_prob = shlick(cos, self.index);
        } else {
            *scattered = Ray::new(hit_record.p.clone(), reflected.clone());
            reflect_prob = 1.0;
        }
        if rng.gen_range::<f32>(0.0, 0.99999) < reflect_prob {
            *scattered = Ray::new(hit_record.p.clone(), reflected);
        } else {
            *scattered = Ray::new(hit_record.p.clone(), refracted);
        }
        true
    }
}
