use crate::{line::Line, point::Point};

#[derive(Debug, Clone)]
pub struct Event {
    pub point: Point,
    pub event_type: EventType,
    pub line: Option<Line>,
    pub other: Option<Line>,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
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

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }
}

impl Eq for Event {}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let point_cmp = self.point.cmp(&other.point);
        match point_cmp {
            std::cmp::Ordering::Equal => match (self.event_type, other.event_type) {
                (EventType::Intersection { .. }, _) => std::cmp::Ordering::Less,
                (EventType::End { .. }, _) => std::cmp::Ordering::Greater,
                _ => std::cmp::Ordering::Equal,
            },
            _ => point_cmp,
        }
    }
}