mod four;
use std::{fs::File};
use std::io::Write;
use four::{Mesh, Ray, Vec3};
use nalgebra::{vector};
use rand::Rng;

use four::{MaterialScatter, random_in_unit_sphere};

use crate::four::{Camera, Dielectric, Lambertian, Metal, Sphere};


fn color(ray: &Ray, meshs: &Vec<Box<&Mesh>>, depth: u32) -> Vec3 {
    // if let Some(hit) = ray.hit(&sphere) {
    if let Some(hit) = ray.hits(&mut meshs.iter()) {
        if depth < 50 {
            if let Some(MaterialScatter { attenuation , scattered }) = hit.material.scatter(ray, &hit) {
                let a: Vec3 = attenuation;
                let b: Vec3 = color(&scattered, meshs, depth + 1);
                return a.zip_map(&b, |a, b| a * b)
            }
        }
        let target = hit.position + hit.normal + random_in_unit_sphere() * 0.5;
        let recursive_color = 0.5 * color(&Ray {
            origin: hit.position,
            direction: target - hit.position
        }, &meshs, depth + 1);
        return recursive_color
    }
    let t = 0.3 * (ray.direction.normalize().y + 1.0);
    let white_color = vector![1.0, 1.0, 1.0];
    let blue_color = vector![0.5, 0.7, 1.0];
    return white_color.lerp(&blue_color, t); // Linear interpolation
}

fn main() {
    let mut w = File::create("image.ppm").unwrap();

    let nx = 200;
    let ny = 100;

    writeln!(&mut w, "P3").unwrap();
    writeln!(&mut w, "{} {}", nx, ny).unwrap();
    writeln!(&mut w, "{}", 255).unwrap();

    let camera = Camera::default();

    let sphere  = Mesh { geometry: &Sphere { center: vector![0.0, 0.0, -1.0], radius: 0.5 }, material: &Lambertian::new(vector![0.1, 0.2, 0.5]) };
    let sphere2 = Mesh { geometry: &Sphere { center: vector![0.0, -100.5, -1.0], radius: 100.0 }, material: &Lambertian::new(vector![0.8, 0.8, 0.0]) };
    let sphere3 = Mesh { geometry: &Sphere { center: vector![1.0, 0.0, -1.0], radius: 0.5 }, material: &Metal::new(vector![0.8, 0.6, 0.2], 0.0) };
    let sphere4 = Mesh { geometry: &Sphere { center: vector![-1.0, 0.0, -1.0], radius: 0.5 }, material: &Dielectric::new(1.5) };
    let sphere5 = Mesh { geometry: &Sphere { center: vector![-1.0, 0.0, -1.0], radius: 0.45 }, material: &Dielectric::new(1.0 / 1.5) };

    let meshs: Vec<Box<&Mesh>> = vec![
        Box::new(&sphere),
        Box::new(&sphere2),
        Box::new(&sphere3),
        Box::new(&sphere4),
        Box::new(&sphere5),
    ];

    for v in (0..ny).map(|i| i as f32 / ny as f32).rev() {
    for u in (0..nx).map(|i| i as f32 / nx as f32) {
        let num_sample = 6;
        let mut rng = rand::thread_rng();
        let average_color = (0..num_sample).map(|_| {
            let u = u + (rng.gen_range(0.0..1.0) / (nx as f32));
            let v = v + (rng.gen_range(0.0..1.0) / (ny as f32));
            color(&camera.get_ray(u, v), &meshs, 0)
        }).sum::<Vec3>() / (num_sample as f32);
        let gamma = 2;
        let rgb: Vec3 = average_color
            .map(|f| f.powf(1.0 / gamma as f32))
            .map(|f| (f * 255.99).floor());
        writeln!(&mut w, "{:.0} {:.0} {:.0}", rgb.x, rgb.y, rgb.z).unwrap();
    }}
}
