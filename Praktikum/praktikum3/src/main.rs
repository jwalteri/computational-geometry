
use std::collections::BinaryHeap;

use praktikum3::{event::Event, event::EventType, line::Line, point::Point, sweepline::SweepLine};

// main function
fn main() {
    println!("Hello, world!");

    let segments = vec![
        Line { start: Point { x: -1.0, y: 1.0 }, end: Point { x: 5.0, y: 1.0 }},
        Line { start: Point { x: 1.0, y: 0.0 }, end: Point { x: 4.0, y: 4.0 }},
        Line { start: Point { x: 2.0, y: 4.0 }, end: Point { x: 5.0, y: 0.0 }},
    ];

    let mut events = BinaryHeap::new();

    for item in segments {
        events.push(Event {
            point: item.start,
            event_type: EventType::Start,
            line: Some(item),
            other: None
        });
        events.push(Event {
            point: item.end,
            event_type: EventType::End,
            line: Some(item),
            other: None
        });
    }

    let mut SL = SweepLine::new();
    let mut intersections = Vec::new();
    
    while let Some(event) = events.pop() {
        match event.event_type {
            EventType::Start => {
                SL.add_line(event.point, event.line.unwrap());

                let (above, below) = SL.get_neighbors(event.line.unwrap());

                if let Some(above) = above {
                    // check for intersection
                    let intersection = event.line.unwrap().intersection(above.line);
                    if let Some(intersection) = intersection {
                        events.push(Event {
                            point: intersection,
                            event_type: EventType::Intersection,
                            line: Some(event.line.unwrap()),
                            other: Some(above.line)
                        });
                    }
                }

                if let Some(below) = below {
                    // check for intersection
                    let intersection = event.line.unwrap().intersection(below.line);
                    if let Some(intersection) = intersection {
                        events.push(Event {
                            point: intersection,
                            event_type: EventType::Intersection,
                            line: Some(event.line.unwrap()),
                            other: Some(below.line)
                        });
                    }
                }
            },
            EventType::End => {
                let (above, below) = SL.get_neighbors(event.line.unwrap());

                // Below neighbor intersection
                if let Some(below) = below {
                    // check for intersection
                    let intersection = event.line.unwrap().intersection(below.line);
                    if let Some(intersection) = intersection {
                        events.push(Event {
                            point: intersection,
                            event_type: EventType::Intersection,
                            line: Some(event.line.unwrap()),
                            other: Some(below.line)
                        });
                    }
                }

                SL.remove_line(event.line.unwrap());

            },
            EventType::Intersection => {
                intersections.push(event.point);
                // Verify?
                let (segE1, segE2) = SL.swap_lines(event.line.unwrap(), event.other.unwrap(), event.point);

                let (_, segE1_below) = SL.get_neighbors(segE1);
                let (segE2_above, _) = SL.get_neighbors(segE2);

                if let Some(segE1_below) = segE1_below {
                    // check for intersection
                    let intersection = segE1.intersection(segE1_below.line);
                    if let Some(intersection) = intersection {
                        let new_event = Event {
                            point: intersection,
                            event_type: EventType::Intersection,
                            line: Some(segE1),
                            other: Some(segE1_below.line)
                        };

                        if !contains_event(&events, &new_event) {
                            events.push(new_event);
                        }
                    }
                }

                if let Some(segE2_above) = segE2_above {
                    // check for intersection
                    let intersection = segE2.intersection(segE2_above.line);
                    if let Some(intersection) = intersection {
                        let new_event = Event {
                            point: intersection,
                            event_type: EventType::Intersection,
                            line: Some(segE2),
                            other: Some(segE2_above.line)
                        };

                        if !contains_event(&events, &new_event) {
                            events.push(new_event);
                        }
                    }
                }
            }
        }
    }

    println!("Done");
    println!("Schnittpunkte: {}", intersections.len());

}

fn contains_event(heap: &BinaryHeap<Event>, event: &Event) -> bool {
    // Prüfung, ob event in heap vorhanden ist
    for e in heap.iter() {
        if e == event {
            return true;
        }
    }

    return false;
}