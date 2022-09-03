use crate::math::Rect;

pub trait Contains<T> {
    fn contains(&self, x: T, y: T) -> bool;
}

impl<U, T: PartialOrd<U>> Contains<T> for Rect<U> {
    #[inline]
    fn contains(&self, x: T, y: T) -> bool {
        x >= self.x1 && x < self.x2 && y >= self.y1 && y < self.y2
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
