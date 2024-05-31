use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

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

impl PartialEq for LineSegment {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

#[derive(Debug, Clone)]
struct Event {
    point: Point,
    event_type: String,
    segment: LineSegment,
    other: Option<LineSegment>,
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        //println!("BLIB");
        if self.point.x == other.point.x && self.point.y == other.point.y {
            return Ordering::Equal;
        } else

        if self.point.x > other.point.x {
            return Ordering::Greater;
        } else 
        {
            return Ordering::Less;
        }
    }
}

impl Eq for Event {}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.point.x == other.point.x && self.point.y == other.point.y {
            ////println!("{} is equal to {}", self.point.x, other.point.x);
            return Some(Ordering::Equal);
        } else

        if self.point.x > other.point.x {
            ////println!("{} is greater than {}", self.point.x, other.point.x);
            //return Some(Ordering::Greater);
            return Some(Ordering::Less);
        } else 
        {
            ////println!("{} is less than {}", self.point.x, other.point.x);
            //return Some(Ordering::Less);
            return Some(Ordering::Greater);
        }    
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }
}

fn contains_event(heap: &BinaryHeap<Event>, event: &Event) -> bool {
    heap.iter().any(|e| e == event)
}

fn main() {
    let segments = vec![
        LineSegment { start: Point { x: -1.0, y: 1.0 }, end: Point { x: 5.0, y: 3.0 } },
        LineSegment { start: Point { x: 0.0, y: 0.0 }, end: Point { x: 4.0, y: 4.0 } },
        LineSegment { start: Point { x: 0.0, y: 4.0 }, end: Point { x: 4.0, y: 0.0 } },
        LineSegment { start: Point { x: 0.3, y: 2.0 }, end: Point { x: 0.6, y: 1.0 } },
        LineSegment { start: Point { x: 3.16, y: 1.24 }, end: Point { x: 3.9, y: 3.32 } },
        LineSegment { start: Point { x: 2.0, y: 1.5 }, end: Point { x: 4.0, y: 1.5 } },
        LineSegment { start: Point { x: 0.0, y: 4.0 }, end: Point { x: 0.0, y: 0.0 } },
        LineSegment { start: Point { x: -0.5, y: 4.0 }, end: Point { x: 0.5, y: 4.0 } },
    ];

    let mut events = BinaryHeap::new();

    for segment in segments {
        events.push(Event {
            point: segment.start,
            event_type: "Start".to_owned(),
            segment: segment,
            other: None
        });
        events.push(Event {
            point: segment.end,
            event_type: "End".to_owned(),
            segment: segment,
            other: None
        });
    }

    let mut sweep_line = Vec::new();

    let mut intersections = Vec::new();

    while let Some(event) = events.pop() {
        match event.event_type.as_str() {
            "Start" => {
                sweep_line.push(event.segment);

                if sweep_line.len() > 1 {
                    let pred = sweep_line.get(sweep_line.len() - 2);

                    if let Some(pred) = pred {
                        if let Some(point) = event.segment.intersect(pred) {
                            events.push(Event {
                                point,
                                event_type: "Intersection".to_owned(),
                                segment: event.segment,
                                other: Some(pred.clone()),
                            });
                        }
                    }
                }
            }


            "End" => {
                if let Some(index) = sweep_line.iter().position(|x| x == &event.segment) {
                    sweep_line.remove(index);
                }

                // Finden von above und below
                let index = sweep_line.iter().position(|&x| x == event.segment);
                let above = index.and_then(|i| sweep_line.get(i + 1));
                let below = index.and_then(|i| sweep_line.get(i - 1));

                // Prüfe auf Schnittpunkte mit above und below, wenn above und below existieren
                if let (Some(above), Some(below)) = (above, below) {
                    if let Some(point) = below.intersect(above) {
                        // Wenn Event noch nicht in events ist, füge es hinzu

                        let new_event = Event {
                            point,
                            event_type: "Intersection".to_owned(),
                            segment: *below,
                            other: Some(*above)
                        };

                        if !contains_event(&events, &new_event) && new_event != event {
                            events.push(new_event);
                        }
                    }
                }
            }
            "Intersection" => {
                intersections.push(event.point);

                 // Tausche Position von segA und segB in der Sweep Line
                 let indexA = sweep_line.iter().position(|&x| x == event.segment);
                 let indexB = sweep_line.iter().position(|&x| x == event.other.unwrap());

                 if let (Some(indexA), Some(indexB)) = (indexA, indexB) {
                    sweep_line.swap(indexA, indexB);

                    if (sweep_line.len() - 2) <= indexB {
                        let segA = sweep_line.get(indexB + 1);
                        if let Some(segA) = segA {
                            if let Some(point) = segA.intersect(&event.other.unwrap()) {

                                let new_event = Event {
                                    point,
                                    event_type: "Intersection".to_owned(),
                                    segment: *segA,
                                    other: Some(event.segment)
                                };

                                if !contains_event(&events, &new_event) && new_event != event {
                                    events.push(new_event);
                                }
                            }
                        }
                    }

                    if indexA - 1 > 0 {
                        let segB = sweep_line.get(indexA - 1);

                        if let Some(segB) = segB {
                            if let Some(point) = segB.intersect(&event.segment) {
    
                                let new_event = Event {
                                    point,
                                    event_type: "Intersection".to_owned(),
                                    segment: *segB,
                                    other: Some(event.other.unwrap())
                                };
    
                                if !contains_event(&events, &new_event) && new_event != event {
                                    events.push(new_event);
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }

    }

    println!("Number of intersections: {}", intersections.len());
    for intersection in intersections {
        println!("Intersection at ({}, {})", intersection.x, intersection.y);
    }

}


