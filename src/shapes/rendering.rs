use crate::color::Color;
use crate::drawable::{DrawType, Drawable};
use crate::drawing::Renderable;
use crate::Graphics;
use graphics_shapes::circle::Circle;
use graphics_shapes::line::Line;
use graphics_shapes::polygon::Polygon;
use graphics_shapes::rect::Rect;
use graphics_shapes::triangle::Triangle;
use graphics_shapes::Shape;

impl Renderable for Drawable<Line> {
    fn render(&self, graphics: &mut Graphics) {
        graphics.draw_line(
            self.drawing_points()[0],
            self.drawing_points()[1],
            self.draw_type().color(),
        )
    }
}

impl Renderable for Drawable<Rect> {
    fn render(&self, graphics: &mut Graphics) {
        let points = &self.drawing_points();
        match self.draw_type() {
            DrawType::Stroke(color) => {
                for x in points[0].x..=points[1].x {
                    graphics.update_pixel(x, points[0].y, color);
                    graphics.update_pixel(x, points[1].y, color);
                }
                for y in points[0].y..=points[1].y {
                    graphics.update_pixel(points[0].x, y, color);
                    graphics.update_pixel(points[1].x, y, color);
                }
            }
            DrawType::Fill(color) => {
                for x in points[0].x..=points[1].x {
                    for y in points[0].y..=points[1].y {
                        graphics.update_pixel(x, y, color);
                    }
                }
            }
        }
    }
}

impl Renderable for Drawable<Circle> {
    fn render(&self, graphics: &mut Graphics) {
        match self.draw_type() {
            DrawType::Stroke(color) => {
                let cx = self.obj().center().x as isize;
                let cy = self.obj().center().y as isize;
                let mut d = (5_isize - (self.obj().radius() as isize) * 4) / 4;
                let mut x = 0;
                let mut y = self.obj().radius() as isize;

                while x <= y {
                    graphics.update_pixel(cx + x, cy + y, color);
                    graphics.update_pixel(cx + x, cy - y, color);
                    graphics.update_pixel(cx - x, cy + y, color);
                    graphics.update_pixel(cx - x, cy - y, color);
                    graphics.update_pixel(cx + y, cy + x, color);
                    graphics.update_pixel(cx + y, cy - x, color);
                    graphics.update_pixel(cx - y, cy + x, color);
                    graphics.update_pixel(cx - y, cy - x, color);
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
                let cx = self.obj().center().x as isize;
                let cy = self.obj().center().y as isize;
                let squared_radius = (self.obj().radius() * self.obj().radius()) as isize;
                for y in 0..(self.obj().radius() as isize) {
                    let up = cy - y;
                    let down = cy + y;
                    let half_width =
                        (((squared_radius - y * y) as f64).sqrt().round() as isize).max(0);
                    for x in 0..=half_width {
                        let left = cx - x;
                        let right = cx + x;
                        graphics.update_pixel(left, up, color);
                        graphics.update_pixel(right, up, color);
                        graphics.update_pixel(left, down, color);
                        graphics.update_pixel(right, down, color);
                    }
                }
            }
        }
    }
}

impl Renderable for Drawable<Triangle> {
    fn render(&self, graphics: &mut Graphics) {
        let color = self.draw_type().color();
        let points = self.drawing_points();
        graphics.draw_line(points[0], points[1], color);
        graphics.draw_line(points[1], points[2], color);
        graphics.draw_line(points[0], points[2], color);
        if let DrawType::Fill(_) = self.draw_type() {
            let points = [
                (points[0].x as f32, points[0].y as f32),
                (points[1].x as f32, points[1].y as f32),
                (points[2].x as f32, points[2].y as f32),
            ];
            if points[1].1 == points[2].1 {
                draw_flat_bottom(graphics, points, color);
            } else if points[0].1 == points[1].1 {
                draw_flat_top(graphics, points, color);
            } else {
                let p = (
                    points[0].0
                        + ((points[1].1 - points[0].1) / (points[2].1 - points[0].1))
                            * (points[2].0 - points[0].0),
                    points[1].1,
                );
                draw_flat_bottom(graphics, [points[0], points[1], p], color);
                draw_flat_top(graphics, [points[1], p, points[2]], color);
            }
        }
    }
}

pub fn draw_flat_bottom(graphics: &mut Graphics, points: [(f32, f32); 3], color: Color) {
    let slope1 = (points[1].0 - points[0].0) / (points[1].1 - points[0].1);
    let slope2 = (points[2].0 - points[0].0) / (points[2].1 - points[0].1);
    let mut x1 = points[0].0;
    let mut x2 = points[0].0;
    for y in (points[0].1 as usize)..(points[1].1 as usize) {
        graphics.draw_line(
            ((x1.min(x2)) as usize, y),
            (x2.max(x1) as usize + 1, y),
            color,
        );
        x1 += slope1;
        x2 += slope2;
    }
}

pub fn draw_flat_top(graphics: &mut Graphics, points: [(f32, f32); 3], color: Color) {
    let slope1 = (points[2].0 - points[0].0) / (points[2].1 - points[0].1);
    let slope2 = (points[2].0 - points[1].0) / (points[2].1 - points[1].1);
    let mut x1 = points[2].0;
    let mut x2 = points[2].0;
    for y in ((points[0].1 as usize)..(points[2].1 as usize)).rev() {
        graphics.draw_line(
            ((x1.min(x2)) as usize, y),
            (x2.max(x1) as usize + 1, y),
            color,
        );
        x1 -= slope1;
        x2 -= slope2;
    }
}

impl Renderable for Drawable<Polygon> {
    fn render(&self, graphics: &mut Graphics) {
        let poly = self.obj().fpoints();
        let color = self.draw_type().color();
        for i in 0..poly.len() - 1 {
            graphics.draw_line(poly[i], poly[i + 1], color);
        }
        graphics.draw_line(poly[poly.len() - 1], poly[0], color);
        if let DrawType::Fill(_) = self.draw_type() {
            let width = graphics.width as f32;
            for y in 0..graphics.height {
                let mut node = vec![];
                let mut node_count = 0;
                let y = y as f32;
                let mut j = poly.len() - 1;
                for i in 0..poly.len() {
                    if poly[i].1 < y && poly[j].1 >= y || poly[j].1 < y && poly[i].1 >= y {
                        node.push(
                            poly[i].0
                                + (y - poly[i].1) / (poly[j].1 - poly[i].1)
                                    * (poly[j].0 - poly[i].0),
                        );
                        node_count += 1;
                    }
                    j = i;
                }
                let mut i = 0;
                if node_count > 0 {
                    while i < (node_count - 1) {
                        if node[i] > node[i + 1] {
                            node.swap(i, i + 1);
                            if i != 0 {
                                i -= 1;
                            }
                        } else {
                            i += 1;
                        }
                    }
                    for i in (0..node_count - 1).step_by(2) {
                        if node[i] >= width {
                            break;
                        }
                        if node[i + 1] > 0.0 {
                            if node[i] < 0.0 {
                                node[i] = 0.0;
                            }
                            if node[i + 1] > width {
                                node[i + 1] = width;
                            }
                            for x in (node[i] as usize)..(node[i + 1] as usize) {
                                graphics.set_pixel(x as isize + 1, y as isize, color);
                            }
                        }
                    }
                }
            }
        }
    }
}
