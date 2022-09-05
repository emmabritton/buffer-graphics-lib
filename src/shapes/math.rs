/// Union this rect and another, the result will contain both rectangles
/// Generally, this means the result will be bigger than self
pub fn union(&self, other: &Rect) -> Rect {
    let x1 = self.start.x.min(other.start.x);
    let y1 = self.start.y.min(other.start.y);
    let x2 = self.end.x.max(other.end.x);
    let y2 = self.end.y.max(other.end.y);

    Rect::new((x1, y1), (x2, y2), self.draw_type)
}

/// Intersect this rect and another, the result will contain the area covered by both rectangles
/// Generally, this means the result will be smaller than self
///
/// # Returns
///
/// self if rectangles do not intersect
pub fn intersect(&self, other: &Rect) -> Rect {
    let x1 = self.start.x.max(other.start.x);
    let y1 = self.start.y.max(other.start.y);
    let x2 = self.end.x.min(other.end.x);
    let y2 = self.end.y.min(other.end.y);
    if x1 < x2 && y1 < y2 {
        Rect::new((x1, y1), (x2, y2), self.draw_type)
    } else {
        self.clone()
    }
}

pub fn intersects(&self, other: &Rect) -> bool {
    let x1 = self.start.x.max(other.start.x);
    let y1 = self.start.y.max(other.start.y);
    let x2 = self.end.x.min(other.end.x);
    let y2 = self.end.y.min(other.end.y);

    x1 < x2 && y1 < y2
}