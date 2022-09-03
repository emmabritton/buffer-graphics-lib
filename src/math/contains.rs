use crate::math::Rect;

pub trait Contains<T> {
    fn contains(&self, x: T, y: T) -> bool;
}

impl<T: Ord> Contains<T> for Rect<T> {
    #[inline]
    fn contains(&self, x: T, y: T) -> bool {
        self.x1 <= x && x <= self.x2 && self.y1 <= y && y <= self.y2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn contains_isize_rect() {
        let rect = Rect::new(50, 50, 100, 100);
        assert!(!rect.contains(-60_isize, 60));
        assert!(!rect.contains(0_isize, 0));
        assert!(rect.contains(60_isize, 60));
        assert!(!rect.contains(150_isize, 60));
    }

    #[test]
    fn contains_usize_rect() {
        let rect = Rect::new(50, 50, 100, 100);
        assert!(!rect.contains(0_usize, 0));
        assert!(rect.contains(60_usize, 60));
        assert!(!rect.contains(150_usize, 60));
    }
}
