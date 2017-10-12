extern crate image;

use std::cmp;
use std::path::Path;

use image::{GenericImage, ImageBuffer, Pixel, Rgb};

fn euc<P: Pixel<Subpixel = u8>>(p1: P, p2: P) -> u64 {
    let p1_rgb = p1.to_rgb();
    let p2_rgb = p2.to_rgb();

    let c1 = p1_rgb.channels();
    let c2 = p2_rgb.channels();

    // we're probably guaranteed that the length = 3
    (((c2[0] as i32 - c1[0] as i32).pow(2) +
      (c2[1] as i32 - c1[1] as i32).pow(2) +
      (c2[2] as i32 - c1[2] as i32).pow(2)) as f64).sqrt() as u64
}

fn diff<P, T>(img1: T, img2: T, p1: (u32, u32), p2: (u32, u32)) -> Option<u64>
    where P: Pixel<Subpixel = u8>,
          T: GenericImage<Pixel = P> {
    let (x1, y1) = p1;
    let (x2, y2) = p2;

    let mut d = 0;

    if !(img1.in_bounds(x1, y1) && img1.in_bounds(x2, y2)) ||
       !(img2.in_bounds(x1, y1) && img2.in_bounds(x2, y2)) {
        return None;
    }

    let sx = cmp::min(x1, x2);
    let bx = cmp::max(x1, x2);

    let sy = cmp::min(y1, y2);
    let by = cmp::max(y1, y2);

    for x in sx..bx + 1 {
        for y in sy..by + 1 {
            d += euc(img1.get_pixel(x, y), img2.get_pixel(x, y));
        }
    }

    Some(d)
}

fn main() {
    let img = image::open(&Path::new("example_s.png")).unwrap().to_rgb();
    let (w, h) = img.dimensions();

    let buf = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(w, h);

    println!("{:?}", diff(img, buf, (0, 0), (w - 1, h - 1)));
}
