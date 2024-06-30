use std::collections::BTreeSet;
use std::cmp::Ordering;
use std::fmt::Formatter;
use std::io::Write;
use std::fmt::Debug;

use ordered_float::OrderedFloat;

#[derive(Debug, Clone)]
struct Intersection {
    point: Point,
    line: Line,
    other: Line,
}

impl Eq for Intersection {}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }
}

struct Event {
    pub point: Point,
    pub event_type: EventType,
    pub line: Option<Line>,
    pub other: Option<Line>,
}

enum EventType {
    Start,
    End,
    Intersection,
}

impl PartialEq for EventType {
    fn eq(&self, other: &Self) -> bool {
        match self {
            EventType::Start => match other {
                EventType::Start => true,
                _ => false,
            },
            EventType::End => match other {
                EventType::End => true,
                _ => false,
            },
            EventType::Intersection => match other {
                EventType::Intersection => true,
                _ => false,
            },
        }
    }
}

impl Debug for EventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::Start => write!(f, "Start"),
            EventType::End => write!(f, "End"),
            EventType::Intersection => write!(f, "Intersection"),
        }
    }
}

impl Eq for Event {}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point && self.event_type == other.event_type
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.point.x.partial_cmp(&other.point.x).unwrap()
            .then_with(|| self.point.y.partial_cmp(&other.point.y).unwrap())
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


#[derive(Debug, PartialEq, Eq, Clone)]
struct Point {
    x: OrderedFloat<f64>,
    y: OrderedFloat<f64>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Line {
    start: Point,
    end: Point,
    id : usize,
}

impl Line {
    pub fn new(start: Point, end: Point, id: usize) -> Line {
        Line { start, end, id: id }
    }

    pub fn y(&self, x: OrderedFloat<f64>) -> OrderedFloat<f64> {
        let m = (self.end.y - self.start.y) / (self.end.x - self.start.x);
        let b = self.start.y - m * self.start.x;
        m * x + b
    }

    pub fn intersection(&self, other: &Line) -> Option<Point> {
        let x1 = self.start.x;
        let y1 = self.start.y;
        let x2 = self.end.x;
        let y2 = self.end.y;

        let x3 = other.start.x;
        let y3 = other.start.y;
        let x4 = other.end.x;
        let y4 = other.end.y;

        let d = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        if d == 0.0 {
            return None;
        }

        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / d;
        let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / d;

        if t >= OrderedFloat(0.0) && t <= OrderedFloat(1.0) && u >= OrderedFloat(0.0) && u <= OrderedFloat(1.0) {
            let x = x1 + t * (x2 - x1);
            let y = y1 + t * (y2 - y1);
            Some(Point { x: OrderedFloat(*x), y: OrderedFloat(*y) })
        } else {
            None
        }
    }
}

impl Ord for Line {
    fn cmp(&self, other: &Self) -> Ordering {
        // self.start.y.partial_cmp(&other.start.y).unwrap()
        //     .then_with(|| self.start.x.partial_cmp(&other.start.x).unwrap())
        self.start.y.cmp(&other.start.y)
        .then_with(|| self.start.x.cmp(&other.start.x))
        .then_with(|| self.end.y.cmp(&other.end.y))
        .then_with(|| self.end.x.cmp(&other.end.x))
    }
}

impl PartialOrd for Line {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
struct LineCollection {
    lines: BTreeSet<Line>,
}

impl LineCollection {
    fn new() -> Self {
        LineCollection { lines: BTreeSet::new() }
    }

    fn add_line(&mut self, line: Line) {
        // println!("Add line: {:?}", line);
        let ret = self.lines.insert(line.clone());

        if !ret {
            println!("Line already exists: {:?}", line);
        }
    }

    fn update_lines(&mut self, point: Point) {
        // Update each Start Point to have the new y value
        for line in self.lines.clone().iter() {
            let new_y = line.y(point.x);
            self.lines.remove(line);
            self.lines.insert(Line { start: Point { x: point.x, y: new_y }, end: line.end.clone(), id: line.id });
        }
    }

    // fn remove_line(&mut self, start: &Point, end: &Point) -> bool {
    //     self.lines.remove(&Line { start: start.clone(), end: end.clone() })
    // }

