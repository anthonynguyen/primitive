use image::{ImageBuffer, Rgb, open};

use errors::*;
use generator::Generator;
use settings::Settings;

pub struct Context {
	pub image: ImageBuffer<Rgb<u8>, Vec<u8>>,
	pub w: u32,
	pub h: u32,

	pub generator: Generator,
	pub settings: Settings
}

pub fn new(settings: Settings) -> Result<Context> {
	let img = open(&settings.input_file)?.to_rgb();
    let (w, h) = img.dimensions();

    let gen = Generator::new(w, h, settings.min_angle, settings.max_angle)?;
	Ok(Context { image: img, w, h, generator: gen, settings })
}
