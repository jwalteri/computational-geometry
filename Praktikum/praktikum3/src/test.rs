use std::collections::BTreeSet;
use std::cmp::Ordering;

use ordered_float::OrderedFloat;

use crate::point;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Point {
    x: OrderedFloat<f64>,
    y: OrderedFloat<f64>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(start: Point, end: Point) -> Self {
        Line { start, end }
    }

    pub fn y(&self, x: OrderedFloat<f64>) -> OrderedFloat<f64> {
        let m = (self.end.y - self.start.y) / (self.end.x - self.start.x);
        let b = self.start.y - m * self.start.x;
        m * x + b
    }
}

impl Ord for Line {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.y.partial_cmp(&other.start.y).unwrap()
            .then_with(|| self.start.x.partial_cmp(&other.start.x).unwrap())
    }
}

impl PartialOrd for Line {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct LineCollection {
    lines: BTreeSet<Line>,
}

impl LineCollection {
    fn new() -> Self {
        LineCollection { lines: BTreeSet::new() }
    }

    fn add_line_by_points(&mut self, start: Point, end: Point) {
        self.lines.insert(Line { start, end });
    }

    fn add_line(&mut self, line: Line) {
        self.lines.insert(line);
    }

    fn remove_line(&mut self, start: &Point, end: &Point) -> bool {
        self.lines.remove(&Line { start: start.clone(), end: end.clone() })
    }

    fn find_neighbors(&self, line: &Line) -> (Option<&Line>, Option<&Line>) {
        let mut before = None;
        let mut after = None;
        for l in self.lines.range(..line) {
            before = Some(l);
        }
        for l in self.lines.range(line..) {
            if l != line {
                after = Some(l);
                break;
            }
        }
        (before, after)
    }

    fn swap_lines(&mut self, line1: &Line, line2: &Line, x_value: OrderedFloat<f64>) -> (Line, Line) {
        self.lines.remove(line1);
        self.lines.remove(line2);

        let epsilon = 1e-9;

        let new_X = x_value + epsilon;
        let new_y1 = line1.y(new_X);
        let new_y2 = line2.y(new_X);

        let line1 = Line::new(Point { x: new_X, y: new_y1 }, line1.end.clone());
        let line2 = Line::new(Point { x: new_X, y: new_y2 }, line2.end.clone());

        self.lines.insert(line1.clone());
        self.lines.insert(line2.clone());

        (line1, line2)
    }
}


// Unit test
#[cfg(test)]
mod tests {
    use std::mem::swap;

    use super::*;

    #[test]
    fn test_find_neighbors() {
        let mut collection = LineCollection::new();
        let line1 = Line { start: Point { x: OrderedFloat(2.0), y: OrderedFloat(1.0) }, end: Point { x: OrderedFloat(5.0), y: OrderedFloat(7.0) } };
        let line2 = Line { start: Point { x: OrderedFloat(3.0), y: OrderedFloat(2.0) }, end: Point { x: OrderedFloat(6.0), y: OrderedFloat(8.0) } };
        let line3 = Line { start: Point { x: OrderedFloat(4.0), y: OrderedFloat(3.0) }, end: Point { x: OrderedFloat(7.0), y: OrderedFloat(9.0) } };
    
        collection.add_line(line3.clone());
        collection.add_line(line1.clone());
        collection.add_line(line2.clone());

        let (before, after) = collection.find_neighbors(&line2);

        assert_eq!(before, Some(&line1));
        assert_eq!(after, Some(&line3));
    }

