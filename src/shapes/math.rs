use crate::Coord;
use crate::shapes::rect::Rect;

pub trait Contains<P: Into<Coord>> {
    fn contains(&self, xy: P) -> bool;
}

impl <P: Into<Coord>> Contains<P> for Rect {
    fn contains(&self, xy: P) -> bool {
        let xy = xy.into();
        xy.x >= self.topleft().x && xy.x < self.bottomright().x && xy.y >= self.topleft().y && xy.y < self.bottomright().y
    }
}

#[cfg(test)]
mod test {
    use crate::color::BLACK;
    use crate::shapes::DrawType;
    use super::*;

    #[test]
    fn contains_isize_rect() {
        let rect = Rect::new((50, 50), (100, 100), DrawType::Stroke(BLACK));
        assert!(!rect.contains((-60_isize, 60)));
        assert!(!rect.contains((0_isize, 0)));
        assert!(rect.contains((60_isize, 60)));
        assert!(!rect.contains((150_isize, 60)));
    }

    // #[test]
    // fn contains_usize_rect() {
    //     let rect = Rect::new(50, 50, 100, 100);
    //     assert!(!rect.contains(0_usize, 0));
    //     assert!(rect.contains(60_usize, 60));
    //     assert!(!rect.contains(150_usize, 60));
    // }
}
