use std::fs::File;
use std::io::Write;
use nalgebra::{Vector3, vector};

type Vec3 = Vector3<f32>;
struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}
impl Ray {
    fn point_at_parameter(&self, t: f32) -> Vec3 {
        return self.origin + self.direction * t;
    }
    fn unit_direction(&self) -> Vec3 {
        self.direction.normalize()
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
        let t = 0.3 * (ray.unit_direction().y + 1.0);
        let white_color = vector![1.0, 1.0, 1.0];
        let blue_color = vector![0.5, 0.7, 1.0];
        return white_color.lerp(&blue_color, t); // Linear interpolation
    }

    for u in (0..ny).map(|i| i as f32 / ny as f32).rev() {
    for v in (0..nx).map(|i| i as f32 / nx as f32) {
        let ray = Ray {
            origin, 
            direction: lower_left_coner + u * horizontal + v * vertical
        };
        let rgb: Vec3 = color(&ray) * 255.99;
        writeln!(&mut w, "{:.0} {:.0} {:.0}", rgb.x, rgb.y, rgb.z).unwrap();
    }}
}
