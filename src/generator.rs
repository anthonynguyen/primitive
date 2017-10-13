use std::cmp;

use rand::{Rng, StdRng};

use errors::*;

pub struct Generator {
    xmax: u32,
    ymax: u32,

    amin: u32,
    amax: u32,

    rng: StdRng
}

impl Generator {
    pub fn new(xmax: u32, ymax: u32, amin: u32, amax: u32) -> Result<Self> {
        Ok(Generator {
            xmax, ymax, amin, amax,
            rng: StdRng::new()?
        })
    }

    fn angle(&mut self) -> u32 {
        self.rng.gen_range(self.amin, self.amax)
    }

    pub fn point(&mut self) -> (u32, u32) {
        (
            self.rng.gen_range(0, self.xmax),
            self.rng.gen_range(0, self.ymax)
        )
    }

    // Generate a random line by generating an initial point, and a displacement
    // vector. The resulting final point is clamped to fit within the image
    // boundaries
    pub fn line(&mut self, len: u32) -> ((u32, u32), (u32, u32)) {
        let p1 = self.point();
        let angle = (self.angle() as f64).to_radians();

        let lenf64 = len as f64;

        let p2 = (
            clamp((angle.cos() * lenf64) as i32 + p1.0 as i32, self.xmax - 1),
            clamp((angle.sin() * lenf64) as i32 + p1.1 as i32, self.ymax - 1)
        );

        (p1, p2)
    }
}

fn clamp(num: i32, max: u32) -> u32 {
    cmp::min(cmp::max(0, num) as u32, max)
}
