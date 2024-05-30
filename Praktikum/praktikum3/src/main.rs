use std::collections::{BTreeSet, BinaryHeap};
use std::cmp::Ordering;
use std::fmt;
use std::os::windows::io::BorrowedHandle;

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

// Funktion zum Einlesen von LineSegments aus Datei im Format "0 0 0.1 0.1"
fn read_segments_from_file(filename: &str) -> Vec<LineSegment> {
    let mut segments = Vec::new();
    let file = std::fs::read_to_string(filename).expect("Could not read file");
    for line in file.lines() {
        let mut parts = line.split_whitespace();
        let mut x1: f64 = parts.next().unwrap().parse().unwrap();
        let y1: f64 = parts.next().unwrap().parse().unwrap();
        let mut x2: f64 = parts.next().unwrap().parse().unwrap();
        let y2: f64 = parts.next().unwrap().parse().unwrap();

        // Erstellung der Punkte
        let start = Point { x: x1, y: y1 };
        let end = Point { x: x2, y: y2 };

        if start.x >= end.x {
            segments.push(LineSegment {
                start: Point { x: x1, y: y1 },
                end: Point { x: x2, y: y2 },
            });
        } else {
            segments.push(LineSegment {
                start: Point { x: x2, y: y2 },
                end: Point { x: x1, y: y1 },
            });
        }

    }
    segments
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
    Intersection(Point, LineSegment, LineSegment),
}

#[derive(Debug, Clone)]
struct Event {
    point: Point,
    event_type: EventType,
    segment: Option<LineSegment>,
}

impl Event {
    // Funktion um Event zu String
    fn to_string(&self) -> String {
        match &self.event_type {
            EventType::Start => format!("S"), //tart event at (x: {:.2}, y: {:.2})", self.point.x, self.point.y),
            EventType::End => format!("E"), //nd event at (x: {:.2}, y: {:.2})", self.point.x, self.point.y),
            EventType::Intersection(point, segA, segB) => format!("I") //ntersection event at (x: {:.2}, y: {:.2})", point.x, point.y),
        }
    }
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



