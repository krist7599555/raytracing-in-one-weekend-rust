use nalgebra::vector;

use super::{ Vec3, Ray };

pub struct Camera {
  pub lower_left_coner: Vec3,
  pub horizontal: Vec3,
  pub vertical: Vec3,
  pub origin: Vec3,
}
impl Camera {
  pub fn get_ray(&self, u: f32, v: f32) -> Ray {
      return Ray {
          origin: self.origin,
          direction: self.lower_left_coner + u * self.horizontal + v * self.vertical
      }
  }
}
impl Default for Camera {
  fn default() -> Self {
      Self { 
          lower_left_coner: vector![-2.0, -1.0, -1.0],
          horizontal: vector![4.0, 0.0, 0.0],
          vertical: vector![0.0, 2.0, 0.0],
          origin: vector![0.0, 0.0, 0.0], 
      }
  }
}