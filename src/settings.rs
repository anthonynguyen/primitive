use clap::ArgMatches;

use errors::*;

pub struct Settings {
    pub input_file: String,
    pub output_file: Option<String>,

    pub iterations: u32,

    pub line_length: u32,

    pub min_angle: u32,
    pub max_angle: u32,

    pub bench: bool,
    pub bench_iters: u32
}

pub fn new(matches: &ArgMatches) -> Result<Settings> {
    let input_file = matches.value_of("input")
        .ok_or_else(|| "Missing input!")
        .map(|inp| inp.to_string())?;

    let output_file = matches.value_of("output")
        .map(|out| out.to_string());

    let iterations = matches.value_of("iterations")
        .ok_or_else(|| "Missing iterations???")
        .map(|iter| iter.parse::<u32>())??;

    let line_length = matches.value_of("line_length")
        .ok_or_else(|| "Missing line length???")
        .map(|len| len.parse::<u32>())??;

    let min_angle = matches.value_of("min_angle")
        .ok_or_else(|| "Missing minimum angle???")
        .map(|angle| angle.parse::<u32>())??;

    let max_angle = matches.value_of("max_angle")
        .ok_or_else(|| "Missing maximum angle???")
        .map(|angle| angle.parse::<u32>())??;

    let mut bench = false;
    let mut bench_iters = 0;

    if let Some(sub_matches) = matches.subcommand_matches("bench") {
        bench = true;
        bench_iters = sub_matches.value_of("bench_iters")
            .ok_or_else(|| "Missing benchmark iterations???")
            .map(|iter| iter.parse::<u32>())??;
    }

    Ok({
        Settings {
            input_file,
            output_file,

            iterations,

            line_length,

            min_angle,
            max_angle,

            bench,
            bench_iters
        }
    })
}
