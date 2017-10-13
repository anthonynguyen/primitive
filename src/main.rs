extern crate chrono;
#[macro_use]
extern crate error_chain;
extern crate image;
#[macro_use]
extern crate log;
extern crate loglog;
extern crate rand;

use std::path::Path;

use image::{ImageBuffer, Rgb};

mod bresenham;
mod distance;
mod generator;

mod errors;
use errors::*;

fn run(iterations: u32, print_iter: bool) -> Result<()> {
    let img = image::open(&Path::new("example_s.png"))?.to_rgb();

    let (w, h) = img.dimensions();
    let mut buf = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(w, h);
    let mut gen = generator::Generator::new(w, h, 0, 360);

    for i in 0..iterations + 1 {
        if print_iter && i % 10_000 == 0 {
            debug!("{:02}% ({}/{})", (i as f32 / iterations as f32 * 100f32) as u8, i, iterations);
            buf.save(&Path::new(&format!("output/{:07}.jpg", i / 10_000)))?;
        }

        let (sample_x, sample_y) = gen.point();
        let sample_pixel = img.get_pixel(sample_x, sample_y).clone();

        let ((x1, y1), (x2, y2)) = gen.line(25);
        let points = bresenham::points((x1, y1), (x2, y2));

        let mut before_dist = 0;
        let mut after_dist = 0;
        for point in &points {
            let (x, y) = *point;
            before_dist += distance::manhattan(img.get_pixel(x, y), buf.get_pixel(x, y));
            after_dist += distance::manhattan(img.get_pixel(x, y), &sample_pixel);
        }

        if after_dist < before_dist {
            for point in &points {
                buf.put_pixel(point.0, point.1, sample_pixel);
            }
        }
    }

    buf.save(&Path::new("test.png"))?;

    Ok(())
}

fn bench() -> Result<()> {
    let num_outer_iter = 10;
    let num_inner_iter = 100_000;

    let num_iter = num_outer_iter * num_inner_iter;

    let start_time = chrono::Utc::now();

    for i in 0..num_outer_iter {
        run(num_inner_iter, false)?;
        info!("Iteration {}/{} done", i + 1, num_outer_iter);
    }

    let end_time = chrono::Utc::now();
    let elapsed = end_time.signed_duration_since::<chrono::Utc>(start_time);
    let nanos = elapsed.num_nanoseconds().ok_or_else(|| "Couldn't get nanos")?;

    info!("{} iterations completed in {}", num_iter, elapsed);
    info!("{} ns/iter", nanos as f64 / num_iter as f64);

    Ok(())
}

fn main() {
    loglog::build().target(false).init().unwrap_or_else(|err| {
        eprintln!("Could not start logger:\n{}", err);
        std::process::exit(1);
    });

    // if let Err(e) = run(1_000_000, true) {
    if let Err(e) = bench() {
        error!("{}", e);

        for e in e.iter().skip(1) {
            error!("Caused by: {}", e);
        }

        if let Some(backtrace) = e.backtrace() {
            error!("Backtrace: {:?}", backtrace);
        }

        std::process::exit(1);
    }
}
