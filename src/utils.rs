use sdl2::rect::Point;

pub struct BezierPoint {
    pub location: Point,
    pub pressed: bool,
}

pub struct BezierPoints {
    pub start: BezierPoint,
    pub end: BezierPoint,
    pub control: BezierPoint,
}

pub fn get_distance_between_points(p1: &Point, p2: &Point) -> i32 {
    (((p2.x - p1.x) * (p2.x - p1.x) + (p2.y - p1.y) * (p2.y - p1.y)) as f32).sqrt() as i32
}