extern crate chrono;
extern crate clap;
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
mod settings;

mod errors;
use errors::*;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

fn single(s: &settings::Settings) -> Result<()> {
    let img = image::open(&Path::new(&s.input_file))?.to_rgb();

    let (w, h) = img.dimensions();
    let mut buf = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(w, h);
    let mut gen = generator::Generator::new(w, h, s.min_angle, s.max_angle);

    let iter_10 = s.iterations / 10;

    for i in 0..s.iterations + 1 {
        if !s.bench && i % iter_10 == 0 {
            info!("{:02}% ({}/{})", (i as f32 / s.iterations as f32 * 100f32) as u8, i, s.iterations);
            // buf.save(&Path::new(&format!("output/{:07}.jpg", i / 10_000)))?;
        }

        let (sample_x, sample_y) = gen.point();
        let sample_pixel = img.get_pixel(sample_x, sample_y).clone();

        let ((x1, y1), (x2, y2)) = gen.line(s.line_length);
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

    if !s.bench {
        if let Some(ref fname) = s.output_file {
            buf.save(&Path::new(fname))?;
        }
    }

    Ok(())
}

fn bench(s: &settings::Settings) -> Result<()> {
    let total_iters = s.bench_iters * s.iterations;
    let start_time = chrono::Utc::now();

    for i in 0..s.bench_iters {
        single(s)?;
        info!("Iteration {}/{} done", i + 1, s.bench_iters);
    }

    let end_time = chrono::Utc::now();
    let elapsed = end_time.signed_duration_since::<chrono::Utc>(start_time);
    let nanos = elapsed.num_nanoseconds().ok_or_else(|| "Couldn't get nanos")?;

    info!("{} iterations completed in {}", total_iters, elapsed);
    info!("{} ns/iter", nanos as f64 / total_iters as f64);

    Ok(())
}

fn run(matches: clap::ArgMatches) -> Result<()> {
    let s = settings::new(&matches)?;

    if !s.bench {
        single(&s)
    } else {
        bench(&s)
    }
}

fn main() {
    let matches = clap::App::new("primitive")
        .version(VERSION)
        .author(AUTHORS)
        .arg(
            clap::Arg::with_name("input")
                .help("Input filename")
                .required(true)
                .short("i")
                .long("input")
                .takes_value(true)
                .value_name("INPUT_FILE"),
        )
        .arg(
            clap::Arg::with_name("output")
                .help("Output filename")
                .short("o")
                .long("output")
                .takes_value(true)
                .value_name("OUTPUT_FILE"),
        )
        .arg(
            clap::Arg::with_name("iterations")
                .help("Number of iterations to run")
                .short("n")
                .long("iterations")
                .takes_value(true)
                .default_value("1000000")
                .value_name("NUM_ITERS"),
        )
        .arg(
            clap::Arg::with_name("line_length")
                .help("The length (in pixels) of the lines to draw")
                .short("l")
                .long("line-length")
                .takes_value(true)
                .default_value("25")
                .value_name("LINE_LENGTH"),
        )
        .arg(
            clap::Arg::with_name("min_angle")
                .help("The minimum angle (in degrees) to generate")
                .long("min-angle")
                .takes_value(true)
                .default_value("0")
                .value_name("MIN_ANGLE"),
        )
        .arg(
            clap::Arg::with_name("max_angle")
                .help("The maximum angle (in degrees) to generate")
                .long("max-angle")
                .takes_value(true)
                .default_value("360")
                .value_name("MAX_ANGLE"),
        )
        .subcommand(
            clap::SubCommand::with_name("bench")
                .about("Runs in benchmark mode")
                .arg(
                    clap::Arg::with_name("bench_iters")
                        .help("Number of benchmark iterations to run")
                        .long("bench-iterations")
                        .takes_value(true)
                        .default_value("10")
                        .value_name("BENCH_ITERS")
                )
        )
        .get_matches();

    loglog::build().target(false).init().unwrap_or_else(|err| {
        eprintln!("Could not start logger:\n{}", err);
        std::process::exit(1);
    });

    if let Err(e) = run(matches) {
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
