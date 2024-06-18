
use std::{collections::{BinaryHeap, HashSet}, io::Write};

use praktikum3::{event::Event, event::EventType, line::Line, point::Point, sweepline::SweepLine};
use rand::Rng;

struct Intersection {
    point: Point,
    line: Line,
    other: Line,
}

impl Intersection {
    fn new(point: Point, line: Line, other: Line) -> Self {
        Self {
            point,
            line,
            other,
        }
    }
}

// main function
fn main() {
    println!("Hello, world!");

    // let segments = vec![
    //     Line { start: Point { x: -1.0, y: 1.0 }, end: Point { x: 5.0, y: 1.0 }},
    //     Line { start: Point { x: 1.0, y: 0.0 }, end: Point { x: 4.0, y: 4.0 }},
    //     Line { start: Point { x: 2.0, y: 4.0 }, end: Point { x: 5.0, y: 0.0 }},
    // ];

    // let file_path = "strecken/s_1000_1.dat";
    let file_path = "strecken/s_10000_1.dat";
    // let file_path = "strecken/s_100000_1.dat";
    let segments = read_segments_from_file(file_path);

    let intersectios_brute_force = check_intersections(&segments);
    println!("Brute Force Schnittpunkte: {}", intersectios_brute_force.len());
    write_intersections_to_file(intersectios_brute_force, "brute_force_intersections.dat");


    // let segments = generate_non_parallel_line_segments(10, 0.0, 100.0, 0.0, 100.0);

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

                // Above and below intersection
                if let Some(above) = above {
                    if let Some(below) = below {
                        // check for intersection
                        let intersection = above.line.intersection(below.line);
                        if let Some(intersection) = intersection {
                            events.push(Event {
                                point: intersection,
                                event_type: EventType::Intersection,
                                line: Some(above.line),
                                other: Some(below.line)
                            });
                        }
                    }
                }

                SL.remove_line(event.line.unwrap());

            },
            EventType::Intersection => {
                intersections.push(Intersection{
                    point: event.point,
                    line: event.line.unwrap(),
                    other: event.other.unwrap()
                
                });
                // Verify?
                let (segE1, segE2) = SL.swap_lines(event.line.unwrap(), event.other.unwrap(), event.point);

                let (segE1_above, segE1_below) = SL.get_neighbors(segE1);
                let (segE2_above, segE2_below) = SL.get_neighbors(segE2);

                // Intersection zwischen segE1_above und segE2_below
                if let Some(segE1_above) = segE1_above {
                    if let Some(segE2_below) = segE2_below {
                        let intersection = segE1_above.line.intersection(segE2_below.line);
                        if let Some(intersection) = intersection {
                            let new_event = Event {
                                point: intersection,
                                event_type: EventType::Intersection,
                                line: Some(segE1_above.line),
                                other: Some(segE2_below.line)
                            };

                            if !contains_event(&events, &new_event) && new_event.point != event.point && new_event.line != event.other && new_event.other != event.other {
                                events.push(new_event);
                            }
                        }
                    }
                }



                // Intersection zwischen segE1_below und segE2_above
                if let Some(segE1_below) = segE1_below {
                    if let Some(segE2_above) = segE2_above {
                        let intersection = segE1_below.line.intersection(segE2_above.line);
                        if let Some(intersection) = intersection {
                            let new_event = Event {
                                point: intersection,
                                event_type: EventType::Intersection,
                                line: Some(segE1_below.line),
                                other: Some(segE2_above.line)
                            };

                            if !contains_event(&events, &new_event) && new_event.point != event.point && new_event.line != event.other && new_event.other != event.other {
                                events.push(new_event);
                            }
                        }
                    }
                }


                // Intersection zwischen segE2_below und segE2
                if let Some(segE2_below) = segE2_below {
                    let intersection = segE2_below.line.intersection(segE2);
                    if let Some(intersection) = intersection {
                        let new_event = Event {
                            point: intersection,
                            event_type: EventType::Intersection,
                            line: Some(segE2_below.line),
                            other: Some(segE2)
                        };

                        if !contains_event(&events, &new_event) && new_event.point != event.point && new_event.line != event.other && new_event.other != event.other {
                            events.push(new_event);
                        }
                    }
                }

                // Intersection zwischen segE2 und segE2_above
                if let Some(segE2_above) = segE2_above {
                    let intersection = segE2.intersection(segE2_above.line);
                    if let Some(intersection) = intersection {
                        let new_event = Event {
                            point: intersection,
                            event_type: EventType::Intersection,
                            line: Some(segE2),
                            other: Some(segE2_above.line)
                        };

                        if !contains_event(&events, &new_event) && new_event.point != event.point && new_event.line != event.other && new_event.other != event.other {
                            events.push(new_event);
                        }
                    }
                }

                // Intersection zwischen segE1_above und segE1
                if let Some(segE1_above) = segE1_above {
                    let intersection = segE1_above.line.intersection(segE1);
                    if let Some(intersection) = intersection {
                        let new_event = Event {
                            point: intersection,
                            event_type: EventType::Intersection,
                            line: Some(segE1_above.line),
                            other: Some(segE1)
                        };

                        if !contains_event(&events, &new_event) && new_event.point != event.point && new_event.line != event.other && new_event.other != event.other {
                            events.push(new_event);
                        }
                    }
                }

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

                        if !contains_event(&events, &new_event) && new_event.point != event.point && new_event.line != event.other && new_event.other != event.other {
                            events.push(new_event);
                        }
                    }
                }
            }
        }
    }

    println!("Done");
    println!("Schnittpunkte: {}", intersections.len());
    write_intersections_to_file(intersections, "sweep_line_intersections.dat");

}

