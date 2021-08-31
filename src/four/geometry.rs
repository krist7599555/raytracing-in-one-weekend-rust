use nalgebra::Matrix;

use super::{Ray, Vec3};

pub trait Geometry {
  fn hit(&self, ray: &Ray) -> Option<HitRecordGeometric>;
}

pub struct HitRecordGeometric {
  pub t: f32,
  pub position: Vec3,
  pub normal: Vec3,
}

pub struct Sphere {
  pub center: Vec3,
  pub radius: f32
}
impl Sphere {
  pub fn new(center: Vec3, radius: f32) -> Self {
    return Self {
      center, radius
    }
  }
}
impl Geometry for Sphere {
  /// `(x-cx)² + (y-cy)² + (z-cz)² = r²` (sphere equation)<br>
  /// so define 
  /// ```rust 
  /// fn dotself(a) = dot(a, a) = a.x² + a.y² + a.z²
  /// ```
  /// ```rust
  /// hit sphere if forsome(t)
  ///     where 
  ///       ray.origin = A
  ///       ray.direction = B
  ///       ray(t) = A + t * B
  ///       sphere.center = C
  ///   = dotself(ray(t) - C) = r²
  ///   = dotself(A + t * B + t * ray.direction - C) = r²
  ///   = dotself((t * B) + (A - C)) = r²
  ///   = (t² * dotself(B)) 
  ///     + (2 * t * dot(B, A - C)) 
  ///     + (dotself(A - C) - (r²)) = 0
  /// ```
  /// just solve quadratic equation `(-b +/- sqrt(b² - 4ac)) / 2a`
  /// we need to check just is answer exists. so, check if `(b² - 4ac)` is positive
  ///
  /// also optimize, we only need normal vector not care about scalar
  /// ```rust
  /// if B = 2b
  ///  = -B +/- sqrt(B² - 4ac) / 2a
  ///  = -2b +/- 2sqrt(b²-ac) / 2a
  ///  = 2 (b +/- sqrt(b²-ac) / a)
  ///    ^remove 2
  /// ```
  #[allow(non_snake_case)]
  fn hit(&self, ray: &Ray) -> Option<HitRecordGeometric> {
      let (A, B, C, r): _ = (&ray.origin, &ray.direction, &self.center, &self.radius);
      let a = B.dot(B);
      let b = B.dot(&(A-C));
      let c = (A-C).dot(&(A-C)) - r.powi(2);
      let discriminant = b.powi(2) - a * c;
      if discriminant < 0.0 {
          return None
      } else {
          let t_pos = (-b - discriminant.sqrt()) / (a);
          let t_neg = (-b + discriminant.sqrt()) / (a);
          for t in vec![t_pos, t_neg] {
            if t < f32::INFINITY && t > 0.001 {
              let hit_surface = ray.point_at_parameter(t);
              return Some(HitRecordGeometric {
                  t: t,
                  position: hit_surface,
                  normal: (hit_surface - C).normalize(),
              })
            }
          }
          None
      }
  }
}