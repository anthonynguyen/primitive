# `primitive`

primitive is a Rust program that approximates images by drawing random lines.

## Running

Assuming you have Rust installed, run

```
cargo install primitive
```

## Usage

```
primitive 0.1.0
Anthony Nguyen <anknguyen@gmail.com>

USAGE:
    primitive [OPTIONS] --input <INPUT_FILE> [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <INPUT_FILE>           Input filename
    -n, --iterations <NUM_ITERS>       Number of iterations to run [default: 1000000]
    -l, --line-length <LINE_LENGTH>    The length (in pixels) of the lines to draw [default: 25]
        --max-angle <MAX_ANGLE>        The maximum angle (in degrees) to generate [default: 360]
        --min-angle <MIN_ANGLE>        The minimum angle (in degrees) to generate [default: 0]
    -o, --output <OUTPUT_FILE>         Output filename

SUBCOMMANDS:
    bench    Runs in benchmark mode
    help     Prints this message or the help of the given subcommand(s)
```

## License

primitive is licensed under the MIT license. Please see the `LICENSE` file for
more details.
