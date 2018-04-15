extern crate rand;
extern crate tracer;

use tracer::{unit_vector, Hit, HitList, HitRecord, Ray, Sphere, RAND_END};
use tracer::vec::Vec3;
use tracer::materials::*;
use tracer::camera::*;
use std::f32;
use rand::{thread_rng, Rng};

fn color<T: Hit>(ray: &Ray, world_hit_record: &T, depth: i32) -> Vec3 {
    let mut hit_record = HitRecord::new();
    if world_hit_record.hit(ray, 0.001, f32::MAX, &mut hit_record) {
        let mut scattered = Ray::new(Vec3::new(0f32, 0f32, 0f32), Vec3::new(0f32, 0f32, 0f32));
        let mut attenuation = Vec3::new(0f32, 0f32, 0f32);
        if depth < 50
            && hit_record.mat.clone().unwrap().scatter(
                ray,
                &hit_record,
                &mut attenuation,
                &mut scattered,
            ) {
            return attenuation * color::<T>(&scattered, world_hit_record, depth + 1);
        } else {
            return Vec3::new(0f32, 0f32, 0f32);
        }
    } else {
        let unit_direction = unit_vector(&ray.direction());
        let t: f32 = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    }
}

fn main() {
    let nx: i32 = 2000;
    let ny: i32 = 1000;
    let ns: i32 = 100;
    let mut rng = thread_rng();

    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;

    let camera = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        (nx / ny) as f32,
        0.1,
        dist_to_focus,
    );

    let mut hit_list = HitList::new(0);
    for i in (-11..11).rev() {
        for j in (-11..11).rev() {
            let mat_choice = rng.gen_range::<f32>(0.0, RAND_END);
            let center = Vec3::new(
                i as f32 + 0.9 + rng.gen_range::<f32>(0.0, RAND_END),
                0.2,
                j as f32 + 0.9 + rng.gen_range::<f32>(0.0, RAND_END),
            );
            if (center.clone() - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if mat_choice < 0.8 {
                    hit_list.list.push(Sphere::new(
                        center.clone(),
                        0.2,
                        Box::new(Lambertian::new(Vec3::new(
                            rng.gen_range::<f32>(0.0, RAND_END)
                                * rng.gen_range::<f32>(0.0, RAND_END),
                            rng.gen_range::<f32>(0.0, RAND_END)
                                * rng.gen_range::<f32>(0.0, RAND_END),
                            rng.gen_range::<f32>(0.0, RAND_END)
                                * rng.gen_range::<f32>(0.0, RAND_END),
                        ))),
                    ));
                } else if mat_choice < 0.95 {
                    hit_list.list.push(Sphere::new(
                        center.clone(),
                        0.2,
                        Box::new(Metal::new(
                            Vec3::new(
                                0.5 * (1.0 + rng.gen_range::<f32>(0.0, RAND_END)),
                                0.5 * (1.0 + rng.gen_range::<f32>(0.0, RAND_END)),
                                0.5 * (1.0 + rng.gen_range::<f32>(0.0, RAND_END)),
                            ),
                            0.5 * rng.gen_range::<f32>(0.0, RAND_END),
                        )),
                    ));
                } else {
                    hit_list.list.push(Sphere::new(
                        center.clone(),
                        0.2,
                        Box::new(Dielectric::new(1.5)),
                    ));
                }
            }
        }
    }
    hit_list.list.push(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
    ));
    hit_list.list.push(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Box::new(Dielectric::new(1.5)),
    ));
    hit_list.list.push(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
    ));
    hit_list.list.push(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    ));

    println!("P3\n{} {}\n255\n", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0f32, 0f32, 0f32);
            for _ in 0..ns {
                let u: f32 = (i as f32 + rng.gen_range::<f32>(0.0, RAND_END)) / nx as f32;
                let v: f32 = (j as f32 + rng.gen_range::<f32>(0.0, RAND_END)) / ny as f32;
                let ray = camera.ray(u, v);
                col += color(&ray, &hit_list, 0);
            }
            col /= ns as f32;
            let ir: i32 = (255.99 * col.r().sqrt()) as i32;
            let ig: i32 = (255.99 * col.g().sqrt()) as i32;
            let ib: i32 = (255.99 * col.b().sqrt()) as i32;
            println!("{} {} {}\n", ir, ig, ib);
        }
    }
}
