// https://stackoverflow.com/a/34722500
pub fn points(p1: (u32, u32), p2: (u32, u32)) -> Vec<(u32, u32)> {
    let (mut x, mut y) = (p1.0 as i32, p1.1 as i32);
    let (x2, y2) = (p2.0 as i32, p2.1 as i32);

    let dx = if x > x2 { x - x2 } else { x2 - x };
    let dy = if y > y2 { y - y2 } else { y2 - y };

    let sx = if x < x2 { 1 } else { -1 };
    let sy = if y < y2 { 1 } else { -1 };

    let mut err = (if dx > dy { dx } else { -dy }) / 2;
    let mut err2;

    let mut ret = Vec::new();

    loop {
        ret.push((x as u32, y as u32));

        if x == x2 && y == y2 {
            break;
        }

        err2 = 2 * err;

        if err2 > -dx {
            err -= dy;
            x += sx;
        }

        if err2 < dy {
            err += dx;
            y += sy;
        }
    }

    ret
}