fn contains_event(heap: &BinaryHeap<Event>, event: &Event) -> bool {
    // Prüfung, ob event in heap vorhanden ist
    for e in heap.iter() {
        if e.point == event.point {
            return true;
        }
    }

    return false;
}

fn read_segments_from_file(filename: &str) -> Vec<Line> {
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

        // Kein Punkt
        if start == end {
            continue;
        }

        if start.x == end.x {
            continue;
        }

        if start.x < end.x {
            segments.push(Line {
                start: Point { x: x1, y: y1 },
                end: Point { x: x2, y: y2 },
            });
        } else {
            segments.push(Line {
                start: Point { x: x2, y: y2 },
                end: Point { x: x1, y: y1 },
            });
        }

    }

    // extract fileName from path
    let filename = filename.split("/").last().unwrap();

    // safe segments to new file
    let mut file = std::fs::File::create("strecken/clean_".to_owned() + filename).expect("Could not create file");
    for segment in &segments {
        let line = format!("{} {} {} {}\n", segment.start.x, segment.start.y, segment.end.x, segment.end.y);
        file.write_all(line.as_bytes()).expect("Could not write to file");
    }
    

    segments
}

fn generate_random_point(x_min: f64, x_max: f64, y_min: f64, y_max: f64) -> Point {
    let mut rng = rand::thread_rng();
    Point {
        x: rng.gen_range(x_min..x_max),
        y: rng.gen_range(y_min..y_max),
    }
}

fn generate_non_parallel_line_segments(n: usize, x_min: f64, x_max: f64, y_min: f64, y_max: f64) -> Vec<Line> {
    let mut rng = rand::thread_rng();
    let mut segments = Vec::new();
    let mut x_coords = Vec::new();

    while segments.len() < n {
        let start = generate_random_point(x_min, x_max, y_min, y_max);
        let mut end;

        loop {
            end = generate_random_point(x_min, x_max, y_min, y_max);
            // Ensure the end point is not the same as the start point and the segment is not vertical
            if end.x != start.x && end.y != start.y && !x_coords.contains(&end.x) {
                break;
            }
        }

        // Ensure the x-coordinates are unique for the segment
        if x_coords.contains(&start.x) && x_coords.contains(&end.x) {
            x_coords.push(start.x);
            x_coords.push(end.x);
            // Ensure the length of the segment is greater than 0
            if ((end.x - start.x).powi(2) + (end.y - start.y).powi(2)).sqrt() > 0.0 {
                segments.push(Line { start, end });
            }
        }
    }

    segments
}


// Funktion, um jedes Segment zu überprüfen, ob es sich mit einem anderen schneidet
fn check_intersections(segments: &Vec<Line>) -> Vec<Intersection> {
    let mut intersections = Vec::new();

    for i in 0..segments.len() {
        for j in i+1..segments.len() {
            let intersection = segments[i].intersection(segments[j]);
            if let Some(intersection) = intersection {
                intersections.push(Intersection::new(intersection, segments[i], segments[j]));
            }
        }
    }

    intersections
}

// Funktion, um Schnittpunkte in File auszugeben
fn write_intersections_to_file(intersections: Vec<Intersection>, filename: &str) {
    let mut file = std::fs::File::create(filename).expect("Could not create file");

    // Sortiere nach x
    let mut intersections = intersections;
    intersections.sort_by(|a, b| a.point.x.partial_cmp(&b.point.x).unwrap());

    // Duplikate entfernen
    let mut set = HashSet::new();
    intersections.retain(|e| set.insert(e.point));

    // Anzahl der Schnittpunkte auf Console
    println!("{} - Schnittpunkte: {}", filename, intersections.len());

    for intersection in intersections {
        let line = format!("{} {} {} {}\n", intersection.point.x, intersection.point.y, intersection.line.start.y, intersection.line.end.y);
        file.write_all(line.as_bytes()).expect("Could not write to file");
    }
}