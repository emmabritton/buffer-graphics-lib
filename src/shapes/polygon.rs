use crate::shapes::DrawType;
use crate::{Coord, Graphics};

pub fn polygon_contains(poly: Vec<(f32, f32)>, point: Coord) -> bool {
    let mut j = poly.len() - 1;
    let mut odd_nodes = false;
    let fpoint = (point.x as f32, point.y as f32);

    for i in 0..poly.len() {
        if (poly[i].1 < fpoint.1 && poly[j].1 >= fpoint.1
            || poly[j].1 < fpoint.1 && poly[i].1 >= fpoint.1)
            && (poly[i].0 <= fpoint.0 || poly[j].0 <= fpoint.0)
        {
            odd_nodes ^= poly[i].0
                + (fpoint.1 - poly[i].1) / (poly[j].1 - poly[i].1) * (poly[j].0 - poly[i].0)
                < fpoint.0;
        }
        j = i;
    }

    odd_nodes
}

pub fn polygon_draw(graphics: &mut Graphics, poly: Vec<(f32, f32)>, draw_type: DrawType) {
    let color = draw_type.color();
    for i in 0..poly.len() - 1 {
        graphics.draw_line(poly[i], poly[i + 1], color);
    }
    graphics.draw_line(poly[poly.len() - 1], poly[0], color);
    if let DrawType::Fill(_) = draw_type {
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
                            + (y - poly[i].1) / (poly[j].1 - poly[i].1) * (poly[j].0 - poly[i].0),
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
