use crate::{line::Line, point::Point};


pub struct SweepLine {
    segments: Vec<SweepLineEntry>,
}

#[derive(Debug, Clone)]
pub struct SweepLineEntry {
    pub line: Line,
    pub point: Point,
}

impl SweepLineEntry {
    pub fn new(point: Point, line: Line) -> SweepLineEntry {
        SweepLineEntry {
            line: line,
            point: point,
        }
    }
}

impl SweepLine {
    pub fn new() -> SweepLine {
        SweepLine {
            segments: Vec::new(),
        }
    }

    pub fn add_line(&mut self, point: Point, entry: Line) {
        let new_entry = SweepLineEntry::new(point, entry);
        self.segments.push(new_entry);
        self.sort_y();
    }

    pub fn sort_y(&mut self) {
        // Sort the segments by their points y-coordinate. biggest y-coordinate first
        self.segments.sort_by(|a, b| b.point.y.partial_cmp(&a.point.y).unwrap());
    }


    pub fn remove_line(&mut self, entry: Line) -> SweepLineEntry {
        let index = self.segments.iter().position(|x| x.line == entry).unwrap();
        self.segments.remove(index)
    }

    pub fn pinpoint_points(&mut self, point: Point) {
        let x_value = point.x;

        let mut new_sweep_line = Vec::new();

        // Update all y-values of the segments
        for segment in &self.segments {
            let y_value = segment.line.y(x_value);
            new_sweep_line.push(SweepLineEntry::new(Point::new(x_value, y_value), segment.line));
        }

        self.segments = new_sweep_line;
        self.sort_y();
    } 

    pub fn swap_lines(&mut self, line1: Line, line2: Line, point: Point) -> (Line, Line) {
        let entry1 = self.remove_line(line1);
        let entry2 = self.remove_line(line2);

        let epsilon = 1e-9;

        let new_y1 = entry1.line.y(point.x + epsilon);
        let new_entry1 = SweepLineEntry::new(Point::new(point.x, new_y1), entry1.line);
        let new_y2 = entry2.line.y(point.x + epsilon);
        let new_entry2 = SweepLineEntry::new(Point::new(point.x, new_y2), entry2.line);

        self.segments.push(new_entry1.clone());
        self.segments.push(new_entry2.clone());
        self.sort_y();
        
        // Return the highest y-value first
        // if new_entry1.point.y > new_entry2.point.y {
        //     return (new_entry1.line, new_entry2.line)
        // } else {
        //     return (new_entry2.line, new_entry1.line)
        // }

        (new_entry1.line, new_entry2.line)
    }

    // Get neighbors
    pub fn get_neighbors(&self, line: Line) -> (Option<&SweepLineEntry>, Option<&SweepLineEntry>) {
        let index = self.segments.iter().position(|x| x.line == line).unwrap();
        let mut left = None;
        let mut right = None;

        if index > 0 {
            left = Some(&self.segments[index - 1]);
        }

        if index < self.segments.len() - 1 {
            right = Some(&self.segments[index + 1]);
        }

        (left, right)
    }
}


// Unit Test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sweepline() {
        let mut b = SweepLine::new();
        let p1 = Point::new(1.0, 1.0);
        let p2 = Point::new(2.0, 2.0);
        let p3 = Point::new(3.0, 3.0);
        let l1 = Line::new(p1, p2);
        let l2 = Line::new(p2, p3);
        b.add_line(p1, l1);
        b.add_line(p2, l2);

        assert_eq!(b.segments.len(), 2);
        
        // Prüfe reihenfolge
        assert_eq!(b.segments[0].point, p2);
        assert_eq!(b.segments[1].point, p1);
    }
}