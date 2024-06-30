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

    pub fn intersection(&self, other: &Line) -> Option<Point> {
        let p1 = &self.start;
        let p2 = &self.end;
        let q1 = &other.start;
        let q2 = &other.end;

        // Orientierung von q1 zu Linie p1p2
        let q1_to_p1p2 = ccw(p1, p2, q1);

        // Orientierung von q2 zu Linie p1p2
        let q2_to_p1p2 = ccw(p1, p2, q2);

        // Wenn beide Orientierungen das gleiche Vorzeichen haben,
        // dann liegen q1 und q2 auf der gleichen Seite der Linie p1p2
        // => Die Linien können sich nicht schneiden
        if q1_to_p1p2 * q2_to_p1p2 > 0.0 {
            return None;
        }

        // Orientierung von p1 zu Linie q1q2
        let p1_to_q1q2 = ccw(q1, q2, p1);

        // Orientierung von p2 zu Linie q1q2
        let p2_to_q1q2 = ccw(q1, q2, p2);

        // Gleiches Prinzip wie oben
        if p1_to_q1q2 * p2_to_q1q2 > 0.0 {
            return None;
        }

        // Verhältnis CCW-Werte
        // -> Bestimmt Anteil der Strecke q1q2 für Schnittpunktberechnung
        let ratio = (q2_to_p1p2 / q1_to_p1p2).abs();

        // Faktor a für Berechnung des Schnittpunktes
        // Normalisiert ratio auf 0..1
        let a = ratio / (ratio + 1.0);

        // Berechnung des Schnittpunktes
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