use std::{cmp::Ordering, collections::{BTreeSet, BinaryHeap}};

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
        self.start.y == other.start.y
    }
}

impl Ord for LineSegment {
    fn cmp(&self, other: &Self) -> Ordering {

        /*
        Start = Start && Ende = Ende -> Equal
        Start = Start && Ende > Ende -> Greater
        Start = Start && Ende < Ende -> Less
        Start > Start -> Greater
        Start < Start -> Less
        */

        if self.start.y == other.start.y {
            if self.end.y == other.end.y {
                if self.start.x == other.start.x {
                    if self.end.x == other.end.x {
                        Ordering::Equal
                    } else if self.end.x > other.end.x {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                } else if self.start.x > other.start.x {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            } else if self.end.y > other.end.y {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        } else if self.start.y > other.start.y {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl PartialOrd for LineSegment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for LineSegment {}

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
            return Ordering::Equal; // TODO: SORTIERUNG PASST? ODER AUF SEGMENT/OTHER PRÜFEN
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

// Sort sweepline by y-coordinate
fn sort_sweep_line(sweep_line: &mut Vec<LineSegment>) {
    sweep_line.sort_by(|a, b| a.start.y.partial_cmp(&b.start.y).unwrap());
}

// Add to sweep line
fn add_to_sweep_line(sweep_line: &mut Vec<LineSegment>, segment: LineSegment) {
    sweep_line.push(segment);
    sort_sweep_line(sweep_line);
}

// Remove from sweep line
fn remove_from_sweep_line(sweep_line: &mut Vec<LineSegment>, segment: LineSegment) {
    if let Some(index) = sweep_line.iter().position(|x| x == &segment) {
        sweep_line.remove(index);
        sort_sweep_line(sweep_line);
    }
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

    // Alle Events ausgeben
    for event in events.iter() {
        println!("Event at ({}, {}) of type {}", event.point.x, event.point.y, event.event_type);
    }

    let mut sweep_line = BTreeSet::new();

    let mut intersections = Vec::new();

    while let Some(event) = events.pop() {
        match event.event_type.as_str() {
            "Start" => {
                sweep_line.insert(event.segment);
                
                // Above und below finden
                let above = sweep_line.range(..event.segment).next_back();
                let below = sweep_line.range(event.segment..).skip(1).next();

                // Schnittpunkt zwischen event.segment und above
                if let Some(above) = above {
                    if let Some(point) = event.segment.intersect(above) {
                        events.push(Event {
                            point,
                            event_type: "Intersection".to_owned(),
                            segment: event.segment,
                            other: Some(*above),
                        });
                    }
                }

                // Schnittpunkt zwischen event.segment und below
                if let Some(below) = below {
                    if let Some(point) = event.segment.intersect(below) {
                        events.push(Event {
                            point,
                            event_type: "Intersection".to_owned(),
                            segment: event.segment,
                            other: Some(*below),
                        });
                    }
                }
            }


            "End" => {
                sweep_line.remove(&event.segment);

                // Above und below finden
                let above = sweep_line.range(..event.segment).next_back();
                let below = sweep_line.range(event.segment..).skip(1).next();
                
                // Schnittpunkt zwischen above und below
                if let (Some(above), Some(below)) = (above, below) {
                    if let Some(point) = above.intersect(below) {
                        // Wenn event noch nicht in events vorhanden, hinzufügen
                        let new_event = Event {
                            point,
                            event_type: "Intersection".to_owned(),
                            segment: *above,
                            other: Some(*below)
                        };

                        if !contains_event(&events, &new_event) && new_event != event {
                            events.push(new_event);
                        }
                    }
                }



            }
            "Intersection" => {
                intersections.push(event.point);

                let segE1 = event.segment;
                let segE2 = event.other.unwrap();

                sweep_line.remove(&segE1);
                sweep_line.remove(&segE2);
                
                let new_segE1 = LineSegment { start: event.point, end: segE1.end };
                let new_segE2 = LineSegment { start: event.point, end: segE2.end };

                sweep_line.insert(new_segE1);
                sweep_line.insert(new_segE2);

                
                // Above und below finden
                let above = sweep_line.range(..new_segE2).next_back();
                let below = sweep_line.range(new_segE1..).skip(1).next();

                // Schnittpunkt zwischen new_segE2 und above
                if let Some(above) = above {
                    if let Some(point) = new_segE2.intersect(above) {
                        let new_event = Event {
                            point,
                            event_type: "Intersection".to_owned(),
                            segment: new_segE2,
                            other: Some(*above),
                        };

                        if !contains_event(&events, &new_event) && new_event != event {
                            events.push(new_event);
                        }
                    }
                }

                // Schnittpunkt zwischen new_segE1 und below
                if let Some(below) = below {
                    if let Some(point) = new_segE1.intersect(below) {
                        let new_event = Event {
                            point,
                            event_type: "Intersection".to_owned(),
                            segment: new_segE1,
                            other: Some(*below),
                        };

                        if !contains_event(&events, &new_event) && new_event != event {
                            events.push(new_event);
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



// Unit Test for intersect
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_segment_intersect() {
        let AB = LineSegment { start: Point { x: -1.0, y: 1.0 }, end: Point { x: 5.0, y: 3.0 } };
        let CD = LineSegment { start: Point { x: 0.0, y: 0.0 }, end: Point { x: 4.0, y: 4.0 } };
        let EF = LineSegment { start: Point { x: 0.0, y: 4.0 }, end: Point { x: 4.0, y: 0.0 } };
        let GH = LineSegment { start: Point { x: 0.3, y: 2.0 }, end: Point { x: 0.6, y: 1.0 } };
        let IJ = LineSegment { start: Point { x: 3.16, y: 1.24 }, end: Point { x: 3.9, y: 3.32 } };
        let KL = LineSegment { start: Point { x: 2.0, y: 1.5 }, end: Point { x: 4.0, y: 1.5 } };
        let EC = LineSegment { start: Point { x: 0.0, y: 4.0 }, end: Point { x: 0.0, y: 0.0 } };
        let MN: LineSegment = LineSegment { start: Point { x: -0.5, y: 4.0 }, end: Point { x: 0.5, y: 4.0 } };
    
        assert_eq!(AB.intersect(&CD), Some(Point { x: 2.0, y: 2.0 }));
        assert_eq!(AB.intersect(&EF), Some(Point { x: 2.0, y: 2.0 }));
        assert_eq!(CD.intersect(&EF), Some(Point { x: 2.0, y: 2.0 }));
        assert_eq!(AB.intersect(&GH), Some(Point { x: 0.4545454545454548, y: 1.4848484848484849 }));
        assert_eq!(AB.intersect(&IJ), Some(Point { x: 3.622836363636364, y: 2.5409454545454544 }));
        assert_eq!(KL.intersect(&EF), Some(Point { x: 2.5, y: 1.5 }));
        assert_eq!(KL.intersect(&IJ), Some(Point { x: 3.2525000000000004, y: 1.5 }));
        assert_eq!(EC.intersect(&EF), Some(Point { x: 0.0, y: 4.0 }));
        assert_eq!(EC.intersect(&CD), Some(Point { x: 0.0, y: 0.0 }));
        assert_eq!(MN.intersect(&EF), Some(Point { x: 0.0, y: 4.0 }));
        assert_eq!(MN.intersect(&EC), Some(Point { x: 0.0, y: 4.0 }));

        // Segmente in Liste einfügen
        let segments = vec![AB, CD, EF, GH, IJ];

        // Teste alle Punkte miteinander in segments
        // for (i, segment) in segments.iter().enumerate() {
        //     for (j, other) in segments.iter().enumerate() {
        //         if i == j {
        //             continue;
        //         }

        //         let result = segment.intersect(other);

        //         // Ausgabe, ob Schnittpunkt oder None
        //         match result {
        //             Some(point) => {
        //                 //println!("Intersection at (x: {:.2}, y: {:.2})\n", point.x, point.y);
        //             }
        //             None => todo!(),
        //         }
        //     }
        // }
     }
}