use crate::Graphics;
use crate::shapes::{DrawType, Shape};
use crate::coord::Coord;
use crate::drawing::Renderable;

#[cfg(feature = "serde_derive")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Circle {
    center_x: isize,
    center_y: isize,
    radius: usize,
    draw_type: DrawType
}

impl Circle {
    pub fn new(center_x: isize, center_y: isize, radius: usize, draw_type: DrawType) -> Self {
        Self { center_x, center_y, radius, draw_type }
    }

    pub fn center(&self) -> Coord {
        Coord::new(self.center_x, self.center_y)
    }

    pub fn radius(&self) -> usize {
        self.radius
    }
}

impl Shape for Circle {
    fn draw_type(&self) -> DrawType {
        self.draw_type
    }

    fn with_draw_type(&self, draw_type: DrawType) -> Self {
        Circle::new(self.center_x, self.center_y, self.radius, draw_type)
    }

    fn translate_by<P: Into<Coord>>(&self, delta: P) -> Self {
        let delta = delta.into();
        Circle::new(self.center_x + delta.x, self.center_y + delta.y, self.radius, self.draw_type)
    }

    fn move_to<P: Into<Coord>>(&self, xy: P) -> Self {
        let xy = xy.into();
        Circle::new(xy.x, xy.y, self.radius, self.draw_type)
    }

    fn points(&self) -> Vec<Coord> {
        vec![self.center()]
    }
}

impl Renderable for Circle {
    fn render(&self, graphics: &mut Graphics) {
        match self.draw_type {
            DrawType::Stroke(color) => {
                let cx = self.center_x as isize;
                let cy = self.center_y as isize;
                let mut d = (5_isize - (self.radius as isize) * 4) / 4;
                let mut x = 0;
                let mut y = self.radius as isize;
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
                let cx = self.center_x as isize;
                let cy = self.center_y as isize;
                let w = graphics.width as isize;
                let h = graphics.height as isize;
                let squared_radius = (self.radius * self.radius) as isize;
                let clamp_w = |num: isize| num.clamp(0, w);
                let clamp_h = |num: isize| num.clamp(0, h);
                for y in 0..(self.radius as isize) {
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
}