    #[test]
    fn test_find_neighbors2() {
        let mut collection = LineCollection::new();
        let line0 = Line { start: Point { x: OrderedFloat(4.0), y: OrderedFloat(0.0) }, end: Point { x: OrderedFloat(7.0), y: OrderedFloat(9.0) } };
        let line1 = Line { start: Point { x: OrderedFloat(2.0), y: OrderedFloat(1.0) }, end: Point { x: OrderedFloat(5.0), y: OrderedFloat(8.0) } };
        let line2 = Line { start: Point { x: OrderedFloat(3.0), y: OrderedFloat(2.0) }, end: Point { x: OrderedFloat(6.0), y: OrderedFloat(7.0) } };
        let line3 = Line { start: Point { x: OrderedFloat(4.0), y: OrderedFloat(3.0) }, end: Point { x: OrderedFloat(7.0), y: OrderedFloat(16.0) } };
        let line4 = Line { start: Point { x: OrderedFloat(4.0), y: OrderedFloat(4.0) }, end: Point { x: OrderedFloat(7.0), y: OrderedFloat(15.0) } };
        let line5 = Line { start: Point { x: OrderedFloat(4.0), y: OrderedFloat(5.0) }, end: Point { x: OrderedFloat(7.0), y: OrderedFloat(14.0) } };
        let line6 = Line { start: Point { x: OrderedFloat(4.0), y: OrderedFloat(6.0) }, end: Point { x: OrderedFloat(7.0), y: OrderedFloat(29.0) } };
        let line7 = Line { start: Point { x: OrderedFloat(4.0), y: OrderedFloat(7.0) }, end: Point { x: OrderedFloat(7.0), y: OrderedFloat(329.0) } };
        let line8 = Line { start: Point { x: OrderedFloat(4.0), y: OrderedFloat(8.0) }, end: Point { x: OrderedFloat(7.0), y: OrderedFloat(1239.0) } };
        let line9 = Line { start: Point { x: OrderedFloat(4.0), y: OrderedFloat(9.0) }, end: Point { x: OrderedFloat(7.0), y: OrderedFloat(4329.0) } };
    
        collection.add_line(line0.clone());
        collection.add_line(line1.clone());
        collection.add_line(line2.clone());
        collection.add_line(line3.clone());
        collection.add_line(line4.clone());
        collection.add_line(line5.clone());
        collection.add_line(line6.clone());
        collection.add_line(line7.clone());
        collection.add_line(line8.clone());
        collection.add_line(line9.clone());

        let (before, after) = collection.find_neighbors(&line5);

        assert_eq!(before, Some(&line4));
        assert_eq!(after, Some(&line6));
    }

    #[test]
    fn test_find_neighbors3() {
        let mut collection = LineCollection::new();
        let line1 = Line { start: Point { x: OrderedFloat(2.0), y: OrderedFloat(1.0) }, end: Point { x: OrderedFloat(5.0), y: OrderedFloat(7.0) } };
        let line2 = Line { start: Point { x: OrderedFloat(3.0), y: OrderedFloat(2.0) }, end: Point { x: OrderedFloat(6.0), y: OrderedFloat(8.0) } };
        let line3 = Line { start: Point { x: OrderedFloat(4.0), y: OrderedFloat(3.0) }, end: Point { x: OrderedFloat(7.0), y: OrderedFloat(9.0) } };
    
        collection.add_line(line3.clone());
        collection.add_line(line1.clone());
        collection.add_line(line2.clone());

        let (before, after) = collection.find_neighbors(&line1);

        assert_eq!(before, None);
        assert_eq!(after, Some(&line2));

        let line4 = Line { start: Point { x: OrderedFloat(4.0), y: OrderedFloat(0.0) }, end: Point { x: OrderedFloat(7.0), y: OrderedFloat(9.0) } };
        collection.add_line(line4.clone());

        let (before, after) = collection.find_neighbors(&line1);

        assert_eq!(before, Some(&line4));
        assert_eq!(after, Some(&line2));
    }

    #[test]
    fn test_find_neighbors4() {
        let mut collection = LineCollection::new();
        let line1 = Line { start: Point { x: OrderedFloat(2.0), y: OrderedFloat(1.0) }, end: Point { x: OrderedFloat(5.0), y: OrderedFloat(7.0) } };
        let line2 = Line { start: Point { x: OrderedFloat(3.0), y: OrderedFloat(2.0) }, end: Point { x: OrderedFloat(6.0), y: OrderedFloat(8.0) } };
        let line3 = Line { start: Point { x: OrderedFloat(4.0), y: OrderedFloat(3.0) }, end: Point { x: OrderedFloat(7.0), y: OrderedFloat(9.0) } };
    
        collection.add_line(line3.clone());
        collection.add_line(line1.clone());
        collection.add_line(line2.clone());

        let (line1, line2) = collection.swap_lines(&line1, &line2, OrderedFloat(2.5));

        let (before, after) = collection.find_neighbors(&line1);

        assert_eq!(before, Some(&line2));
        assert_eq!(after, Some(&line3));
    }
}