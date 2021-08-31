use super::{Geometry, Material, Ray, Vec3};

pub struct Mesh<G, M> where G: Geometry, M: Material {
  pub geometry: G,
  pub material: M,
}
pub trait Rayable {
  fn hit<'a>(&'a self, ray: &Ray) -> Option<HitRecord<'a>>;
}

impl<G, M> Mesh<G, M> where G: Geometry, M: Material {
  pub fn new(geometry: G, material: M) -> Self {
    Self {
      geometry,
      material,
    }
  }
}

impl<G, M> Rayable for Mesh<G, M> where G: Geometry, M: Material {
  fn hit(&self, ray: &Ray) -> Option<HitRecord> {
    self.geometry.hit(ray).map(|o| HitRecord {
      material: &self.material,
      normal:   o.normal,
      position: o.position,
      t:        o.t,
    })
  }
}

pub struct HitRecord<'a> {
  pub t:        f32,
  pub position: Vec3,
  pub normal:   Vec3,
  pub material: &'a dyn Material,
}
