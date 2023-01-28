use crate::drawing::Renderable;
use crate::shapes::polyline::Polyline;
use crate::shapes::polyline::Segment::*;
use crate::Graphics;
use log::error;

impl Renderable<Polyline> for Polyline {
    fn render(&self, graphics: &mut Graphics) {
        if self.segments.len() < 2 {
            error!("Polyline only has start or is empty")
        }

        let mut last_coord = if let Start(coord) = self.segments[0] {
            coord
        } else {
            error!("Polyline is invalid, missing start");
            return;
        };

        for segment in self.segments.iter().skip(1) {
            match segment {
                Start(_) => error!("Polyline is invalid, second start found"),
                LineTo(coord) => graphics.draw_line(last_coord, coord, self.color),
                ArcAround {
                    center,
                    angle_start,
                    angle_end,
                    radius,
                } => graphics.draw_arc(
                    *center,
                    *angle_start,
                    *angle_end,
                    *radius,
                    false,
                    self.color,
                ),
            }
            last_coord = segment.end_coord();
        }
    }
}
