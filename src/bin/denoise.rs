use oidn;
fn main() {
  println!("denoise");
  let input_img: Vec<f32> = // A float3 RGB image produced by your renderer
  let mut filter_output = vec![0.0f32; input_img.len()];

  let device = oidn::Device::new();
  oidn::RayTracing::new(&device)
      // Optionally add float3 normal and albedo buffers as well
      .srgb(true)
      .image_dimensions(input.width() as usize, input.height() as usize);
      .filter(&input_img[..], &mut filter_output[..])
      .expect("Filter config error!");

  if let Err(e) = device.get_error() {
      println!("Error denosing image: {}", e.1);
  }
}