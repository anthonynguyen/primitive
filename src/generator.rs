use std::cmp;

use rand::{Rng, StdRng};

use bresenham;
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

    // Generate a pair of points by generating an initial point, and a
    // displacement vector. The resulting final point is clamped to fit within
    // the image boundaries
    fn endpoints(&mut self, len: u32) -> ((u32, u32), (u32, u32)) {
        let p1 = self.point();
        let angle = (self.angle() as f64).to_radians();

        let lenf64 = len as f64;

        let p2 = (
            clamp((angle.cos() * lenf64) as i32 + p1.0 as i32, self.xmax - 1),
            clamp((angle.sin() * lenf64) as i32 + p1.1 as i32, self.ymax - 1)
        );

        (p1, p2)
    }

    pub fn line(&mut self, len: u32) -> Vec<(u32, u32)> {
        let (p1, p2) = self.endpoints(len);
        bresenham::points(p1, p2)
    }

    pub fn rect(&mut self, len: u32) -> Vec<(u32, u32)> {
        let (p1, p2) = self.endpoints(len);

        let minx = cmp::min(p1.0, p2.0);
        let maxx = cmp::max(p1.0, p2.0);

        let miny = cmp::min(p1.1, p2.1);
        let maxy = cmp::max(p1.1, p2.1);

        let mut res = Vec::new();

        for x in minx..maxx {
            for y in miny..maxy {
                res.push((x, y))
            }
        }

        res
    }
}

fn clamp(num: i32, max: u32) -> u32 {
    cmp::min(cmp::max(0, num) as u32, max)
}