    fn remove_line(&mut self, line: Line) {
        let removed = self.lines.remove(&line);

        if !removed {
            println!("Line not found: {:?}", line);
        }
    }

    fn find_neighbors(&self, line: &Line) -> (Option<&Line>, Option<&Line>) {
        let mut before = None;
        let mut after = None;

        // Find before with iterator
        let mut iter = self.lines.iter();
        while let Some(l) = iter.next() {
            if l.id == line.id {
                break;
            }
            before = Some(l);
        }

        // Find after with iterator
        let mut iter = self.lines.iter();


        let mut found = false;
        while let Some(l) = iter.next() {
            if found {
                after = Some(l);
                break;
            }
            if l.id == line.id {
                found = true;
            }
        }

        // for l in self.lines.range(..line) {
        //     before = Some(l);
        // }
        // for l in self.lines.range(line..) {
        //     if l != line {
        //         after = Some(l);
        //         break;
        //     }
        // }
        (before, after)
        
    }

    fn swap_lines(&mut self, line1: &Line, line2: &Line, x_value: OrderedFloat<f64>) -> (Line, Line) {
        // Find by id
        let collection = self.lines.clone();
        let line1 = collection.iter().find(|x| x.id == line1.id).unwrap();
        let line2 = collection.iter().find(|x| x.id == line2.id).unwrap();

        self.lines.remove(line1);
        self.lines.remove(line2);

        let epsilon = 1e-9;

        let new_X = x_value + epsilon;
        let new_y1 = line1.y(new_X);
        let new_y2 = line2.y(new_X);

        let new_line1 = Line::new(Point { x: x_value, y: new_y1 }, line1.end.clone(), line1.id);
        let new_line2 = Line::new(Point { x: x_value, y: new_y2 }, line2.end.clone(), line2.id);

        self.lines.insert(new_line1.clone());
        self.lines.insert(new_line2.clone());

        (new_line1, new_line2)
    }
}

fn brute_force() -> Vec<Intersection> {
    let file_path = "G:\\Git\\computational-geometry\\Praktikum\\praktikum3\\strecken\\s_1000_10.dat";
    let segments = read_segments_from_file(file_path);

    let mut intersections = Vec::new();

    for i in 0..segments.len() {
        for j in i+1..segments.len() {
            let intersection = segments[i].intersection(&segments[j]);
            if let Some(intersection) = intersection {
                intersections.push(Intersection { point: intersection, line: segments[i].clone(), other: segments[j].clone() });
            }
        }
    }

    // Order by x value of Start absteigend
    intersections.sort_by(|a, b| a.point.x.partial_cmp(&b.point.x).unwrap());
    // Flip the order
    intersections.reverse();


    // To File
    let mut file = std::fs::File::create("output_brute-force.txt").expect("Could not create file");
    for intersection in &intersections {
        writeln!(file, "({}, {})", intersection.point.x.into_inner(), intersection.point.y.into_inner()).expect("Could not write to file");
    }

    intersections
}

fn read_segments_from_file(filename: &str) -> Vec<Line> {
    let mut segments = Vec::new();
    let file = std::fs::read_to_string(filename).expect("Could not read file");

    let mut id = 0;
    for line in file.lines() {
        let mut parts = line.split_whitespace();
        let mut x1: f64 = parts.next().unwrap().parse().unwrap();
        let y1: f64 = parts.next().unwrap().parse().unwrap();
        let mut x2: f64 = parts.next().unwrap().parse().unwrap();
        let y2: f64 = parts.next().unwrap().parse().unwrap();

        // Erstellung der Punkte
        let start = Point { x: OrderedFloat(x1), y: OrderedFloat(y1) };
        let end = Point { x: OrderedFloat(x2), y: OrderedFloat(y2) };

        // if start.x == 62.462 && start.y == 76.608 {
        //     continue;
        // }

        // if start.x == 10.649 && start.y == 2.807 {
        //     continue;
        // }

        // Kein Punkt
        if start == end {
            continue;
        }

        if start.x == end.x {
            continue;
        }

        if start.x < end.x {
            segments.push(Line {
                start: Point { x: OrderedFloat(x1), y: OrderedFloat(y1) },
                end: Point { x: OrderedFloat(x2), y: OrderedFloat(y2) },
                id: id,
            });
            id += 1;
        } else {
            segments.push(Line {
                start: Point { x: OrderedFloat(x2), y: OrderedFloat(y2) },
                end: Point { x: OrderedFloat(x1), y: OrderedFloat(y1) },
                id: id,
            });
            id += 1;
        }

    }
    segments
}

fn run() -> usize {

    let mut intersections = Vec::new();
    let mut events = BTreeSet::new();
    let mut sweep_line = LineCollection::new();

    let file_path = "G:\\Git\\computational-geometry\\Praktikum\\praktikum3\\strecken\\s_1000_10.dat";
    // let file_path = "G:\\Git\\computational-geometry\\Praktikum\\praktikum3\\strecken\\s_5_1.dat";
    let segments = read_segments_from_file(file_path);

    let segments = vec![
        Line {start:Point{x:ordered_float::OrderedFloat(91.1050),y:ordered_float::OrderedFloat(22.0320)},end:Point{x:ordered_float::OrderedFloat(91.1120),y:ordered_float::OrderedFloat(22.6269)}, id: 0 },
        Line { start: Point { x: ordered_float::OrderedFloat(90.5472), y: ordered_float::OrderedFloat(22.7331) }, end: Point { x: ordered_float::OrderedFloat(91.1570), y: ordered_float::OrderedFloat(22.4220) }, id: 1},
        Line { start: Point { x: ordered_float::OrderedFloat(90.7466), y: ordered_float::OrderedFloat(22.0581) }, end: Point { x: ordered_float::OrderedFloat(91.5450), y: ordered_float::OrderedFloat(21.3290) }, id: 2},
        Line { start: Point { x: ordered_float::OrderedFloat(90.8549), y: ordered_float::OrderedFloat(22.0544) }, end: Point { x: ordered_float::OrderedFloat(91.2730), y: ordered_float::OrderedFloat(21.5360) }, id: 3},
        Line { start: Point { x: ordered_float::OrderedFloat(90.5983), y: ordered_float::OrderedFloat(22.2864) }, end: Point { x: ordered_float::OrderedFloat(90.6610), y: ordered_float::OrderedFloat(21.9340) }, id: 4},
        ];

    for segment in segments {
        events.insert(Event { point: segment.start.clone(), event_type: EventType::Start, line: Some(segment.clone()), other: None });
        events.insert(Event { point: segment.end.clone(), event_type: EventType::End, line: Some(segment.clone()), other: None });

    }

    // // Ausgabe der Events
    // for event in &events {
    //     println!("{:?} {:?}", event.point, event.event_type);
    // }

    while let Some(event) = events.pop_first() {

        sweep_line.update_lines(event.point.clone());

        match event.event_type {
            EventType::Start => {
                // Add the line to the sweep line
                sweep_line.add_line(event.line.clone().unwrap());

                // Find the neighbors of the line
                let (before, after) = sweep_line.find_neighbors(&event.line.clone().unwrap());

                // Check for intersections
                if let Some(before) = before {
                    if let Some(intersection) = event.line.clone().unwrap().intersection(before) {                       
                        let new_intersection = Intersection { point: intersection.clone(), line: event.line.clone().unwrap(), other: before.clone() };
                        // intersections.push(new_intersection.clone());

                        // Create new event
                        let new_event = Event { point: intersection.clone(), event_type: EventType::Intersection, line: event.line.clone(), other: Some(before.clone()) };

                        // Check if the event already exists
                        // if !events.contains(&new_event) && !intersections.contains(&new_intersection) {
                        //     events.insert(new_event);
                        // }

                        events.insert(new_event);
                    }
                }

                if let Some(after) = after {
                    if let Some(intersection) = event.line.clone().unwrap().intersection(after) {
                        let new_intersection = Intersection { point: intersection.clone(), line: event.line.clone().unwrap(), other: after.clone() };
                        // intersections.push(new_intersection.clone());

                        // Create new event
                        let new_event = Event { point: intersection.clone(), event_type: EventType::Intersection, line: event.line.clone(), other: Some(after.clone()) };

                        // Check if the event already exists
                        // if !events.contains(&new_event) && !intersections.contains(&new_intersection) {
                        //     events.insert(new_event);
                        // }

                        events.insert(new_event);
                    }
                }
            },
            EventType::End => {
                // Find the neighbors of the line
                let (before, after) = sweep_line.find_neighbors(&event.line.clone().unwrap());

                // Check for intersections between the neighbors
                if let Some(before) = before {
                    if let Some(after) = after {
                        if let Some(intersection) = before.intersection(after) {
                            let new_intersection = Intersection { point: intersection.clone(), line: before.clone(), other: after.clone() };
                            // intersections.push(new_intersection.clone());

                            // Create new event
                            let new_event = Event { point: intersection.clone(), event_type: EventType::Intersection, line: Some(before.clone()), other: Some(after.clone()) };

                            // Check if the event already exists
                            if !events.contains(&new_event) && !intersections.contains(&new_intersection) {
                                events.insert(new_event);
                            }
                        }
                    }
                }

                // Remove the line from the sweep line
                //sweep_line.remove_line(event.line.unwrap());



                // Find line by id
                let remove_id = event.line.clone().unwrap().id;
                // println!("Remove id: {}", remove_id);

                // if remove_id == 606 {
                //     // Ausgabe sweep line
                //     println!("Sweep Line");
                //     for line in &sweep_line.lines {
                //         println!("{:?}", line);
                //     }
                // }

                let line = sweep_line.lines.iter().find(|x| x.id == remove_id).unwrap();
                sweep_line.remove_line(line.clone());
            },
            EventType::Intersection => {
                let new_intersection = Intersection { point: event.point.clone(), line: event.line.clone().unwrap(), other: event.other.clone().unwrap() };
                intersections.push(new_intersection.clone());


                let _index_line1 = sweep_line.lines.iter().position(|x| x.id == event.line.clone().unwrap().id).unwrap();
                let _index_other1 = sweep_line.lines.iter().position(|x| x.id == event.other.clone().unwrap().id).unwrap();


                // Swap lines
                let (line, other) = sweep_line.swap_lines(&event.line.clone().unwrap(), &event.other.clone().unwrap(), event.point.x);

                // index of line
                let _index_line2 = sweep_line.lines.iter().position(|x| x.id == line.id).unwrap();
                let _index_other2 = sweep_line.lines.iter().position(|x| x.id == other.id).unwrap();

                // let line = event.line.clone().unwrap();
                // let other = event.other.clone().unwrap();

                // Find the neighbors
                let (line_before, line_after) = sweep_line.find_neighbors(&line);
                let (other_before, other_after) = sweep_line.find_neighbors(&other);

                // let line_before = line_after;
                // let other_after = other_before;

                // Check for intersection between line and line_before
                if let Some(line_before) = line_before {
                    if let Some(intersection) = line.intersection(line_before) {
                        // let new_intersection = Intersection { point: intersection.clone(), line: line.clone(), other: line_before.clone() };
                        // intersections.push(new_intersection.clone());

                        // Create new event
                        let new_event = Event { point: intersection.clone(), event_type: EventType::Intersection, line: Some(line.clone()), other: Some(line_before.clone()) };

                        // Check if the event already exists
                        if !events.contains(&new_event) && !intersections.contains(&new_intersection) {
                            events.insert(new_event);
                        }
                    }
                }

                // Check for intersection between other and other_after
                if let Some(other_after) = other_after {
                    if let Some(intersection) = other.intersection(other_after) {
                        // let new_intersection = Intersection { point: intersection.clone(), line: other.clone(), other: other_after.clone() };
                        // intersections.push(new_intersection.clone());

                        // Create new event
                        let new_event = Event { point: intersection.clone(), event_type: EventType::Intersection, line: Some(other.clone()), other: Some(other_after.clone()) };

                        // Check if the event already exists
                        if !events.contains(&new_event) && !intersections.contains(&new_intersection) {
                            events.insert(new_event);
                        }
                    }
                }
                
            }
        }

    }

    // Remove duplicates from intersections
    let mut unique_intersections = Vec::new();
    for intersection in intersections {
        if !unique_intersections.contains(&intersection) {
            unique_intersections.push(intersection);
        }
    }

    unique_intersections.reverse();

    // Write intersections to file
    let mut file = std::fs::File::create("output_sweepline.txt").expect("Could not create file");
    for intersection in &unique_intersections {
        writeln!(file, "({}, {})", intersection.point.x.into_inner(), intersection.point.y.into_inner()).expect("Could not write to file");
    }

    println!("Anzahl der Schnittpunkte: {}", unique_intersections.len());
    unique_intersections.len()
}


// Unit test
#[cfg(test)]
mod tests {
    use crate::sweepline;

