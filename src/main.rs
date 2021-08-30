use std::ops::Mul;
use std::{fs::File};
use std::io::Write;
use nalgebra::{Matrix, Vector3, vector};
use rand::Rng;

struct Mesh<'a> {
    geometry: &'a dyn Geometry,
    material: &'a dyn Material,
}

struct MaterialScatter {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<MaterialScatter>;
}

impl Mesh<'_> {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        self.geometry.hit(ray).map(|o| HitRecord {
            material: self.material,
            normal: o.normal,
            position: o.position,
            t: o.t
        })
    }
}

#[derive(Clone, Debug)]
struct Lambertian {
    albedo: Vec3
}
impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<MaterialScatter> {
        let target = rec.position + rec.normal + random_in_unit_sphere();
        return Some(MaterialScatter {
            scattered: Ray { origin: rec.position, direction: target - rec.position },
            attenuation: self.albedo.to_owned(),
        });
    }
}
impl Default for Lambertian {
    fn default() -> Self {
        Self { albedo: vector![0.0, 1.0, 0.5] }
    }
}
impl Lambertian {
    fn new(albedo: Vec3) -> Self {
        Self {
            albedo,
        }
    }
}

#[derive(Clone, Debug)]
struct Metal {
    albedo: Vec3,
    fuzz: f32,
}
impl Default for Metal {
    fn default() -> Self {
        Self { albedo: vector![0.0, 1.0, 0.5], fuzz: 1.0 }
    }
}
impl Metal {
    fn new(albedo: Vec3, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0)
        }
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    return v - 2.0 * v.dot(n) * n;
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<MaterialScatter> {
        // let target = rec.position + rec.normal + random_in_unit_sphere();
        let reflected = reflect(&r_in.direction.normalize(), &rec.normal);
        let scattered = Ray { origin: rec.position, direction: reflected + self.fuzz * random_in_unit_sphere() };
        let attenuation = self.albedo.to_owned();
        if scattered.direction.dot(&rec.normal) > 0.0 {
            Some(MaterialScatter {
                scattered,
                attenuation
            })
        } else {
            None
        }
    }
}



type Vec3 = Vector3<f32>;
struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}
struct HitRecordGeometric {
    pub t: f32,
    pub position: Vec3,
    pub normal: Vec3,
}
struct HitRecord<'a> {
    pub t: f32,
    pub position: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material
}
impl Ray {
    fn point_at_parameter(&self, t: f32) -> Vec3 {
        return self.origin + self.direction * t;
    }

    /// return is `Some(t: f32)` where if hit at position `ray.center + t * ray.direction`
    fn hit<'a>(&self, mesh: &'a Mesh) -> Option<HitRecord<'a>> {
        mesh.hit(&self)
    }
    fn hits<'a>(&self, meshs: &mut dyn Iterator<Item = &Box<&'a Mesh>>) -> Option<HitRecord<'a>> {
        meshs
            .filter_map(|b| self.hit(*b.as_ref()))
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(std::cmp::Ordering::Equal))
    }
}

trait Geometry {
    fn hit(&self, ray: &Ray) -> Option<HitRecordGeometric>;
}
struct Sphere {
    center: Vec3,
    radius: f32
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

struct Model {}
impl Geometry for Model {
    fn hit(&self, _ray: &Ray) -> Option<HitRecordGeometric> {
        None
    }
}

struct Camera {
    pub lower_left_coner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3,
}
impl Camera {
    fn get_ray(&self, u: f32, v: f32) -> Ray {
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

/// crerate unsmooth texture
fn random_in_unit_sphere() -> Vec3 {
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

fn color(ray: &Ray, meshs: &Vec<Box<&Mesh>>, depth: u32) -> Vec3 {
    // if let Some(hit) = ray.hit(&sphere) {
    if let Some(hit) = ray.hits(&mut meshs.iter()) {
        if depth < 50 {
            if let Some(MaterialScatter { attenuation , scattered }) = hit.material.scatter(ray, &hit) {
                let a: Vec3 = attenuation;
                let b: Vec3 = color(&scattered, meshs, depth + 1);
                return a.zip_map(&b, |a, b| a * b)
            }
        }
        let target = hit.position + hit.normal + random_in_unit_sphere() * 0.5;
        let recursive_color = 0.5 * color(&Ray {
            origin: hit.position,
            direction: target - hit.position
        }, &meshs, depth + 1);
        return recursive_color
    }
    let t = 0.3 * (ray.direction.normalize().y + 1.0);
    let white_color = vector![1.0, 1.0, 1.0];
    let blue_color = vector![0.5, 0.7, 1.0];
    return white_color.lerp(&blue_color, t); // Linear interpolation
}

fn main() {
    let mut w = File::create("image.ppm").unwrap();

    let nx = 200;
    let ny = 100;

    writeln!(&mut w, "P3").unwrap();
    writeln!(&mut w, "{} {}", nx, ny).unwrap();
    writeln!(&mut w, "{}", 255).unwrap();

    let camera = Camera::default();

    let sphere  = Mesh { geometry: &Sphere { center: vector![0.0, 0.0, -1.0], radius: 0.5 }, material: &Lambertian { albedo: vector![0.8, 0.3, 0.3] } };
    let sphere2 = Mesh { geometry: &Sphere { center: vector![0.0, -100.5, -1.0], radius: 100.0 }, material: &Lambertian { albedo: vector![0.8, 0.8, 0.0] } };
    let sphere3 = Mesh { geometry: &Sphere { center: vector![1.0, 0.0, -1.0], radius: 0.5 }, material: &Metal { albedo: vector![0.8, 0.6, 0.2], fuzz: 1.0 } };
    let sphere4 = Mesh { geometry: &Sphere { center: vector![-1.0, 0.0, -1.0], radius: 0.5 }, material: &Metal { albedo: vector![0.8, 0.8, 0.8], fuzz: 0.3 } };

    let meshs: Vec<Box<&Mesh>> = vec![
        Box::new(&sphere),
        Box::new(&sphere2),
        Box::new(&sphere3),
        Box::new(&sphere4),
    ];

    for v in (0..ny).map(|i| i as f32 / ny as f32).rev() {
    for u in (0..nx).map(|i| i as f32 / nx as f32) {
        let num_sample = 6;
        let mut rng = rand::thread_rng();
        let average_color = (0..num_sample).map(|_| {
            let u = u + (rng.gen_range(0.0..1.0) / (nx as f32));
            let v = v + (rng.gen_range(0.0..1.0) / (ny as f32));
            color(&camera.get_ray(u, v), &meshs, 0)
        }).sum::<Vec3>() / (num_sample as f32);
        let gamma = 2;
        let rgb: Vec3 = average_color
            .map(|f| f.powf(1.0 / gamma as f32))
            .map(|f| (f * 255.99).floor());
        writeln!(&mut w, "{:.0} {:.0} {:.0}", rgb.x, rgb.y, rgb.z).unwrap();
    }}
}
