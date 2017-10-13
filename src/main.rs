extern crate image;
extern crate imageproc;
extern crate rand;

use std::cmp;
use std::path::Path;

use image::{GenericImage, ImageBuffer, Pixel, Rgb};
use imageproc::drawing::draw_line_segment;
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

fn diffa<P, T>(img1: &T, img2: &T) -> Option<u64>
    where P: Pixel<Subpixel = u8>,
          T: GenericImage<Pixel = P> {
    let mut d = 0;

    if img1.dimensions() != img2.dimensions() {
        return None;
    }

    let (w, h) = img1.dimensions();

    for x in 0..w {
        for y in 0..h {
            d += euc(img1.get_pixel(x, y), img2.get_pixel(x, y));
        }
    }

    Some(d)
}

fn len(p1: (u32, u32), p2: (u32, u32)) -> u32 {
    (((p2.0 - p1.0).pow(2) + (p2.1 - p1.1).pow(2)) as f64).sqrt() as u32
}

fn line(w: &Range<u32>, h: &Range<u32>, rng: &mut rand::ThreadRng) -> ((u32, u32), (u32, u32)) {
    let mut done: bool = false;

    let mut x1: u32 = 0;
    let mut x2: u32 = 0;
    let mut y1: u32 = 0;
    let mut y2: u32 = 0;

    while !done {
        x1 = w.ind_sample(rng);
        y1 = h.ind_sample(rng);
        x2 = w.ind_sample(rng);
        y2 = h.ind_sample(rng);

        done = len((x1, y1), (x2, y2)) <= 70;
    }

    ((x1, y1), (x2, y2))
}

fn main() {
    let mut img = image::open(&Path::new("clown_small.jpg")).unwrap().to_rgb();
    let (w, h) = img.dimensions();

    let mut buf = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(w, h);

    let w_range = Range::new(0, w);
    let h_range = Range::new(0, h);
    let mut rng = rand::thread_rng();

    for i in 0..500_000 {
        if i % 1_000 == 0 {
            println!("{}", i);
            // let _ = buf.save(&Path::new(&format!("output/{:04}.jpg", i / 1000))).unwrap();
        }

        let (pick_x, pick_y) = (w_range.ind_sample(&mut rng), h_range.ind_sample(&mut rng));
        // let (pick_x, pick_y) = (200u32, 200u32);

        let source_pixel = img.get_pixel(pick_x, pick_y).clone();

        // println!("source: {}, {}", pick_x, pick_y);

        // get the section of the buffer to adjust
        // let (dest_x1, dest_y1) = (w_range.ind_sample(&mut rng), h_range.ind_sample(&mut rng));
        // let (dest_x2, dest_y2) = (w_range.ind_sample(&mut rng), h_range.ind_sample(&mut rng));

        let ((dest_x1, dest_y1), (dest_x2, dest_y2)) = line(&w_range, &h_range, &mut rng);

        // let (dest_x1, dest_y1) = (100u32, 120u32);
        // let (dest_x2, dest_y2) = (150u32, 80u32);


        // println!("dest_1: {}, {}", dest_x1, dest_y1);
        // println!("dest_2: {}, {}", dest_x2, dest_y2);

        let dest_minx = cmp::min(dest_x1, dest_x2);
        let dest_miny = cmp::min(dest_y1, dest_y2);

        // println!("dest_min: {}, {}", dest_minx, dest_miny);

        let dest_maxx = cmp::max(dest_x1, dest_x2);
        let dest_maxy = cmp::max(dest_y1, dest_y2);

        // println!("dest_max: {}, {}", dest_maxx, dest_maxy);

        let dest_w = (dest_maxx as i32 - dest_minx as i32) as u32;
        let dest_h = (dest_maxy as i32 - dest_miny as i32) as u32;

        // println!("dest_size: {}, {}", dest_w, dest_h);

        // create a 0,0-based section of the buffer
        let (norm_x1, norm_y1) = (
            (dest_x1 as i32 - dest_minx as i32) as f32,
            (dest_y1 as i32 - dest_miny as i32) as f32
        );
        let (norm_x2, norm_y2) = (
            (dest_x2 as i32 - dest_minx as i32) as f32,
            (dest_y2 as i32 - dest_miny as i32) as f32
        );

        // println!("norm_1: {}, {}", norm_x1, norm_y1);
        // println!("norm_2: {}, {}", norm_x2, norm_y2);

        let img_section = img.sub_image(dest_minx, dest_miny, dest_w, dest_h).to_image();
        let buf_section = buf.sub_image(dest_minx, dest_miny, dest_w, dest_h).to_image();
        let mut buf2 = buf_section.clone();

        let buf2 = draw_line_segment(&mut buf2, (norm_x1, norm_y1), (norm_x2, norm_y2), source_pixel);

        let before_dist = diffa(&img_section, &buf_section).unwrap();
        let after_dist = diffa(&img_section, &buf2).unwrap();

        if after_dist < before_dist {
            buf.copy_from(&buf2, dest_minx, dest_miny);
        }
    }

    let _ = buf.save(&Path::new("test.png")).unwrap();
}