    use super::*;

    #[test]
    fn test_find_neighbors() {
        let mut collection = LineCollection::new();
        let line1 = Line { start: Point { x: OrderedFloat(2.0), y: OrderedFloat(1.0) }, end: Point { x: OrderedFloat(5.0), y: OrderedFloat(7.0) }, id: 0};
        let line2 = Line { start: Point { x: OrderedFloat(3.0), y: OrderedFloat(2.0) }, end: Point { x: OrderedFloat(6.0), y: OrderedFloat(8.0) }, id: 1};
        let line3 = Line { start: Point { x: OrderedFloat(4.0), y: OrderedFloat(3.0) }, end: Point { x: OrderedFloat(7.0), y: OrderedFloat(9.0) }, id: 2};
    
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
        let line0 = Line { start: Point { x: OrderedFloat(4.0), y: OrderedFloat(0.0) }, end: Point { x: OrderedFloat(7.0), y: OrderedFloat(9.0) }, id: 0};
        let line1 = Line { start: Point { x: OrderedFloat(2.0), y: OrderedFloat(1.0) }, end: Point { x: OrderedFloat(5.0), y: OrderedFloat(8.0) }, id: 1};
        let line2 = Line { start: Point { x: OrderedFloat(3.0), y: OrderedFloat(2.0) }, end: Point { x: OrderedFloat(6.0), y: OrderedFloat(7.0) }, id: 2};
        let line3 = Line { start: Point { x: OrderedFloat(4.0), y: OrderedFloat(3.0) }, end: Point { x: OrderedFloat(7.0), y: OrderedFloat(16.0) }, id: 3};
        let line4 = Line { start: Point { x: OrderedFloat(4.0), y: OrderedFloat(4.0) }, end: Point { x: OrderedFloat(7.0), y: OrderedFloat(15.0) }, id: 4};
        let line5 = Line { start: Point { x: OrderedFloat(4.0), y: OrderedFloat(5.0) }, end: Point { x: OrderedFloat(7.0), y: OrderedFloat(14.0) }, id: 5};
        let line6 = Line { start: Point { x: OrderedFloat(4.0), y: OrderedFloat(6.0) }, end: Point { x: OrderedFloat(7.0), y: OrderedFloat(29.0) }, id: 6};
        let line7 = Line { start: Point { x: OrderedFloat(4.0), y: OrderedFloat(7.0) }, end: Point { x: OrderedFloat(7.0), y: OrderedFloat(329.0) }, id: 7};
        let line8 = Line { start: Point { x: OrderedFloat(4.0), y: OrderedFloat(8.0) }, end: Point { x: OrderedFloat(7.0), y: OrderedFloat(1239.0) }, id: 8};
        let line9 = Line { start: Point { x: OrderedFloat(4.0), y: OrderedFloat(9.0) }, end: Point { x: OrderedFloat(7.0), y: OrderedFloat(4329.0) }, id: 9};
    
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
        let line1 = Line { start: Point { x: OrderedFloat(2.0), y: OrderedFloat(1.0) }, end: Point { x: OrderedFloat(5.0), y: OrderedFloat(7.0) }, id: 0};
        let line2 = Line { start: Point { x: OrderedFloat(3.0), y: OrderedFloat(2.0) }, end: Point { x: OrderedFloat(6.0), y: OrderedFloat(8.0) }, id: 1};
        let line3 = Line { start: Point { x: OrderedFloat(4.0), y: OrderedFloat(3.0) }, end: Point { x: OrderedFloat(7.0), y: OrderedFloat(9.0) }, id: 2};
    
        collection.add_line(line3.clone());
        collection.add_line(line1.clone());
        collection.add_line(line2.clone());

        let (before, after) = collection.find_neighbors(&line1);

        assert_eq!(before, None);
        assert_eq!(after, Some(&line2));

        let line4 = Line { start: Point { x: OrderedFloat(4.0), y: OrderedFloat(0.0) }, end: Point { x: OrderedFloat(7.0), y: OrderedFloat(9.0) }, id: 3};
        collection.add_line(line4.clone());

        let (before, after) = collection.find_neighbors(&line1);

        assert_eq!(before, Some(&line4));
        assert_eq!(after, Some(&line2));
    }

