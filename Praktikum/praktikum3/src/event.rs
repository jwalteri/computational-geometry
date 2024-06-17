use crate::{line::Line, point::Point};


pub struct Event {
    pub point: Point,
    pub event_type: EventType,
    pub line: Option<Line>,
    pub other: Option<Line>,
}

pub enum EventType {
    Start,
    End,
    Intersection,
}

impl Event {
    pub fn new(point: Point, event_type: EventType, line: Option<Line>, other: Option<Line>) -> Event {
        Event {
            point: point,
            event_type: event_type,
            line: line,
            other: other,
        }
    }
}

impl std::cmp::PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Ordne nach x-Koordinate aufsteigend
        if self.point.x < other.point.x {
            Some(std::cmp::Ordering::Greater)//Less
        } else if self.point.x > other.point.x {
            Some(std::cmp::Ordering::Less)//Greater
        } else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}


impl Ord for Event {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.point.x < other.point.x {
            std::cmp::Ordering::Greater//Less
        } else if self.point.x > other.point.x {
            std::cmp::Ordering::Less//Greater
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

impl Eq for Event {}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }
}



// Unit Test
#[cfg(test)]
mod tests {
    use std::collections::BinaryHeap;

    use super::*;

    #[test]
    fn test_event() {
        let p1 = Point::new(1.0, 1.0);
        let p2 = Point::new(2.0, 1.0);
        let p3 = Point::new(3.0, 1.0);
        let l1 = Line::new(Point::new(1.0, 1.0), Point::new(2.0, 2.0));
        let l2 = Line::new(Point::new(2.0, 2.0), Point::new(3.0, 3.0));
        let l3 = Line::new(Point::new(2.0, 2.0), Point::new(3.0, 3.0));
        let e1 = Event::new(p3, EventType::Start, Some(l1), None);
        let e2 = Event::new(p2, EventType::Start, Some(l2), None);
        let e3 = Event::new(p1, EventType::Start, Some(l3), None);

        let mut events = BinaryHeap::new();
        events.push(e3);
        events.push(e1);
        events.push(e2);

        assert_eq!(events.pop().unwrap().point, p1);
        assert_eq!(events.pop().unwrap().point, p2);
        assert_eq!(events.pop().unwrap().point, p3);
    }
}