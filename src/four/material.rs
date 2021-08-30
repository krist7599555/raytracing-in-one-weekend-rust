use nalgebra::vector;

use super::{Ray, Vec3, mesh::HitRecord, utils::random_in_unit_sphere};

pub trait Material {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<MaterialScatter>;
}
pub struct MaterialScatter {
  pub attenuation: Vec3,
  pub scattered: Ray,
}

#[derive(Clone, Debug)]
pub struct Lambertian {
  pub albedo: Vec3
}
impl Material for Lambertian {
  fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<MaterialScatter> {
      let target = rec.position + rec.normal + random_in_unit_sphere();
      return Some(MaterialScatter {
          scattered: Ray { origin: rec.position, direction: target - rec.position },
          attenuation: self.albedo.to_owned(),
      });
  }
}
impl Default for Lambertian {
  fn default() -> Self {
      Self { albedo: vector![0.0, 1.0, 0.5] }
  }
}
impl Lambertian {
  pub fn _new(albedo: Vec3) -> Self {
      Self {
          albedo,
      }
  }
}

#[derive(Clone, Debug)]
pub struct Metal {
  pub albedo: Vec3,
  pub fuzz: f32,
}
impl Default for Metal {
  fn default() -> Self {
      Self { albedo: vector![0.0, 1.0, 0.5], fuzz: 1.0 }
  }
}
impl Metal {
  pub fn _new(albedo: Vec3, fuzz: f32) -> Self {
      Self {
          albedo,
          fuzz: fuzz.min(1.0)
      }
  }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
  return v - 2.0 * v.dot(n) * n;
}

impl Material for Metal {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<MaterialScatter> {
      // let target = rec.position + rec.normal + random_in_unit_sphere();
      let reflected = reflect(&r_in.direction.normalize(), &rec.normal);
      let scattered = Ray { origin: rec.position, direction: reflected + self.fuzz * random_in_unit_sphere() };
      let attenuation = self.albedo.to_owned();
      if scattered.direction.dot(&rec.normal) > 0.0 {
          Some(MaterialScatter {
              scattered,
              attenuation
          })
      } else {
          None
      }
  }
}