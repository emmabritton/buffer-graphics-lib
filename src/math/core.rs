use crate::math::Rect;
use mint::Point2;
use std::ops::{Add, Sub};

impl<T: Add<Output = T> + Copy> Rect<T> {
    pub const fn new(x1: T, y1: T, x2: T, y2: T) -> Self {
        Rect { x1, y1, x2, y2 }
    }

    pub fn from_point(point: Point2<T>, w: T, h: T) -> Self {
        Rect {
            x1: point.x,
            y1: point.y,
            x2: point.x + w,
            y2: point.y + h,
        }
    }
}

impl<T: Copy + Add<Output = T> + Sub<Output = T> + Ord + Default> Rect<T> {
    pub fn width(&self) -> T {
        self.x2 - self.x1
    }

    pub fn height(&self) -> T {
        self.y2 - self.y1
    }

    pub fn clip_to_positive(&self) -> Rect<T> {
        Rect::new(
            self.x1.max(T::default()),
            self.y1.max(T::default()),
            self.x2.max(T::default()),
            self.y2.max(T::default()),
        )
    }

    pub fn move_to(&self, x: T, y: T) -> Rect<T> {
        let w = self.width();
        let h = self.height();
        Rect::new(x, y, x + w, y + h)
    }

    pub fn translate(&self, x: T, y: T) -> Rect<T> {
        Rect::new(self.x1 + x, self.y1 + y, self.x2 + x, self.y2 + y)
    }

    pub fn topleft(&self) -> Point2<T> {
        Point2 {
            x: self.x1,
            y: self.y1,
        }
    }

    pub fn bottomright(&self) -> Point2<T> {
        Point2 {
            x: self.x2,
            y: self.y2,
        }
    }

    /// Union this rect and another, the result will contain both rectangles
    /// Generally, this means the result will be bigger than self
    pub fn union(&self, other: &Rect<T>) -> Rect<T> {
        let x1 = self.x1.min(other.x1);
        let y1 = self.y1.min(other.y1);
        let x2 = self.x2.max(other.x2);
        let y2 = self.y2.max(other.y2);

        Rect::new(x1, y1, x2, y2)
    }

    /// Intersect this rect and another, the result will contain the area covered by both rectangles
    /// Generally, this means the result will be smaller than self
    ///
    /// # Returns
    ///
    /// self if rectangles do not intersect
    pub fn intersect(&self, other: &Rect<T>) -> Rect<T> {
        let x1 = self.x1.max(other.x1);
        let y1 = self.y1.max(other.y1);
        let x2 = self.x2.min(other.x2);
        let y2 = self.y2.min(other.y2);
        if x1 < x2 && y1 < y2 {
            Rect::new(x1, y1, x2, y2)
        } else {
            *self
        }
    }

    pub fn intersects(&self, other: &Rect<T>) -> bool {
        let x1 = self.x1.max(other.x1);
        let y1 = self.y1.max(other.y1);
        let x2 = self.x2.min(other.x2);
        let y2 = self.y2.min(other.y2);

        x1 < x2 && y1 < y2
    }
}

#[cfg(test)]
mod test {
    mod rect {
        use crate::math::Rect;

        #[test]
        fn width() {
            let rect = Rect::new(10, 10, 20, 20);
            assert_eq!(rect.width(), 10);
        }

        #[test]
        fn height() {
            let rect = Rect::new(10, 10, 20, 20);
            assert_eq!(rect.height(), 10);
        }

        #[test]
        fn translate() {
            let rect = Rect::new(10, 10, 20, 20);
            let translated = rect.translate(5, 5);
            assert_eq!(translated, Rect::new(15, 15, 25, 25));

            let rect = Rect::new(10, 10, 20, 20);
            let translated = rect.translate(-5, -5);
            assert_eq!(translated, Rect::new(5, 5, 15, 15));
        }

        #[test]
        fn union() {
            let rect = Rect::new(10, 10, 20, 20);
            let other = Rect::new(15, 15, 25, 25);

            let union = rect.union(&other);

            assert_eq!(union, Rect::new(10, 10, 25, 25));

            let rect = Rect::new(50, 1, 50, 100);
            let other = Rect::new(1, 50, 100, 50);

            let union = rect.union(&other);

            assert_eq!(union, Rect::new(1, 1, 100, 100));
        }

        #[test]
        fn intersects() {
            let rect = Rect::new(10, 10, 20, 20);
            let does_intersect = Rect::new(15, 15, 25, 25);
            let doesnt_intersect = Rect::new(30, 30, 40, 40);

            assert!(rect.intersects(&does_intersect));
            assert!(!rect.intersects(&doesnt_intersect));
        }

        #[test]
        fn intersect() {
            let rect = Rect::new(10, 10, 20, 20);
            let other = Rect::new(15, 15, 25, 25);

            let intersection = rect.intersect(&other);

            assert_eq!(intersection, Rect::new(15, 15, 20, 20));

            let rect = Rect::new(50, 1, 51, 100);
            let other = Rect::new(1, 50, 100, 51);

            let intersection = rect.intersect(&other);

            assert_eq!(intersection, Rect::new(50, 50, 51, 51));

            let rect = Rect::new(10, 10, 20, 20);
            let doesnt_intersect = Rect::new(30, 30, 40, 40);

            assert_eq!(rect.intersect(&doesnt_intersect), rect);
        }
    }
}