    #[test]
    fn test_find_neighbors4() {
        let mut collection = LineCollection::new();
        let line1 = Line { start: Point { x: OrderedFloat(2.0), y: OrderedFloat(1.0) }, end: Point { x: OrderedFloat(5.0), y: OrderedFloat(7.0) }, id: 0};
        let line2 = Line { start: Point { x: OrderedFloat(3.0), y: OrderedFloat(2.0) }, end: Point { x: OrderedFloat(6.0), y: OrderedFloat(8.0) }, id: 1};
        let line3 = Line { start: Point { x: OrderedFloat(4.0), y: OrderedFloat(3.0) }, end: Point { x: OrderedFloat(7.0), y: OrderedFloat(9.0) }, id: 2};
    
        collection.add_line(line3.clone());
        collection.add_line(line1.clone());
        collection.add_line(line2.clone());

        let (line1, line2) = collection.swap_lines(&line1, &line2, OrderedFloat(2.5));

        let (before, after) = collection.find_neighbors(&line1);

        assert_eq!(before, Some(&line2));
        assert_eq!(after, Some(&line3));

        // find by id
        let line = collection.lines.iter().find(|x| x.id == 1).unwrap();
        collection.remove_line(line.clone());

        // assert elements
        assert_eq!(collection.lines.len(), 2);
    }

