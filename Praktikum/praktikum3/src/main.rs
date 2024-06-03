use std::{cmp::Ordering, collections::{BTreeMap, BTreeSet, BinaryHeap, HashSet}};

/////////////
/// POINT ///
/////////////

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

// Hash for Point
impl std::hash::Hash for Point {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
    }
}

/////////////////
// LINESEGMENT //
/////////////////

#[derive(Debug, Clone, Copy)]
struct LineSegment {
    start: Point,
    end: Point,
    id: usize,
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

    fn round_to_4_decimals(num: f64) -> f64 {
        (num * 10000.0).round() / 10000.0
    }

    fn intersect(&self, other: &LineSegment) -> Option<Point> {
        let LineSegment { start: p1, end: p2 , id: id1} = self;
        let LineSegment { start: p3, end: p4 , id: id2} = other;

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
            x: Self::round_to_4_decimals(p1.x + u * (p2.x - p1.x)),
            y: Self::round_to_4_decimals(p1.y + u * (p2.y - p1.y)),
        })
    }
}

impl Ord for LineSegment {
    fn cmp(&self, other: &Self) -> Ordering {

        // Sortiere Segmente nach dem Y-Wert des Startpunktes
        if self.start.y < other.start.y {
            Ordering::Greater
        } else if self.start.y > other.start.y {
            Ordering::Less
        } else {
            // Sortiere Segmente nach dem X-Wert des Startpunktes
            if self.start.x < other.start.x {
                Ordering::Greater
            } else if self.start.x > other.start.x {
                Ordering::Less
            } else {
                // Sortiere Segmente nach dem Y-Wert des Endpunktes
                if self.end.y < other.end.y {
                    Ordering::Greater
                } else if self.end.y > other.end.y {
                    Ordering::Less
                } else {
                    // Sortiere Segmente nach dem X-Wert des Endpunktes
                    if self.end.x < other.end.x {
                        Ordering::Greater
                    } else if self.end.x > other.end.x {
                        Ordering::Less
                    } else {
                        Ordering::Equal
                    }
                }
            }
        }


        /*
        Start = Start && Ende = Ende -> Equal
        Start = Start && Ende > Ende -> Greater
        Start = Start && Ende < Ende -> Less
        Start > Start -> Greater
        Start < Start -> Less
        */

        /*if self.start.y == other.start.y {
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
        } else if self.start.y < other.start.y {
            Ordering::Greater
        } else {
            Ordering::Less
    }*/
        }
}

impl PartialOrd for LineSegment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for LineSegment {
    fn eq(&self, other: &Self) -> bool {
        self.start.y == other.start.y
    }
}

impl Eq for LineSegment {}

