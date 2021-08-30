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
impl Geometry for Sphere {
  fn hit(&self, ray: &Ray) -> Option<HitRecordGeometric> {
      // (x-cx)^2 + (y-cy)^2 + (z-cz)^2 = r^2 (sphere equation)
      // dot(a, a) = a.x^2 + a.y^2 + a.z^2
      // so define fun dotself(x) = dot(x, x)
      // hit sphere if FORSOME(t)
      //   = dotself(ray(t) - sphere.center) = r^2
      //   = dotself(ray.origin + t*ray.direction - sphere.center) = r^2
      //   = dotself((t*ray.direction) + (ray.origin - sphere.center)) = r^2
      //   = [t^2 * dotself(ray.direction)] + [2 * t * dot(ray.direction, ray.origin - sphere.center)] + [dotself(ray.origin - sphere.center) - (r^2)] = 0
      //  where 
      //        ray.origin = A
      //        ray.direction = B
      //        ray(t) = A + t * B,
      //        sphere.center = C
      //   = [t^2 * dotself(B)] + [2 * t * dot(B, A-C)] + [dotself(A-C) - (r^2)] = 0
      //   = [t^2 * (a)] + [t * (b)] + [(c)] = 0
      // just solve quadratic equation (-b +/- sqrt(b^2 - 4ac)) / 2a
      // we need to check just is answer exists. so, check if (b^2 - 4ac) is positive
      let sphere = self;
      let center = ray.origin - sphere.center;
      let quadratic_a = Matrix::dot(&ray.direction, &ray.direction);
      let quadratic_b = Matrix::dot(&center,&ray.direction) * 2.0;
      let quadratic_c = Matrix::dot(&center, &center) - sphere.radius.powi(2);
      let discriminant = quadratic_b.powi(2) - 4.0 * quadratic_a * quadratic_c;
      if discriminant < 0.0 {
          return None
      } else {
          let t_pos = (-quadratic_b - discriminant.sqrt()) / (2.0 * quadratic_a);
          let t_neg = (-quadratic_b - discriminant.sqrt()) / (2.0 * quadratic_a);
          let t = t_pos.max(t_neg);
          if t <= 0.001 || t >= f32::INFINITY { return None }
          let hit_surface = ray.point_at_parameter(t);
          return Some(HitRecordGeometric {
              t: t,
              position: hit_surface,
              normal: (hit_surface - sphere.center).normalize(),
          })
      }
  }
}