use super::{Material, Vec3};

pub struct Ray {
  pub origin:    Vec3,
  pub direction: Vec3,
}
impl Ray {
  pub fn point_at_parameter(&self, t: f32) -> Vec3 { return self.origin + self.direction * t; }

  /// return is `Some(t: f32)` where if hit at position `ray.center + t *
  /// ray.direction`
  pub fn hit<'a>(&self, mesh: &'a dyn Rayable) -> Option<HitRecord<'a>> { mesh.hit(&self) }

  pub fn hits<'a>(
    &self, meshs: &mut dyn Iterator<Item = &'a dyn Rayable>,
  ) -> Option<HitRecord<'a>> {
    meshs
      .filter_map(|b| self.hit(b))
      .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(std::cmp::Ordering::Equal))
  }
}

pub trait Rayable {
  fn hit<'a>(&'a self, ray: &Ray) -> Option<HitRecord<'a>>;
}

pub struct HitRecord<'a> {
  pub t:        f32,
  pub position: Vec3,
  pub normal:   Vec3,
  pub material: &'a dyn Material,
}