    // Test für Event Reihenfolge
    #[test]
    fn test_event_order() {
        let p1 = Point { x: OrderedFloat(1.0), y: OrderedFloat(1.0) };
        let p2 = Point { x: OrderedFloat(2.0), y: OrderedFloat(1.0) };
        let p3 = Point { x: OrderedFloat(3.0), y: OrderedFloat(1.0) };
        let l1 = Line::new(Point { x: OrderedFloat(1.0), y: OrderedFloat(1.0) }, Point { x: OrderedFloat(2.0), y: OrderedFloat(2.0) }, 0);
        let l2 = Line::new(Point { x: OrderedFloat(2.0), y: OrderedFloat(2.0) }, Point { x: OrderedFloat(3.0), y: OrderedFloat(3.0) }, 1);
        let l3 = Line::new(Point { x: OrderedFloat(2.0), y: OrderedFloat(2.0) }, Point { x: OrderedFloat(3.0), y: OrderedFloat(3.0) }, 2);
        let e1 = Event { point: p3.clone(), event_type: EventType::Start, line: Some(l1), other: None };
        let e2 = Event { point: p2.clone(), event_type: EventType::Start, line: Some(l2), other: None };
        let e3 = Event { point: p1.clone(), event_type: EventType::Start, line: Some(l3), other: None };

        let mut events = BTreeSet::new();
        events.insert(e3);
        events.insert(e1);
        events.insert(e2);

        let mut iter = events.iter();
        assert_eq!(iter.next().unwrap().point, p1);
        assert_eq!(iter.next().unwrap().point, p2);
        assert_eq!(iter.next().unwrap().point, p3);

        let p4 = Point { x: OrderedFloat(1.5), y: OrderedFloat(1.0) };
        let e4 = Event { point: p4.clone(), event_type: EventType::Start, line: None, other: None };
        events.insert(e4);

        let mut iter = events.iter();
        assert_eq!(iter.next().unwrap().point, p1);
        assert_eq!(iter.next().unwrap().point, p4);
        assert_eq!(iter.next().unwrap().point, p2);
        assert_eq!(iter.next().unwrap().point, p3);


    }

