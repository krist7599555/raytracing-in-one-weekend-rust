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
}

fn main() {
    let mut w = File::create("image.ppm").unwrap();

    let nx = 200;
    let ny = 100;

    writeln!(&mut w, "P3").unwrap();
    writeln!(&mut w, "{} {}", nx, ny).unwrap();
    writeln!(&mut w, "{}", 255).unwrap();

    for j in (0..ny).map(|i| i as f32 / ny as f32).rev() {
    for i in (0..nx).map(|i| i as f32 / nx as f32) {
        let rgb: Vec3 = vector![i, j, 0.2] * 255.0;
        writeln!(&mut w, "{:.0} {:.0} {:.0}", rgb.x, rgb.y, rgb.z).unwrap();
    }}
}
