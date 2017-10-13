use std::cmp;

use rand;
use rand::distributions::{IndependentSample, Range};
use rand::ThreadRng;

pub struct Generator {
    xmax: u32,
    ymax: u32,

    xrange: Range<u32>,
    yrange: Range<u32>,

    arange: Range<u32>,

    rng: ThreadRng
}

impl Generator {
    pub fn new(xmax: u32, ymax: u32, amin: u32, amax: u32) -> Self {
        let xrange = Range::new(0, xmax);
        let yrange = Range::new(0, ymax);

        let arange = Range::new(amin, amax);

        let rng = rand::thread_rng();

        Generator {
            xmax,
            ymax,

            xrange,
            yrange,

            arange,

            rng
        }
    }

    fn angle(&mut self) -> u32 {
        self.arange.ind_sample(&mut self.rng)
    }

    pub fn point(&mut self) -> (u32, u32) {
        (
            self.xrange.ind_sample(&mut self.rng),
            self.yrange.ind_sample(&mut self.rng)
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
