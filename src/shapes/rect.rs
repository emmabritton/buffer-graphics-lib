use crate::coord::Coord;
use crate::shapes::DrawType;
use crate::Graphics;

pub fn rect_contains(topleft: Coord, bottomright: Coord, point: Coord) -> bool {
    topleft.x <= point.x && bottomright.x > point.x &&
        topleft.y <= point.y && bottomright.y > point.y
}

pub fn rect_draw(graphics: &mut Graphics, topleft: Coord, bottomright: Coord, draw_type: DrawType) {
    match draw_type {
        DrawType::Stroke(color) => {
            for x in topleft.x..bottomright.x {
                graphics.update_pixel(x, topleft.y, color);
                graphics.update_pixel(x, bottomright.y, color);
            }
            for y in topleft.y..=bottomright.y {
                graphics.update_pixel(topleft.x, y, color);
                graphics.update_pixel(bottomright.x, y, color);
            }
        }
        DrawType::Fill(color) => {
            for x in topleft.x..bottomright.x {
                for y in topleft.y..bottomright.y {
                    graphics.update_pixel(x, y, color);
                }
            }
        }
    }
}
