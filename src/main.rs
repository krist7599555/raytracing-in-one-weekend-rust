use std::{fs::File};
use std::io::Write;
use nalgebra::{Matrix, Vector3, vector};

type Vec3 = Vector3<f32>;
struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}
struct HitRecord {
    pub t: f32,
    pub position: Vec3,
    pub normal: Vec3
}
impl Ray {
    fn point_at_parameter(&self, t: f32) -> Vec3 {
        return self.origin + self.direction * t;
    }
    fn unit_direction(&self) -> Vec3 {
        self.direction.normalize()
    }

    /// return is `Some(t: f32)` where if hit at position `ray.center + t * ray.direction`
    fn hit(&self, mesh: &dyn RayHitable) -> Option<HitRecord> {
        mesh.hit(&self)
    }
    fn hits(&self, meshs: &mut dyn Iterator<Item = &Box<&dyn RayHitable>>) -> Option<HitRecord> {
        meshs
            .filter_map(|b| self.hit(*b.as_ref()))
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(std::cmp::Ordering::Equal))
    }
}

trait RayHitable {
    fn hit(&self, ray: &Ray) -> Option<HitRecord>;
}
struct Sphere {
    center: Vec3,
    radius: f32
}
impl RayHitable for Sphere {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
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
            if t <= 0.0 || t >= f32::INFINITY { return None }
            let hit_surface = ray.point_at_parameter(t);
            return Some(HitRecord {
                t: t,
                position: hit_surface,
                normal: (hit_surface - sphere.center).normalize()
            })
        }
    }
}

struct Model {}
impl RayHitable for Model {
    fn hit(&self, _ray: &Ray) -> Option<HitRecord> {
        None
    }
}

fn main() {
    let mut w = File::create("image.ppm").unwrap();

    let nx = 200;
    let ny = 100;

    writeln!(&mut w, "P3").unwrap();
    writeln!(&mut w, "{} {}", nx, ny).unwrap();
    writeln!(&mut w, "{}", 255).unwrap();

    let lower_left_coner: Vec3 = vector![-2.0, -1.0, -1.0];
    let horizontal: Vec3 = vector![4.0, 0.0, 0.0];
    let vertical: Vec3 = vector![0.0, 2.0, 0.0];
    let origin: Vec3 = vector![0.0, 0.0, 0.0];


    fn color(ray: &Ray) -> Vec3 {
        let sphere = Sphere { center: vector![0.0, 0.0, -1.0], radius: 0.5 };
        let sphere2 = Sphere { center: vector![1.0, 0.0, -1.0], radius: 0.5 };
        let sphere3 = Sphere { center: vector![0.0, 1.0, -1.0], radius: 0.2 };
        let sphere4 = Sphere { center: vector![0.0, -100.5, -1.0], radius: 100.0 };

        let meshs: Vec<Box<&dyn RayHitable>> = vec![
            Box::new(&sphere),
            Box::new(&sphere2),
            Box::new(&sphere3),
            Box::new(&sphere4),
            Box::new(&Model {})
        ];
        // if let Some(hit) = ray.hit(&sphere) {
        if let Some(hit) = ray.hits(&mut meshs.iter()) {
            return hit.normal.add_scalar(1.0) / 2.0; // convert Dr = [-1, 1] to Dr = [0, 1]
        }
        let t = 0.3 * (ray.direction.normalize().y + 1.0);
        let white_color = vector![1.0, 1.0, 1.0];
        let blue_color = vector![0.5, 0.7, 1.0];
        return white_color.lerp(&blue_color, t); // Linear interpolation
    }

    for v in (0..ny).map(|i| i as f32 / ny as f32).rev() {
    for u in (0..nx).map(|i| i as f32 / nx as f32) {
        let ray = Ray {
            origin, 
            direction: lower_left_coner + u * horizontal + v * vertical
        };
        let rgb: Vec3 = color(&ray).map(|f| (f * 255.99).floor());
        writeln!(&mut w, "{:.0} {:.0} {:.0}", rgb.x, rgb.y, rgb.z).unwrap();
    }}
}
