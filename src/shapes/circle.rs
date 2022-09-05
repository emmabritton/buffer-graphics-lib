use crate::coord::Coord;
use crate::shapes::DrawType;
use crate::Graphics;

pub fn circle_contains(center: Coord, radius: usize, point: Coord) -> bool {
    let dist = center.distance(point);
    dist <= radius
}

pub fn circle_draw(graphics: &mut Graphics, center: Coord, radius: usize, draw_type: DrawType) {
    match draw_type {
        DrawType::Stroke(color) => {
            let cx = center.x as isize;
            let cy = center.y as isize;
            let mut d = (5_isize - (radius as isize) * 4) / 4;
            let mut x = 0;
            let mut y = radius as isize;
            let w = graphics.width as isize;
            let h = graphics.height as isize;

            let clamp_w = |num: isize| num.clamp(0, w);
            let clamp_h = |num: isize| num.clamp(0, h);

            while x <= y {
                graphics.update_pixel(clamp_w(cx + x), clamp_h(cy + y), color);
                graphics.update_pixel(clamp_w(cx + x), clamp_h(cy - y), color);
                graphics.update_pixel(clamp_w(cx - x), clamp_h(cy + y), color);
                graphics.update_pixel(clamp_w(cx - x), clamp_h(cy - y), color);
                graphics.update_pixel(clamp_w(cx + y), clamp_h(cy + x), color);
                graphics.update_pixel(clamp_w(cx + y), clamp_h(cy - x), color);
                graphics.update_pixel(clamp_w(cx - y), clamp_h(cy + x), color);
                graphics.update_pixel(clamp_w(cx - y), clamp_h(cy - x), color);
                if d < 0 {
                    d += 2 * x + 1
                } else {
                    d += 2 * (x - y) + 1;
                    y -= 1;
                }
                x += 1;
            }
        }
        DrawType::Fill(color) => {
            let cx = center.x as isize;
            let cy = center.y as isize;
            let w = graphics.width as isize;
            let h = graphics.height as isize;
            let squared_radius = (radius * radius) as isize;
            let clamp_w = |num: isize| num.clamp(0, w);
            let clamp_h = |num: isize| num.clamp(0, h);
            for y in 0..(radius as isize) {
                let up = cy - y;
                let down = cy + y;
                let half_width = (((squared_radius - y * y) as f64).sqrt().round() as isize).max(0);
                for x in 0..half_width {
                    let left = cx - x;
                    let right = cx + x;
                    graphics.update_pixel(clamp_w(left), clamp_h(up), color);
                    graphics.update_pixel(clamp_w(right), clamp_h(up), color);
                    graphics.update_pixel(clamp_w(left), clamp_h(down), color);
                    graphics.update_pixel(clamp_w(right), clamp_h(down), color);
                }
            }
        }
    }
}