    #[test]
    fn test_reihenfolge_nach_swap() {
        let mut collection = LineCollection::new();
        let line1 = Line { start: Point { x: OrderedFloat(1.0), y: OrderedFloat(3.0) }, end: Point { x: OrderedFloat(3.0), y: OrderedFloat(4.0) }, id: 0};
        let line2 = Line { start: Point { x: OrderedFloat(0.0), y: OrderedFloat(0.0) }, end: Point { x: OrderedFloat(4.0), y: OrderedFloat(1.0) }, id: 1};
        let line3 = Line { start: Point { x: OrderedFloat(2.5), y: OrderedFloat(2.0) }, end: Point { x: OrderedFloat(3.0), y: OrderedFloat(4.75) }, id: 2};
        let line4 = Line { start: Point { x: OrderedFloat(2.5), y: OrderedFloat(5.5) }, end: Point { x: OrderedFloat(4.5), y: OrderedFloat(3.0) }, id: 3};
        let line5 = Line { start: Point { x: OrderedFloat(2.75), y: OrderedFloat(3.0) }, end: Point { x: OrderedFloat(4.5), y: OrderedFloat(4.0) }, id: 4};
    
        collection.add_line(line3.clone());
        collection.add_line(line1.clone());
        collection.add_line(line4.clone());
        collection.add_line(line2.clone());
        collection.add_line(line5.clone());

        // collection.update_lines(Point { x: OrderedFloat(2.833), y: OrderedFloat(3.833) });

        // Get the first line
        let mut copy = collection.clone();
        let first = copy.lines.iter().next().unwrap();
        let second = copy.lines.iter().skip(1).next().unwrap();
        let third = copy.lines.iter().skip(2).next().unwrap();
        let fourth = copy.lines.iter().skip(3).next().unwrap();
        let five = copy.lines.iter().skip(4).next().unwrap();

        assert_eq!(first.id, 1);
        assert_eq!(second.id, 2);
        assert_eq!(third.id, 0);
        assert_eq!(fourth.id, 4);
        assert_eq!(five.id, 3);

        collection.update_lines(Point { x: OrderedFloat(2.84), y: OrderedFloat(3.833) });

        assert_eq!(first.id, 1);
        assert_eq!(second.id, 2);
        assert_eq!(third.id, 0);
        assert_eq!(fourth.id, 4);
        assert_eq!(five.id, 3);

        // Ausgabe mit iterator
        collection.lines.iter().for_each(|x| println!("{:?}", x));

        // Nachbarn von second
        let (before, after) = collection.find_neighbors(&second);

        assert_eq!(before.unwrap().id, 4);
        assert_eq!(after.unwrap().id, 0);

        // Nachbarn von third
        let (before, after) = collection.find_neighbors(&third);

        assert_eq!(before.unwrap().id, 2);
        assert_eq!(after.unwrap().id, 3);


        println!("-------------------\n");

        let (_, _) = collection.swap_lines(&second, &third, OrderedFloat(2.851));

        // Ausgabe mit iterator
        collection.lines.iter().for_each(|x| println!("{:?}", x));

        println!("-------------------\n");

        let first = collection.lines.iter().next().unwrap();
        let second = collection.lines.iter().skip(1).next().unwrap();
        let third = collection.lines.iter().skip(2).next().unwrap();
        let fourth = collection.lines.iter().skip(3).next().unwrap();
        let five = collection.lines.iter().skip(4).next().unwrap();

        assert_eq!(first.id, 1);
        assert_eq!(second.id, 4);
        assert_eq!(third.id, 0);
        assert_eq!(fourth.id, 2);
        assert_eq!(five.id, 3);

        // Nachbarn von second
        let (before, after) = collection.find_neighbors(&fourth);

        assert_eq!(before.unwrap().id, 0);
        assert_eq!(after.unwrap().id, 3);

        // Nachbarn von third
        let (before, after) = collection.find_neighbors(&third);

        assert_eq!(before.unwrap().id, 4);
        assert_eq!(after.unwrap().id, 2);

        
        // for line in &collection.lines {
        //     println!("{:?}\n", line);
        // }

        // println!("-------------------\n");
        // println!("{:?}\n", first);
        // println!("{:?}\n", second);
        // println!("{:?}\n", third);
        // println!("{:?}\n", fourth);

        // assert_eq!(first.id, 2);
        // assert_eq!(second.id, 2);
        // assert_eq!(third.id, 1);
        // assert_eq!(fourth.id, 3);
    }

    #[test]
    fn test_run() {
        assert_eq!(brute_force().len(), 796);
        assert_eq!(run(), 993);
    }
}

