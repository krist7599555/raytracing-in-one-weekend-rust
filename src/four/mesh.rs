use super::{Geometry, Material, Ray, Vec3};

pub struct Mesh<'a> {
  pub geometry: &'a dyn Geometry,
  pub material: &'a dyn Material,
}
pub struct HitRecord<'a> {
  pub t:        f32,
  pub position: Vec3,
  pub normal:   Vec3,
  pub material: &'a dyn Material,
}
impl Mesh<'_> {
  pub fn hit(&self, ray: &Ray) -> Option<HitRecord> {
    self.geometry.hit(ray).map(|o| HitRecord {
      material: self.material,
      normal:   o.normal,
      position: o.position,
      t:        o.t,
    })
  }
}
