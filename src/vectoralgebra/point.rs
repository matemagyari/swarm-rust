#[derive(Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

const ZERO: Point = Point { x: 0.0, y: 0.0};

impl Point {

    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    pub fn distance(&self, p: &Point) -> f64 {
        ((self.x-p.x).powf(2.0) + (self.y-p.y).powf(2.0)).sqrt()
    }

    pub fn weight_point(ps: Vec<Point>) -> Point {

        let add = |p1: Point, p2: &Point| {
            Point { x: p1.x + p2.x, y: p1.y + p2.y }
        };

        let p = ps.iter().fold(ZERO, add);

        let norm = |x: f64| x / ps.len() as f64;

        Point::new(norm(p.x) , norm(p.y))
    }

}