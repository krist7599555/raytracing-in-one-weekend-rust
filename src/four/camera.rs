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
  pub fn get_ray(&self, s: f32, t: f32) -> Ray {
      return Ray {
          origin: self.origin,
          direction: self.lower_left_coner + s * self.horizontal + t * self.vertical - self.origin,
      }
  }
  /// `fovy` as `width / height`<br>
  /// `asspect` as `width / height`
  pub fn new(lookfrom: &Vec3, lookat: &Vec3, up: &Vec3, fovy: f32, aspect: f32) -> Self {
      let theta = fovy * PI / 180.0;
      let half_height = (theta / 2.0).tan();
      let half_width = aspect * half_height;

      let w = (lookfrom - lookat).normalize();
      let u = up.cross(&w).normalize();
      let v = w.cross(&u);

      Self { 
          lower_left_coner: lookfrom - half_width * u - half_height * v - w,
          horizontal: 2.0 * half_width * u,
          vertical: 2.0 * half_height * v,
          origin: lookfrom.to_owned(), 
      }
  }
}