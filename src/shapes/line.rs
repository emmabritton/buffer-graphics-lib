use crate::coord::Coord;

pub fn line_contains(start: Coord, end: Coord, point: Coord) -> bool {
    if start.x == end.x && start.x == point.x {
        start.y <= point.y && point.y <= end.y
    } else if start.y == end.y && start.y == point.y {
        start.x <= point.x && point.x <= end.x
    } else {
        start.distance(point) + end.distance(point) ==
            start.distance(end)
    }
}
