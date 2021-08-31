mod four;
use std::f32::consts::PI;
use std::fs::File;
use std::io::Write;

use four::{Geometry, Material, MaterialScatter, Mesh, Ray, Rayable, Vec3, random_in_unit_sphere};
use nalgebra::vector;
use rand::Rng;

use crate::four::{Camera, Dielectric, Lambertian, Metal, Sphere};

fn color(ray: &Ray, meshs: &Vec<&dyn Rayable>, depth: u32) -> Vec3 {
  // if let Some(hit) = ray.hit(&sphere) {
  if let Some(hit) = ray.hits(&mut meshs.iter().map(|i| *i)) {
    if depth < 10 {
      if let Some(MaterialScatter {
        attenuation,
        scattered,
      }) = hit.material.scatter(ray, &hit)
      {
        let a: Vec3 = attenuation;
        let b: Vec3 = color(&scattered, meshs, depth + 1);
        return a.zip_map(&b, |a, b| a * b);
      }
    }
    let target = hit.position + hit.normal + random_in_unit_sphere() * 0.5;
    let recursive_color = 0.5
      * color(
        &Ray {
          origin:    hit.position,
          direction: target - hit.position,
        },
        &meshs,
        depth + 1,
      );
    return recursive_color;
  }
  let t = 0.3 * (ray.direction.normalize().y + 1.0);
  let white_color = vector![1.0, 1.0, 1.0];
  let blue_color = vector![0.5, 0.7, 1.0];
  return white_color.lerp(&blue_color, t); // Linear interpolation
}

fn main() {
  let mut w = File::create("image.ppm").unwrap();
  let prod = true;

  let nx = 200 * (if prod { 5 } else { 1 });
  let ny = 100 * (if prod { 5 } else { 1 });

  writeln!(&mut w, "P3").unwrap();
  writeln!(&mut w, "{} {}", nx, ny).unwrap();
  writeln!(&mut w, "{}", 255).unwrap();

  let lookfrom = vector![3.0, -0.1, 0.2];
  let lookat = vector![0.0, 0.0, -1.0];
  let camera = Camera::new(
    &lookfrom,
    &lookat,
    &vector![0.0, 1.0, 0.0],
    30.0,
    nx as f32 / ny as f32,
    0.0,
    (lookfrom - lookat).magnitude(),
  );

  let sphere = Mesh::new(
    Sphere::new(vector![0.0, 0.0, -1.0], 0.5),
    Lambertian::new(vector![0.1, 0.2, 0.5]),
  );
  let sphere2 = Mesh::new(
    Sphere::new(vector![0.0, -100.5, -1.0], 100.0),
    Lambertian::new(vector![0.5, 0.3, 0.8]),
  );
  let sphere3 = Mesh::new(
    Sphere::new(vector![1.0, 0.0, -1.0], 0.5),
    Metal::new(vector![0.8, 0.6, 0.2], 0.0),
  );
  let sphere4 = Mesh::new(
    Sphere::new(vector![-1.0, 0.0, -1.0], 0.5),
    Dielectric::new(1.5),
  );
  let sphere5 = Mesh::new(
    Sphere::new(vector![-1.0, 0.0, -1.0], 0.35),
    Dielectric::new(1.0 / 1.5),
  );

  let r = (PI / 4.0).cos();
  let blue_ball = Mesh::new(
    Sphere::new(vector![-r, 0., -1.], r),
    Lambertian::new(vector![0., 0., 1.]),
  );
  let red_ball = Mesh::new(
    Sphere::new(vector![r, 0., -1.], r),
    Lambertian::new(vector![1., 0., 0.]),
  );

  let mut meshs: Vec<Box<dyn Rayable>> = vec![
    Box::new(sphere),
    Box::new(sphere2),
    Box::new(sphere3),
    Box::new(sphere4),
    Box::new(sphere5),
  ];

  let mut rnd = rand::thread_rng();
  for _ in 0..30 {
    let r = 0.15;
    let mesh = Mesh::new(
      Sphere::new(vector![rnd.gen_range(-3.0..3.0), -0.5 + r, rnd.gen_range(-3.0..3.0)], r),
      Lambertian::new(vector![rnd.gen_range(0.0..1.0), rnd.gen_range(0.0..1.0), rnd.gen_range(0.0..1.0)]),
    );
    meshs.push(Box::new(mesh))
  }

  let profile_time = std::time::Instant::now();
  for v in (0..ny).map(|i| i as f32 / ny as f32).rev() {
    println!("process = {:.2}, time = {:.2}s", 1.0 - v, profile_time.elapsed().as_secs());
    for u in (0..nx).map(|i| i as f32 / nx as f32) {
      let num_sample = 1 * (if prod { 10 } else { 1 });
      let mut rng = rand::thread_rng();
      let average_color = (0..num_sample)
        .map(|_| {
          let u = u + (rng.gen_range(0.0..1.0) / (nx as f32));
          let v = v + (rng.gen_range(0.0..1.0) / (ny as f32));
          let meshs = meshs
            .iter()
            .map(|i| &**i)
            .collect::<Vec<_>>();
          color(&camera.get_ray(u, v), &meshs, 0)
        })
        .sum::<Vec3>()
        / (num_sample as f32);
      let gamma = 2;
      let rgb: Vec3 = average_color
        .map(|f| f.powf(1.0 / gamma as f32))
        .map(|f| (f * 255.99).floor());
      writeln!(&mut w, "{:.0} {:.0} {:.0}", rgb.x, rgb.y, rgb.z).unwrap();
    }
  }
  println!(
    "time use per pixel = {:?}",
    profile_time.elapsed() / (ny * nx)
  );
  println!("total time = {:?}", profile_time.elapsed());
}
