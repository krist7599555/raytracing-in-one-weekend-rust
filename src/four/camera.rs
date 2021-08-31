use std::f32::consts::PI;

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
  /// `fovy` as `width / height`<br>
  /// `asspect` as `width / height`
  pub fn new(fovy: f32, aspect: f32) -> Self {
      let theta = fovy * PI / 180.0;
      let half_height = (theta / 2.0).tan();
      let half_width = aspect * half_height;
      Self { 
          lower_left_coner: vector![-half_width, -half_height, -1.0],
          horizontal: vector![2.0 * half_width, 0.0, 0.0],
          vertical: vector![0.0, 2.0 * half_height, 0.0],
          origin: vector![0.0, 0.0, 0.0], 
      }
  }
}