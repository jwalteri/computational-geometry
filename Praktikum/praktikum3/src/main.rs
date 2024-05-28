use std::collections::{BTreeSet, BinaryHeap};
use std::cmp::Ordering;

// Repräsentiert einen Punkt im 2D-Raum
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

// Repräsentiert ein Liniensegment durch zwei Punkte
#[derive(Debug, Clone, Copy)]
struct LineSegment {
    start: Point,
    end: Point,
}

impl LineSegment {
    fn contains(&self, point: &Point) -> bool {
        // Überprüfen, ob der Punkt auf dem Liniensegment liegt
        let cross_product = (point.y - self.start.y) * (self.end.x - self.start.x) - (point.x - self.start.x) * (self.end.y - self.start.y);
        if cross_product.abs() > std::f64::EPSILON {
            return false;
        }
        let dot_product = (point.x - self.start.x) * (self.end.x - self.start.x) + (point.y - self.start.y) * (self.end.y - self.start.y);
        if dot_product < 0.0 {
            return false;
        }
        let squared_length = (self.end.x - self.start.x) * (self.end.x - self.start.x) + (self.end.y - self.start.y) * (self.end.y - self.start.y);
        if dot_product > squared_length {
            return false;
        }
        true
    }

    fn intersect(&self, other: &LineSegment) -> Option<Point> {
        let LineSegment { start: p1, end: p2 } = self;
        let LineSegment { start: p3, end: p4 } = other;

        let d = (p4.y - p3.y) * (p2.x - p1.x) - (p4.x - p3.x) * (p2.y - p1.y);
        if d.abs() < std::f64::EPSILON {
            return None;
        }

        let u = ((p4.x - p3.x) * (p1.y - p3.y) - (p4.y - p3.y) * (p1.x - p3.x)) / d;
        let v = ((p2.x - p1.x) * (p1.y - p3.y) - (p2.y - p1.y) * (p1.x - p3.x)) / d;

        if u < 0.0 || u > 1.0 || v < 0.0 || v > 1.0 {
            return None;
        }

        Some(Point {
            x: p1.x + u * (p2.x - p1.x),
            y: p1.y + u * (p2.y - p1.y),
        })
    }
}

// Repräsentiert ein Ereignis
#[derive(Debug, Clone)]
enum EventType {
    Start,
    End,
    Intersection(Point),
}

#[derive(Debug, Clone)]
struct Event {
    point: Point,
    event_type: EventType,
    segment: Option<LineSegment>,
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        other.point.x.partial_cmp(&self.point.x).unwrap().then_with(|| {
            match (&self.event_type, &other.event_type) {
                (EventType::Intersection(_), EventType::Intersection(_)) => Ordering::Equal,
                (EventType::Intersection(_), _) => Ordering::Greater,
                (_, EventType::Intersection(_)) => Ordering::Less,
                (_, _) => Ordering::Equal,
            }
        })
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }
}

impl Eq for Event {}

impl Iterator for Point {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        // Implement the logic to iterate over the coordinates of the point
        unimplemented!()
    }
}

impl Ord for LineSegment {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare the start points of the line segments
        self.start.x.partial_cmp(&other.start.x).unwrap().then_with(|| {
            self.start.y.partial_cmp(&other.start.y).unwrap()
        })
    }
}

impl PartialOrd for LineSegment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

    impl PartialEq for LineSegment {
        fn eq(&self, other: &Self) -> bool {
            self.start == other.start
        }
    }

    impl Eq for LineSegment {}

fn main() {
    let segments = vec![
        LineSegment { start: Point { x: 0.0, y: 0.0 }, end: Point { x: 4.0, y: 4.0 } },
        LineSegment { start: Point { x: 0.0, y: 4.0 }, end: Point { x: 4.0, y: 0.0 } },
        LineSegment { start: Point { x: 0.62, y: 2.14 }, end: Point { x: 0.56, y: 1.06 } },
        LineSegment { start: Point { x: 3.9, y: 3.32 }, end: Point { x: 3.16, y: 1.24 } },
        LineSegment { start: Point { x: -1.0, y: 1.0 }, end: Point { x: 5.0, y: 3.0 } },
    ];

    let mut events = BinaryHeap::new();

    // Ereignisse erzeugen
    for segment in &segments {
        events.push(Event {
            point: segment.start,
            event_type: EventType::Start,
            segment: Some(*segment),
        });
        events.push(Event {
            point: segment.end,
            event_type: EventType::End,
            segment: Some(*segment),
        });
        // Ausgabe, dass Start und End Events hinzugefügt wurden
        println!("Start event at (x: {:.2}, y: {:.2})", segment.start.x, segment.start.y);
        println!("End event at (x: {:.2}, y: {:.2})\n", segment.end.x, segment.end.y);
    }    

    let mut sweep_line = BTreeSet::new();
    let mut intersections = Vec::new();

    while let Some(event) = events.pop() {
        match event.event_type {
            EventType::Start => {
                if let Some(segment) = event.segment {
                    // Füge das Segment zur Sweep Line hinzu
                    sweep_line.insert(segment);

                    // Prüfe auf Schnittpunkte mit benachbarten Segmenten
                    for neighbor in sweep_line.range(..segment).rev().take(1).chain(sweep_line.range(segment..).skip(1).take(1)) {
                        if let Some(point) = segment.intersect(neighbor) {
                            events.push(Event {
                                point,
                                event_type: EventType::Intersection(point),
                                segment: None,
                            });
                        }
                    }
                }
            }
            // EventType Delete
            /*EventType::Delete => {



            }
            */
            EventType::End => {
                if let Some(segment) = event.segment {
                    // Entferne das Segment aus der Sweep Line
                    sweep_line.remove(&segment);

                    // Prüfe auf Schnittpunkte zwischen den benachbarten Segmenten, die jetzt direkt nebeneinander liegen
                    let prev = sweep_line.range(..segment).rev().take(1).next().cloned();
                    let next = sweep_line.range(segment..).skip(1).take(1).next().cloned();

                    if let (Some(prev), Some(next)) = (prev, next) {
                        if let Some(point) = prev.intersect(&next) {
                            events.push(Event {
                                point,
                                event_type: EventType::Intersection(point),
                                segment: None,
                            });
                        }
                    }
                }
            }
            EventType::Intersection(point) => {
                intersections.push(point);
            }
        }
    }


    // Zeilenweise Ausgabe der Schnittpunkte
    for point in intersections {
        println!("Intersection at (x: {:.2}, y: {:.2})\n", point.x, point.y);
    }
}
