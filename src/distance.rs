use image::Pixel;

pub fn euclidean<P: Pixel<Subpixel = u8>>(p1: &P, p2: &P) -> u32 {
    let c1 = p1.to_rgb();
    let c2 = p2.to_rgb();

    // we're probably guaranteed that the length = 3
    (((c2[0] as i32 - c1[0] as i32).pow(2) +
      (c2[1] as i32 - c1[1] as i32).pow(2) +
      (c2[2] as i32 - c1[2] as i32).pow(2)) as f64).sqrt() as u32
}

pub fn manhattan<P: Pixel<Subpixel = u8>>(p1: &P, p2: &P) -> u32 {
    let c1 = p1.to_rgb();
    let c2 = p2.to_rgb();

    // we're probably guaranteed that the length = 3
    ((c2[0] as i32 - c1[0] as i32).abs() +
     (c2[1] as i32 - c1[1] as i32).abs() +
     (c2[2] as i32 - c1[2] as i32).abs()) as u32
}