        //self.point.x.partial_cmp(&other.point.x).unwrap().then_with(|| {
        //    self.point.y.partial_cmp(&other.point.y).unwrap()
       //})        
       /*
        .then_with(|| {
            match (&self.event_type, &other.event_type) {
                (EventType::Intersection(_), EventType::Intersection(_)) => Ordering::Equal,
                (EventType::Intersection(_), _) => Ordering::Greater,
                (_, EventType::Intersection(_)) => Ordering::Less,
                (_, _) => Ordering::Equal,
            }
        })
         */
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point(x: {:.2}, y: {:.2})", self.x, self.y)
    }
}

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
        // Ausgabe der Vergleiche
        ////println!("COMPARING: Segment from (x: {:.2}, y: {:.2}) to (x: {:.2}, y: {:.2}) with segment from (x: {:.2}, y: {:.2}) to (x: {:.2}, y: {:.2})", self.start.x, self.start.y, self.end.x, self.end.y, other.start.x, other.start.y, other.end.x, other.end.y);
        ////println!("BLUB");
        if self.start.x == other.start.x && self.start.y == other.start.y {
            return Ordering::Equal;
        } else

        if self.start.x > other.start.x {
            //return Some(Ordering::Greater);
            return Ordering::Less;
        } else 
        {
            //return Some(Ordering::Less);
            return Ordering::Greater;
        } 
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

    // TODO: Wenn Start hinter Ende liegt, umdrehen!
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


    let file_path = "strecken/s_1000_1.dat";
    //let file_path = "strecken/s_10000_1.dat";
    //let file_path = "strecken/s_100000_1.dat";
    //segments = read_segments_from_file(file_path);

    // let segments = vec![
    //     LineSegment { start: Point { x: -1.0, y: 10.0 }, end: Point { x: 10.0, y: 3.0 } },
    //     LineSegment { start: Point { x: 0.0, y: 8.0 }, end: Point { x: 9.0, y: 4.0 } },
    //     LineSegment { start: Point { x: 1.0, y: 6.0 }, end: Point { x: 4.0, y: 0.0 } },
    //     LineSegment { start: Point { x: 2.0, y: 4.0 }, end: Point { x: 7.0, y: 1.0 } },
    //     LineSegment { start: Point { x: 3.0, y: 2.0 }, end: Point { x: 5.0, y: 3.32 } },
    // ];

    let mut events = BinaryHeap::new();

    // Ereignisse erzeugen
    for segment in segments {
        events.push(Event {
            point: segment.start,
            event_type: EventType::Start,
            segment: Some(segment),
        });
        events.push(Event {
            point: segment.end,
            event_type: EventType::End,
            segment: Some(segment),
        });
        // Ausgabe, dass Start und End Events hinzugefügt wurden
        ////println!("EVENT: Start event at (x: {:.2}, y: {:.2})", segment.start.x, segment.start.y);
        ////println!("EVENT: End event at (x: {:.2}, y: {:.2})\n", segment.end.x, segment.end.y);

        // Ausgabe der events mit hilfe der tostring funktion in Schleife
        // //println!("Events einfügen:");
        // for event in &events {
        //     print!("{}", event.to_string());
        // }
        // //println!("\n");
    }    

    // Ausgabe mit pop
    // //println!("Events:");
    // while let Some(event) = events.pop() {
    //    //println!("Even: {}", event.to_string());
    // }
    // //println!("----------------------------------");

    //let mut sweep_line = BTreeSet::new();
    let mut sweep_line = Vec::new();

    let mut intersections = Vec::new();

    // Ausgabe der events mit hilfe der tostring funktion in Schleife
    // //println!("Abschleßende Events:");
    // for event in &events {
    //     print!("{}", event.to_string());
    // }
    // //println!("\n");

    while let Some(event) = events.pop() {

        // Ausgabe der events mit hilfe der tostring funktion in Schleife
        // //println!("Events:");
        // for event in &events {
        //     print!("{}", event.to_string());
        // }
        // //println!("\n");

        match event.event_type {
            EventType::Start => {
                if let Some(segment) = event.segment {
                    // Hinzufügen zur SL
                    //sweep_line.insert(segment);
                    sweep_line.push(segment);

                    // Finden von above und below
                    let len = sweep_line.len();
                    println!("Length: {}", len);
                    if sweep_line.len() > 1 {
                        let pred = sweep_line.get(sweep_line.len() - 2);

                        if let Some(pred) = pred {
                            if let Some(point) = segment.intersect(pred) {
                                events.push(Event {
                                    point,
                                    event_type: EventType::Intersection(point, segment, pred.clone()),
                                    segment: None,
                                });
                            }
                        }
                    }
                }
            }
            EventType::End => {
                if let Some(segment) = event.segment {
                    // Entferne das Segment aus der Sweep Line
                    if let Some(index) = sweep_line.iter().position(|&x| x == segment) {
                        sweep_line.remove(index);
                    }

                    // Finden von above und below
                    let index = sweep_line.iter().position(|&x| x == segment);
                    let above = index.and_then(|i| sweep_line.get(i + 1));
                    let below = index.and_then(|i| sweep_line.get(i - 1));
                    
                    // Prüfe auf Schnittpunkte mit above und below, wenn above und below existieren
                    if let (Some(above), Some(below)) = (above, below) {
                        if let Some(point) = below.intersect(above) {
                            // Wenn Event noch nicht in events ist, füge es hinzu
                
                            events.push(Event {
                                point,
                                event_type: EventType::Intersection(point, *below, *above),
                                segment: None,
                            });
                        }
                    }
                }
            }
            EventType::Intersection(point, segE1, segE2) => {
                intersections.push(point);

                // Tausche Position von segA und segB in der Sweep Line
                let indexA = sweep_line.iter().position(|&x| x == segE1);
                let indexB = sweep_line.iter().position(|&x| x == segE2);

                if let (Some(indexA), Some(indexB)) = (indexA, indexB) {
                    sweep_line.swap(indexA, indexB);

                    if (sweep_line.len() - 1) == indexA {
                        let segA = sweep_line.get(indexA + 1);
                        if let Some(segA) = segA {
                            if let Some(point) = segA.intersect(&segE1) {
                                events.push(Event {
                                    point,
                                    event_type: EventType::Intersection(point, *segA, segE1),
                                    segment: None,
                                });
                            }
                        }
                    }
                    let segB = sweep_line.get(indexA - 1);

                    if let Some(segB) = segB {
                        if let Some(point) = segB.intersect(&segE2) {
                            events.push(Event {
                                point,
                                event_type: EventType::Intersection(point, *segB, segE2),
                                segment: None,
                            });
                        }
                    }
                }
            }
        }
    }


    // Zeilenweise Ausgabe der Schnittpunkte
    //println!("Intersections:");
    // for point in intersections {
    //     println!("Intersection at (x: {:.2}, y: {:.2})\n", point.x, point.y);
    // }
    // Anzahl der Schnittpunkte
    println!("Number of intersections: {}", intersections.len());
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