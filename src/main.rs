extern crate image;
extern crate rand;

use std::cmp;
use std::path::Path;

use image::{GenericImage, ImageBuffer, Pixel, Rgb};
use rand::distributions::{IndependentSample, Range};

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

fn diff<P, T>(img1: &T, img2: &T, p1: (u32, u32), p2: (u32, u32)) -> Option<u64>
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

    let mut buf = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(w, h);

    let w_range = Range::new(0, w);
    let h_range = Range::new(0, h);
    let mut rng = rand::thread_rng();

    for _ in 1..50_000 {
        let (pick_x, pick_y) = (w_range.ind_sample(&mut rng), h_range.ind_sample(&mut rng));
        let (set_x, set_y) = (w_range.ind_sample(&mut rng), h_range.ind_sample(&mut rng));

        let pix = img.get_pixel(pick_x, pick_y);

        let mut buf2 = buf.clone();

        let before_dist = diff(&img, &buf, (set_x, pick_y), (set_x, set_y)).unwrap();
        buf2.put_pixel(set_x, set_y, pix.clone());
        let after_dist = diff(&img, &buf2, (set_x, pick_y), (set_x, set_y)).unwrap();

        if after_dist < before_dist {
            buf = buf2;
        }
    }

    let _ = buf.save(&Path::new("test.png")).unwrap();
}
