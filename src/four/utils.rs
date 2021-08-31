use nalgebra::{vector, Vector3};
use rand::Rng;

pub type Vec3 = Vector3<f32>;
pub fn random_in_unit_sphere() -> Vec3 {
  let mut rng = rand::thread_rng();
  loop {
    let p: Vec3 = vector![
      rng.gen_range(-1.0..1.0),
      rng.gen_range(-1.0..1.0),
      rng.gen_range(-1.0..1.0)
    ];
    if p.magnitude_squared() < 1.0 {
      return p;
    }
  }
}
