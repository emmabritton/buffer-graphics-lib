pub mod error;
pub mod rendering;

pub mod prelude {
    pub use crate::shapes::polyline::error::*;
    pub use crate::shapes::polyline::*;
}

use crate::color::Color;
use crate::shapes::polyline::error::PolylineError;
use crate::shapes::polyline::error::PolylineError::{InvalidPolyline, PolylineAlreadyClosed};
use crate::shapes::polyline::Segment::*;
use graphics_shapes::coord::Coord;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Segment {
    Start(Coord),
    LineTo(Coord),
    ArcAround {
        center: Coord,
        angle_start: isize,
        angle_end: isize,
        radius: usize,
    },
}

impl Segment {
    fn end_coord(&self) -> Coord {
        match self {
            Start(c) => *c,
            LineTo(c) => *c,
            ArcAround {
                center,
                radius,
                angle_end,
                ..
            } => Coord::from_angle(center, *radius, *angle_end),
        }
    }
}

pub struct Polyline {
    segments: Vec<Segment>,
    pub color: Color,
    closed: bool,
}

impl Polyline {
    pub fn new(segments: Vec<Segment>, color: Color) -> Self {
        Self {
            segments,
            color,
            closed: false,
        }
    }

    pub fn start<P: Into<Coord>>(start_at: P, color: Color) -> Self {
        Self {
            segments: vec![Start(start_at.into())],
            color,
            closed: false,
        }
    }

    pub fn rounded_rect(
        left: usize,
        top: usize,
        right: usize,
        bottom: usize,
        corner_size: usize,
        color: Color,
    ) -> Result<Self, PolylineError> {
        let tl_arc = Coord::from((left + corner_size, top + corner_size));
        let tr_arc = Coord::from((right - corner_size, top + corner_size));
        let bl_arc = Coord::from((left + corner_size, bottom - corner_size));
        let br_arc = Coord::from((right - corner_size, bottom - corner_size));

        let line1_end = Coord::from((right - corner_size, top));
        let line2_end = Coord::from((right, bottom - corner_size));
        let line3_end = Coord::from((left + corner_size, bottom));
        let line4_end = Coord::from((left, top + corner_size));

        Polyline::start((left + corner_size, top), color)
            .add_line_to(line1_end)?
            .add_arc_around(tr_arc, corner_size, 0, 90)?
            .add_line_to(line2_end)?
            .add_arc_around(br_arc, corner_size, 90, 90)?
            .add_line_to(line3_end)?
            .add_arc_around(bl_arc, corner_size, 180, 90)?
            .add_line_to(line4_end)?
            .add_arc_around(tl_arc, corner_size, 270, 90)
    }
}

impl Polyline {
    pub fn add_line_to<P: Into<Coord>>(mut self, point: P) -> Result<Self, PolylineError> {
        if self.closed {
            return Err(PolylineAlreadyClosed);
        }
        self.segments.push(LineTo(point.into()));
        Ok(self)
    }

    // pub fn add_arc_to<P: Into<Coord>>(mut self, point: P) -> Result<Self, PolylineError> {
    //     if self.closed { return Err(PolylineAlreadyClosed); }
    //
    //     let start = self.segments.last().unwrap().end_coord();
    //     let end = point.into();
    //     let cx =  start.x.min(end.x);
    //     let cy =  start.y.min(end.y);
    //     let center = Coord::new(cx, cy);
    //     let radius = start.distance(end) / 2;
    //     let angle_start = center.angle_to(start);
    //     let angle_end = center.angle_to(end);
    //     self.segments.push(ArcAround { center, radius, angle_start, angle_end });
    //
    //     Ok(self)
    // }

    pub fn add_arc_around<P: Into<Coord>>(
        mut self,
        center: P,
        radius: usize,
        start_angle: isize,
        sweep_angle: usize,
    ) -> Result<Self, PolylineError> {
        if self.closed {
            return Err(PolylineAlreadyClosed);
        }
        self.segments.push(ArcAround {
            center: center.into(),
            radius,
            angle_start: start_angle,
            angle_end: start_angle + (sweep_angle as isize),
        });
        Ok(self)
    }

    pub fn close(self) -> Result<Self, PolylineError> {
        if let Start(coord) = self.segments[0] {
            let mut tmp = self.add_line_to(coord)?;
            tmp.closed = true;
            Ok(tmp)
        } else {
            Err(InvalidPolyline)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::color::RED;
    use crate::shapes::polyline::Polyline;
    use crate::shapes::polyline::Segment::*;
    use graphics_shapes::coord::Coord;

    #[test]
    fn check_closing() {
        let polyline = Polyline::start((10, 10), RED)
            .add_line_to((20, 10))
            .unwrap()
            .add_line_to((20, 20))
            .unwrap()
            .add_line_to((10, 20))
            .unwrap()
            .close()
            .unwrap();
        assert_eq!(
            polyline.segments,
            vec![
                Start(Coord::new(10, 10)),
                LineTo(Coord::new(20, 10)),
                LineTo(Coord::new(20, 20)),
                LineTo(Coord::new(10, 20)),
                LineTo(Coord::new(10, 10))
            ]
        )
    }
}
