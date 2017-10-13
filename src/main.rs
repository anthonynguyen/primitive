extern crate chrono;
extern crate image;
extern crate imageproc;
extern crate rand;

use std::cmp;
use std::path::Path;

use image::{GenericImage, ImageBuffer, Pixel, Rgb};
use imageproc::drawing::draw_line_segment;

mod generator;

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

fn diff<P, T>(img1: &T, img2: &T) -> Option<u64>
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

fn run(iterations: usize, print_iter: bool) {
    let mut img = image::open(&Path::new("example_s.png")).unwrap().to_rgb();
    let (w, h) = img.dimensions();

    let mut buf = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(w, h);

    let mut gen = generator::Generator::new(w, h);

    for i in 0..iterations {
        if print_iter && i % 10_000 == 0 {
            println!("{}", i);
            let _ = buf.save(&Path::new(&format!("output/{:04}.jpg", i / 1000))).unwrap();
        }

        let (pick_x, pick_y) = gen.point();

        let source_pixel = img.get_pixel(pick_x, pick_y).clone();

        let ((dest_x1, dest_y1), (dest_x2, dest_y2)) = gen.line(10);

        let dest_minx = cmp::min(dest_x1, dest_x2);
        let dest_miny = cmp::min(dest_y1, dest_y2);

        let dest_maxx = cmp::max(dest_x1, dest_x2);
        let dest_maxy = cmp::max(dest_y1, dest_y2);

        let dest_w = (dest_maxx as i32 - dest_minx as i32) as u32;
        let dest_h = (dest_maxy as i32 - dest_miny as i32) as u32;

        // create a 0,0-based section of the buffer
        let (norm_x1, norm_y1) = (
            (dest_x1 as i32 - dest_minx as i32) as f32,
            (dest_y1 as i32 - dest_miny as i32) as f32
        );
        let (norm_x2, norm_y2) = (
            (dest_x2 as i32 - dest_minx as i32) as f32,
            (dest_y2 as i32 - dest_miny as i32) as f32
        );

        let img_section = img.sub_image(dest_minx, dest_miny, dest_w, dest_h).to_image();
        let buf_section = buf.sub_image(dest_minx, dest_miny, dest_w, dest_h).to_image();
        let mut buf2 = buf_section.clone();

        let buf2 = draw_line_segment(&mut buf2, (norm_x1, norm_y1), (norm_x2, norm_y2), source_pixel);

        let before_dist = diff(&img_section, &buf_section).unwrap();
        let after_dist = diff(&img_section, &buf2).unwrap();

        if after_dist < before_dist {
            buf.copy_from(&buf2, dest_minx, dest_miny);
        }
    }

    let _ = buf.save(&Path::new("test.png")).unwrap();
}

fn bench() {
    let num_outer_iter = 10;
    let num_inner_iter = 100_000;

    let num_iter = num_outer_iter * num_inner_iter;

    let start_time = chrono::Utc::now();

    for i in 0..num_outer_iter {
        run(num_inner_iter, false);
        println!("Iteration {}/{} done", i + 1, num_outer_iter);
    }

    let end_time = chrono::Utc::now();
    let elapsed = end_time.signed_duration_since::<chrono::Utc>(start_time);
    let nanos = elapsed.num_nanoseconds().unwrap();

    println!("{} iterations completed in {}", num_iter, elapsed);
    println!("{}ns/iter", nanos as f64 / num_iter as f64)
}

fn main() {
    run(1_000_000, true);
    // bench();
}
