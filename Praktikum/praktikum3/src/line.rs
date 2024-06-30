use crate::point::Point;


#[derive(Debug, Clone)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Eq for Line {}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

#[derive(Debug, Clone)]
pub struct SortableLine {
    pub line: Line,
    pub index: f64,
}

impl PartialEq for SortableLine {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}
impl Eq for SortableLine {}

impl PartialOrd for SortableLine {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                other.index.partial_cmp(&self.index)
    }
}

impl Ord for SortableLine {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        f64::total_cmp(&other.index, &self.index)
    }
}

fn ccw(p: &Point, q: &Point, r: &Point) -> f64 {
    (p.x * q.y - p.y * q.x) + (q.x * r.y - q.y * r.x) + (p.y * r.x - p.x * r.y)
}

impl Line {
    pub fn len(&self) -> f64 {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        f64::sqrt(dx * dx + dy * dy)
    }

    pub fn intersection(&self, other: &Line) -> Option<Point> {
        let p1 = &self.start;
        let p2 = &self.end;
        let q1 = &other.start;
        let q2 = &other.end;

        let ccwq1 = ccw(p1, p2, q1);
        let ccwq2 = ccw(p1, p2, q2);
        if ccwq1 * ccwq2 > 0.0 {
            return None;
        }

        let ccwp1 = ccw(q1, q2, p1);
        let ccwp2 = ccw(q1, q2, p2);
        if ccwp1 * ccwp2 > 0.0 {
            return None;
        }

        let r_ab = (ccwq2 / ccwq1).abs();
        let a = r_ab / (r_ab + 1.0);
        let i_x = q2.x + a * (q1.x - q2.x);
        let i_y = q2.y + a * (q1.y - q2.y);

        Some(Point { x: i_x, y: i_y })
    }

    // Updaten der y-Koordinate abhängig von einem X
    pub fn y(&self, x: f64) -> f64 {
       let m = (self.start.y - self.end.y) / (self.start.x - self.end.x);

        m * (x - self.start.x) + self.start.y
    }
}