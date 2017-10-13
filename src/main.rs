extern crate chrono;
extern crate image;
extern crate rand;

use std::path::Path;

use image::{ImageBuffer, Pixel, Rgb};

mod bresenham;
mod generator;

fn euclidean<P: Pixel<Subpixel = u8>>(p1: &P, p2: &P) -> u32 {
    let c1 = p1.to_rgb();
    let c2 = p2.to_rgb();

    // we're probably guaranteed that the length = 3
    (((c2[0] as i32 - c1[0] as i32).pow(2) +
      (c2[1] as i32 - c1[1] as i32).pow(2) +
      (c2[2] as i32 - c1[2] as i32).pow(2)) as f64).sqrt() as u32
}

fn manhattan<P: Pixel<Subpixel = u8>>(p1: &P, p2: &P) -> u32 {
    let c1 = p1.to_rgb();
    let c2 = p2.to_rgb();

    // we're probably guaranteed that the length = 3
    ((c2[0] as i32 - c1[0] as i32).abs() +
     (c2[1] as i32 - c1[1] as i32).abs() +
     (c2[2] as i32 - c1[2] as i32).abs()) as u32
}

fn run(iterations: u32, print_iter: bool) {
    let img = image::open(&Path::new("example_s.png")).unwrap().to_rgb();

    let (w, h) = img.dimensions();
    let mut buf = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(w, h);
    let mut gen = generator::Generator::new(w, h, 0, 360);

    for i in 0..iterations {
        if print_iter && i % 10_000 == 0 {
            println!("{}", i);
            let _ = buf.save(&Path::new(&format!("output/{:07}.jpg", i / 10_000))).unwrap();
        }

        let (sample_x, sample_y) = gen.point();
        let sample_pixel = img.get_pixel(sample_x, sample_y).clone();

        let ((x1, y1), (x2, y2)) = gen.line(25);
        let points = bresenham::points((x1, y1), (x2, y2));

        let mut before_dist = 0;
        let mut after_dist = 0;
        for point in &points {
            let (x, y) = *point;
            before_dist += manhattan(img.get_pixel(x, y), buf.get_pixel(x, y));
            after_dist += manhattan(img.get_pixel(x, y), &sample_pixel);
        }

        if after_dist < before_dist {
            for point in &points {
                buf.put_pixel(point.0, point.1, sample_pixel);
            }
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
    // run(1_000_000, true);
    bench();
}
