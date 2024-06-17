use crate::point::Point;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Line {
        Line { start: start, end: end }
    }

    // Funktion um y zu berechnen mit x Wert
    pub fn y(&self, x: f64) -> f64 {
        let m = (self.end.y - self.start.y) / (self.end.x - self.start.x);
        let b = self.start.y - m * self.start.x;
        m * x + b
    }

    // Intersection
    pub fn intersection(&self, other: Line) -> Option<Point> {
        let x1 = self.start.x;
        let y1 = self.start.y;
        let x2 = self.end.x;
        let y2 = self.end.y;

        let x3 = other.start.x;
        let y3 = other.start.y;
        let x4 = other.end.x;
        let y4 = other.end.y;

        let d = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        if d == 0.0 {
            return None;
        }

        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / d;
        let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / d;

        if t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0 {
            let x = x1 + t * (x2 - x1);
            let y = y1 + t * (y2 - y1);
            Some(Point::new(x, y))
        } else {
            None
        }
    }
}