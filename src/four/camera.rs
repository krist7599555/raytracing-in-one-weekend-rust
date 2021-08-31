use std::f32::consts::PI;

use nalgebra::vector;

use super::{ Vec3, Ray };

pub struct Camera {
  pub lower_left_coner: Vec3,
  pub horizontal: Vec3,
  pub vertical: Vec3,
  pub origin: Vec3,

  pub u: Vec3,
  pub v: Vec3,
  pub w: Vec3,

  pub lens_radius: f32,
}
impl Camera {
  fn random_in_unit_disk() -> Vec3 {
    use rand::Rng;
    let mut rnd = rand::thread_rng();
    loop {
      let p = vector![
        rnd.gen_range(-1.0..1.0),
        rnd.gen_range(-1.0..1.0),
        0.0
      ];
      if p.dot(&p) >= 1.0 {
        return p;
      }
    }
  }
  pub fn get_ray(&self, s: f32, t: f32) -> Ray {
      let rd = self.lens_radius * Self::random_in_unit_disk();
      let offset = self.u * rd.x + self.v * rd.y;
      return Ray {
          origin: self.origin + offset,
          direction: self.lower_left_coner + s * self.horizontal + t * self.vertical - self.origin - offset,
      }
  }
  /// `fovy` as `width / height`<br>
  /// `asspect` as `width / height`
  /// `aperture` how blur is it. when `aperture=0` behavior is like pinhold (all pixel sharp)
  pub fn new(lookfrom: &Vec3, lookat: &Vec3, up: &Vec3, fovy: f32, aspect: f32, aperture: f32, focus_disk: f32) -> Self {
      let lens_radius = aperture / 2.;
      let theta = fovy * PI / 180.0;
      let half_height = (theta / 2.0).tan();
      let half_width = aspect * half_height;

      let w = (lookfrom - lookat).normalize();
      let u = up.cross(&w).normalize();
      let v = w.cross(&u);

      let fd = focus_disk;
      Self { 
          lower_left_coner: lookfrom - fd * half_width * u - fd * half_height * v - fd * w,
          horizontal: 2.0 * fd * half_width  * u,
          vertical:   2.0 * fd * half_height * v,
          origin: lookfrom.to_owned(), 
          lens_radius,
          u,
          v,
          w,
      }
  }
}