////////////////
//// EVENT /////
////////////////
#[derive(PartialEq, Clone, Debug)]
struct Event {
    point: Point,
    event_type: String,
    segment: LineSegment,
    other: Option<LineSegment>,
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Sortiere nach x-Koordinate und dann nach y-Koordinate
        if self.point.x < other.point.x {
            std::cmp::Ordering::Greater//Less
        } else if self.point.x > other.point.x {
            std::cmp::Ordering::Less//Greater
        } else {
            if self.point.y > other.point.y {
                std::cmp::Ordering::Greater//Less
            } else if self.point.y < other.point.y {
                std::cmp::Ordering::Less//Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Event {}

////////////////
/// HELPERS ////
////////////////
fn read_segments_from_file(filename: &str) -> Vec<LineSegment> {
    let mut segments = Vec::new();
    let file = std::fs::read_to_string(filename).expect("Could not read file");
    let mut current_id = 0;
    for line in file.lines() {
        let mut parts = line.split_whitespace();
        let mut x1: f64 = parts.next().unwrap().parse().unwrap();
        let y1: f64 = parts.next().unwrap().parse().unwrap();
        let mut x2: f64 = parts.next().unwrap().parse().unwrap();
        let y2: f64 = parts.next().unwrap().parse().unwrap();

        // Erstellung der Punkte
        let start = Point { x: x1, y: y1 };
        let end = Point { x: x2, y: y2 };

        if start == end {
            continue;
        }

        if start.x == end.x {
            continue;
        } else {
            if start.x < end.x {
                segments.push(LineSegment {
                    start: Point { x: x1, y: y1 },
                    end: Point { x: x2, y: y2 },
                    id: current_id,
                });
            } else {
                segments.push(LineSegment {
                    start: Point { x: x2, y: y2 },
                    end: Point { x: x1, y: y1 },
                    id: current_id,
                });
            }
            current_id += 1;
        }
    }
    segments
}

fn contains_event(heap: &BinaryHeap<Event>, event: &Event) -> bool {
    // Prüfung, ob event in heap vorhanden ist
    for e in heap.iter() {
        if e == event || e.point == event.point {
            return true;
        }
    }

    return false;
}

fn remove_duplicates<T: std::hash::Hash + Eq + Clone>(vec: Vec<T>) -> Vec<T> {
    let mut seen = HashSet::new();
    vec.into_iter().filter(|item| seen.insert(item.clone())).collect()
}

// Funktion zum Ausgeben der Events
fn print_events(events: &BinaryHeap<Event>) {
    // for item in events.iter() {
    //     println!("{:?}", item);
    // }
    if events.len() % 100 == 0 {
        println!("Events: {:?}", events.len());
    }
}

////////////
/// MAIN ///
////////////

fn main() {
    // let file_path = "strecken/s_5_1.dat";
    //let file_path = "strecken/s_1000_1.dat";
    let file_path = "strecken/s_10000_1.dat";
    //let file_path = "strecken/s_100000_1.dat";
    let segments = read_segments_from_file(file_path);

    // Ausgabe
    // for item in segments.iter() {
    //     println!("{:?}", item);
    // }

    // let segments = vec![
    //     LineSegment { start: Point { x: -1.0, y: 1.0 }, end: Point { x: 5.0, y: 3.0 } },
    //     LineSegment { start: Point { x: 0.0, y: 0.0 }, end: Point { x: 4.0, y: 4.0 } },
    //     LineSegment { start: Point { x: 0.3, y: 2.0 }, end: Point { x: 0.6, y: 0.5 } },
    //     LineSegment { start: Point { x: 3.16, y: 1.24 }, end: Point { x: 3.9, y: 3.32 } },
    //     LineSegment { start: Point { x: 2.0, y: 1.5 }, end: Point { x: 4.0, y: 1.5 } },
    // ];

    // let segments = vec![
    //     LineSegment { start: Point { x: -1.0, y: 1.0 }, end: Point { x: 5.0, y: 1.0 }, id: 0},
    //     LineSegment { start: Point { x: 1.0, y: 0.0 }, end: Point { x: 4.0, y: 4.0 }, id: 1 },
    //     LineSegment { start: Point { x: 2.0, y: 4.0 }, end: Point { x: 5.0, y: 0.0 }, id: 2 },
    // ];

    // Füge alle segmente in key_map ein
    let mut key_map = BTreeMap::new();
    for item in segments.iter() {
        key_map.insert(item.id, *item);
    }


    let mut events = BinaryHeap::new();

    // Initialisiere event queue x = all segment endpoints
    // Sortiere x by increasing x and y
    for item in segments {
        events.push(Event {
            point: item.start,
            event_type: "Start".to_owned(),
            segment: item,
            other: None
        });
        events.push(Event {
            point: item.end,
            event_type: "End".to_owned(),
            segment: item,
            other: None
        });
    }

    // while let Some(event) = events.pop() {
    //     println!("Event: {:?} {:?}", event.point.x, event.point.y);
    // }

    // Initialisiere sweep line SL to be empty
    let mut sweep_line = BTreeSet::new();
    let mut intersections = Vec::new();

    while let Some(event) = events.pop() {
        match event.event_type.as_str() {
            "Start" => {
                handle_start_event(&mut events, event, &mut sweep_line);
                print_events(&events);
                // print_sweep_line(&sweep_line);
            }
            "End" => {
                handle_end_event(&mut events, event, &mut sweep_line, &key_map);
                print_events(&events);
                // print_sweep_line(&sweep_line);
            }
            "Intersection" => {
                intersections.push(handle_intersection_event(&mut events, event, &mut sweep_line));
                print_events(&events);
                // print_sweep_line(&sweep_line);
                // let intersections_clone = intersections.clone();
                // if intersections_clone.len() % 100 == 0 {
                //     let test = remove_duplicates(intersections_clone);
                //     println!("Number of intersections: {}", test.len());
                // }
            }
            _ => {}
        }
    }

    // Clear cmd
    print!("\x1B[2J\x1B[1;1H");

    println!("Number of intersections: {}", intersections.len());
    // Sortiere Intersections nach X-Wert
    intersections.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

    for intersection in intersections {
        println!("{:?}", intersection);
    }
}

// Funktion, um sweep line auszugeben
fn print_sweep_line(sweep_line: &BTreeSet<LineSegment>) {
    for item in sweep_line.iter() {
        println!("{:?}", item);
    }
    println!("-------------------");
}

// Funktion, um End-Events zu verarbeiten
fn handle_end_event(events: &mut BinaryHeap<Event>, event: Event, sweep_line: &mut BTreeSet<LineSegment>, key_map: &BTreeMap<usize, LineSegment>) {
    let segE = event.segment;

    // Find the segments segA and segB immediately above and below segE in SL
    let above = sweep_line.range(..event.segment).next_back();
    let below = sweep_line.range(event.segment..).skip(1).next();


    if let (Some(above), Some(below)) = (above, below) {
        if let Some(point) = below.intersect(above) {
            let new_event = Event {
                point,
                event_type: "Intersection".to_owned(),
                segment: *below,
                other: Some(*above),
            };

            if !contains_event(&events, &new_event) && new_event.point != event.point {
                events.push(new_event);
            }

        }
    }

    // Find segment in key_map
    let real_item = key_map.get(&segE.id);
    if real_item.is_some() {
        let mut ret = sweep_line.remove(real_item.unwrap());
        if ret {
            // println!("Item removed: Id: {:?} Start: {:?} Ende: {:?}", segE.id, segE.start, segE.end);
        } else {
            let clonedSweepLine = sweep_line.clone();
            let real_item = clonedSweepLine.iter().find(|&p| p.id == segE.id);
            if real_item.is_some() {
                ret = sweep_line.remove(real_item.unwrap());
                if ret {
            // println!("Item removed: Id: {:?} Start: {:?} Ende: {:?}", segE.id, segE.start, segE.end);
                } else {
                    println!("Item BOOM not found: {:?}", segE.id);
                }
            } else {
                println!("Item REALLY not found: {:?}", segE.id);
            }
        }
    } else {
        println!("Item not found: {:?}", segE.id);
    }

    // let clonedSweepLine = sweep_line.clone();
    // let real_item = clonedSweepLine.iter().find(|&p| p.id == segE.id);
    // if real_item.is_some() {
    //     sweep_line.remove(real_item.unwrap());
    //     println!("Item removed: {:?}", segE.id)
    // } else {
    //     println!("Item not found: {:?}", segE.id);
    // }

}

// Funktion, um Start-Events zu verarbeiten
fn handle_start_event(events: &mut BinaryHeap<Event>, event: Event, sweep_line: &mut BTreeSet<LineSegment>) {
    let segE = event.segment;
    sweep_line.insert(segE);

    let test1 = LineSegment { start: Point { x: 76.772, y: 97.84 }, end: Point { x: 76.8412, y: 97.5077 }, id: 0 };

    let segE1 = event.segment;
    // Vergleiche x und y von segE1 mit test1
    if segE1.start.x == test1.start.x && segE1.start.y == test1.start.y {
        println!("SegE1: {:?}", segE1);
    }


    // Find the segments segA and segB immediately above and below segE in SL
    let above = sweep_line.range(..event.segment).next_back();
    let below = sweep_line.range(event.segment..).skip(1).next();

    if let Some(above) = above {
        if let Some(point) = segE.intersect(above) {
            let new_event = Event {
                point,
                event_type: "Intersection".to_owned(),
                segment: segE,
                other: Some(*above),
            };

            events.push(new_event);
        }
    }

    if let Some(below) = below {
        if let Some(point) = segE.intersect(below) {
            let new_event = Event {
                point,
                event_type: "Intersection".to_owned(),
                segment: segE,
                other: Some(*below),
            };

            events.push(new_event);
        }
    }
}

// Funktion, um Intersection-Events zu verarbeiten
fn handle_intersection_event(events: &mut BinaryHeap<Event>, event: Event, sweep_line: &mut BTreeSet<LineSegment>) -> Point {

    // if sweep_line.len() >= 7 {
    //     println!("Sweep Line: {:?}", sweep_line.len());
    //     print_sweep_line(&sweep_line);
    // }

    let mut segE1 = event.segment;
    let mut segE2 = event.other.unwrap();

    let test1 = LineSegment { start: Point { x: 76.772, y: 97.84 }, end: Point { x: 76.8412, y: 97.5077 }, id: 0 };

    // Vergleiche x und y von segE1 mit test1
    if segE1.start.x == test1.start.x && segE1.start.y == test1.start.y {
        println!("SegE1: {:?}", segE1);
    }

    if segE2.start.x == test1.start.x && segE2.start.y == test1.start.y {
        println!("SegE2: {:?}", segE2);
    }

    sweep_line.remove(&segE1);
    sweep_line.remove(&segE2);

    // Swap segE and segF in SL
    segE1 = LineSegment { start: event.point, end: segE1.end, id: segE1.id };
    segE2 = LineSegment { start: event.point, end: segE2.end, id: segE2.id};
    //std::mem::swap(&mut segE1, &mut segE2);

    let mut ret = sweep_line.insert(segE1);
    if !ret {
        println!("Item not inserted: {:?}", segE1.id);
    }
    ret = sweep_line.insert(segE2);
    if !ret {
        println!("Item not inserted: {:?}", segE1.id);
    }

    // Find the segments segA and segB immediately above and below segE in SL
    let segA = sweep_line.range(..segE2).next_back();
    let segB = sweep_line.range(segE1..).skip(1).next();

    // Intersect(segE2 with above)
    if let Some(segA) = segA {
        if let Some(point) = segE2.intersect(segA) {
            let new_event = Event {
                point,
                event_type: "Intersection".to_owned(),
                segment: segE2,
                other: Some(*segA),
            };

            if !contains_event(&events, &new_event) && new_event.point != event.point {
                events.push(new_event);
            } else {
                // println!("Event already in queue: {:?}", new_event.point);
            }
        }
    }

    // Intersect(segE1 with below)	
    if let Some(segB) = segB {
        if let Some(point) = segE1.intersect(segB) {
            let new_event = Event {
                point,
                event_type: "Intersection".to_owned(),
                segment: segE1,
                other: Some(*segB),
            };

            if !contains_event(&events, &new_event) && new_event.point != event.point {
                events.push(new_event);
            } else {
                // println!("Event already in queue: {:?}", new_event.point);
            }
            // events.push(new_event);
        }
    }

    // if sweep_line.len() >= 7 {
    //     println!("Sweep Line: {:?}", sweep_line.len());
    //     print_sweep_line(&sweep_line);
    // }

    event.point
}


// Unit Test
#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn test_above_below() {
        let segments = vec![
            LineSegment { start: Point { x: -1.0, y: 1.0 }, end: Point { x: 5.0, y: 3.0 }, id: 0 },
            LineSegment { start: Point { x: 0.0, y: 0.0 }, end: Point { x: 4.0, y: 4.0 }, id: 0 },
            LineSegment { start: Point { x: 0.3, y: 2.0 }, end: Point { x: 0.6, y: 0.5 }, id: 0 },
            LineSegment { start: Point { x: 3.16, y: 1.24 }, end: Point { x: 3.9, y: 3.32 }, id: 0 },
            LineSegment { start: Point { x: 2.0, y: 1.5 }, end: Point { x: 4.0, y: 1.5 }, id: 0 },
        ];

        let mut sweep_line = BTreeSet::new();

        // Füge segmente in sweep_line ein
        for item in segments {
            sweep_line.insert(item);
        }

        // Ausgabe der Segmente in sweep_line
        for item in sweep_line.iter() {
            println!("{:?}", item);
        }

        let target = LineSegment { start: Point { x: 3.16, y: 1.24 }, end: Point { x: 3.9, y: 3.32 }, id: 0 };

        let above = sweep_line.range(..target).next_back();
        let below = sweep_line.range(target..).skip(1).next();

        assert_eq!(below, Some(&LineSegment { start: Point { x: -1.0, y: 1.0 }, end: Point { x: 5.0, y: 3.0 }, id: 0 }));
        assert_eq!(above, Some(&LineSegment { start: Point { x: 2.0, y: 1.5 }, end: Point { x: 4.0, y: 1.5 }, id: 0 }));
    }

    // Test contains_event
    #[test]
    fn test_contains_event() {
        let mut events = BinaryHeap::new();
        let event = Event {
            point: Point { x: 1.0, y: 1.0 },
            event_type: "Start".to_owned(),
            segment: LineSegment { start: Point { x: 1.0, y: 1.0 }, end: Point { x: 2.0, y: 2.0 }, id: 0 },
            other: None
        };

        let event2 = event.clone();

        events.push(event);

        assert_eq!(contains_event(&events, &event2), true);
    }

    // test swap funktion
    #[test]
    fn test_swap() {
        let segments = vec![
            LineSegment { start: Point { x: -1.0, y: 1.0 }, end: Point { x: 5.0, y: 3.0 }, id: 0 },
            LineSegment { start: Point { x: 0.0, y: 0.0 }, end: Point { x: 4.0, y: 4.0 }, id: 0 },
            LineSegment { start: Point { x: 0.3, y: 2.0 }, end: Point { x: 0.6, y: 0.5 }, id: 0 },
            LineSegment { start: Point { x: 3.16, y: 1.24 }, end: Point { x: 3.9, y: 3.32 }, id: 0 },
            LineSegment { start: Point { x: 2.0, y: 1.5 }, end: Point { x: 4.0, y: 1.5 }, id: 0 },
        ];

        let mut sweep_line = BTreeSet::new();

        // Füge segmente in sweep_line ein
        for item in segments {
            sweep_line.insert(item);
        }

        // Ausgabe der Segmente in sweep_line
        for item in sweep_line.iter() {
            println!("{:?}", item);
        }

        // Simuliere Schnitt
        println!("Simuliere Schnitt");
        let mut segE1 = LineSegment { start: Point { x: 0.0, y: 0.0 }, end: Point { x: 4.0, y: 4.0 }, id: 0 };
        let mut segE2 = LineSegment { start: Point { x: -1.0, y: 1.0 }, end: Point { x: 5.0, y: 3.0 }, id: 0 };
        
        sweep_line.remove(&segE1);
        sweep_line.remove(&segE2);

        segE1 = LineSegment { start: Point { x: 2.0, y: 2.0 }, end: Point { x: 4.0, y: 4.0 }, id: 0 };
        segE2 = LineSegment { start: Point { x: 2.0, y: 2.0 }, end: Point { x: 5.0, y: 3.0 }, id: 0 };

        sweep_line.insert(segE1);
        sweep_line.insert(segE2);

        // Ausgabe der Segmente in sweep_line
        for item in sweep_line.iter() {
            println!("{:?}", item);
        }

        let target = LineSegment { start: Point { x: 3.16, y: 1.24 }, end: Point { x: 3.9, y: 3.32 }, id: 0 };

        let above = sweep_line.range(..target).next_back();
        let below = sweep_line.range(target..).skip(1).next();

        assert_eq!(below, None);
        assert_eq!(above, Some(&LineSegment { start: Point { x: 2.0, y: 1.5 }, end: Point { x: 4.0, y: 1.5 }, id: 0 }));

    }

    // Test for event ordering
    #[test]
    fn test_event_ordering() {
        let event1 = Event {
            point: Point { x: 2.0, y: 1.0 },
            event_type: "Start".to_owned(),
            segment: LineSegment { start: Point { x: 1.0, y: 1.0 }, end: Point { x: 2.0, y: 2.0 }, id: 0 },
            other: None
        };

        let event2 = Event {
            point: Point { x: 1.0, y: 1.5 },
            event_type: "Start".to_owned(),
            segment: LineSegment { start: Point { x: 1.0, y: 1.0 }, end: Point { x: 2.0, y: 2.0 }, id: 0 },
            other: None
        };

        let event3 = Event {
            point: Point { x: 2.0, y: 1.0 },
            event_type: "Start".to_owned(),
            segment: LineSegment { start: Point { x: 1.0, y: 1.0 }, end: Point { x: 2.0, y: 2.0 }, id: 0 },
            other: None
        };

        let mut events = BinaryHeap::new();
        events.push(event1.clone());
        events.push(event2.clone());
        events.push(event3.clone());

        let mut vec = Vec::new();
        while let Some(event) = events.pop() {
            vec.push(event);
        }

        assert_eq!(vec[0], event2);
        assert_eq!(vec[1], event3);
        assert_eq!(vec[2], event1);
    }

    // Test für Schnittfunktion
    #[test]
    fn test_intersection() {
        let seg1 = LineSegment { start: Point { x: 76.772, y: 97.84 }, end: Point { x: 76.8412, y: 97.5077 }, id: 0 };
        let seg2 = LineSegment { start: Point { x: 77.031, y: 97.833 }, end: Point { x: 76.5049, y: 97.1055 }, id: 0 };

        let point = seg1.intersect(&seg2).unwrap();
        assert_eq!(point, Point { x: 76.831, y: 97.5565 });
    }
}