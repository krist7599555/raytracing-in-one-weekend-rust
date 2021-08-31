use nalgebra::vector;

use super::mesh::HitRecord;
use super::utils::random_in_unit_sphere;
use super::{Ray, Vec3};

pub trait Material {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<MaterialScatter>;
}

impl core::fmt::Debug for dyn Material {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "Material{{}}")
  }
}
pub struct MaterialScatter {
  pub attenuation: Vec3,
  pub scattered:   Ray,
}

#[derive(Clone, Debug)]
pub struct Lambertian {
  pub albedo: Vec3,
}
impl Material for Lambertian {
  fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<MaterialScatter> {
    let target = rec.position + rec.normal + random_in_unit_sphere();
    return Some(MaterialScatter {
      scattered:   Ray {
        origin:    rec.position,
        direction: target - rec.position,
      },
      attenuation: self.albedo.to_owned(),
    });
  }
}
impl Default for Lambertian {
  fn default() -> Self {
    Self {
      albedo: vector![0.0, 1.0, 0.5],
    }
  }
}
impl Lambertian {
  pub fn new(albedo: Vec3) -> Self { Self { albedo } }
}

#[derive(Clone, Debug)]
pub struct Metal {
  pub albedo: Vec3,
  pub fuzz:   f32,
}
impl Default for Metal {
  fn default() -> Self {
    Self {
      albedo: vector![0.0, 1.0, 0.5],
      fuzz:   1.0,
    }
  }
}
impl Metal {
  pub fn new(albedo: Vec3, fuzz: f32) -> Self {
    Self {
      albedo,
      fuzz: fuzz.min(1.0),
    }
  }

  pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 { return v - 2.0 * v.dot(n) * n; }
}

impl Material for Metal {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<MaterialScatter> {
    // let target = rec.position + rec.normal + random_in_unit_sphere();
    let reflected = Self::reflect(&r_in.direction.normalize(), &rec.normal);
    let scattered = Ray {
      origin:    rec.position,
      direction: reflected + self.fuzz * random_in_unit_sphere(),
    };
    let attenuation = self.albedo.to_owned();
    if scattered.direction.dot(&rec.normal) > 0.0 {
      Some(MaterialScatter {
        scattered,
        attenuation,
      })
    } else {
      None
    }
  }
}

/// Dielectric is reflect/flip accordenly to snell's law
/// `n₁ * sin(θ₁) = n₂ * sin(θ₂)`
/// like water / glass / air which some time reflect some time see through
pub struct Dielectric {
  ref_idx: f32,
}
impl Dielectric {
  pub fn new(ref_idx: f32) -> Self { Self { ref_idx } }

  /// return reflacted over snell's law
  pub fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.normalize();
    let dt = uv.dot(n);
    let discriminant = 1. - ni_over_nt * ni_over_nt * (1. - dt * dt);
    if discriminant > 0.0 {
      Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt())
    } else {
      None
    }
  }
}
impl Material for Dielectric {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<MaterialScatter> {
    let (outward_normal, ni_over_nt, cosine) = if r_in.direction.dot(&rec.normal) > 0.0 {
      (
        -rec.normal,
        self.ref_idx,
        self.ref_idx * r_in.direction.dot(&rec.normal) / r_in.direction.magnitude(),
      )
    } else {
      (
        rec.normal,
        1.0 / self.ref_idx,
        -r_in.direction.dot(&rec.normal) / r_in.direction.magnitude(),
      )
    };

    fn schlick(cosine: f32, ref_idx: f32) -> f32 {
      let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
      return r0 + (1. - r0) * (1. - cosine).powi(5);
    }

    let reflected = Metal::reflect(&r_in.direction, &rec.normal);
    let direction = match Self::refract(&r_in.direction, &outward_normal, ni_over_nt) {
      None => reflected,
      Some(refracted) => {
        let reflect_prob = schlick(cosine, self.ref_idx);
        if rand::random::<f32>() < reflect_prob {
          reflected
        } else {
          refracted
        }
      }
    };
    return Some(MaterialScatter {
      attenuation: vector![1.0, 1.0, 1.0], /* pure glass reflect all color, you can try to
                                            * change [0.3, 0.3, 1.0] for blue glass */
      scattered:   Ray {
        origin:    rec.position,
        direction: direction,
      },
    });
  }
}
