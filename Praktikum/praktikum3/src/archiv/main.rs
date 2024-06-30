
use std::{collections::{BinaryHeap, HashSet}, io::Write};

use praktikum3::{event::{Event, EventType}, line::Line, point::Point, sweepline::{SweepLine, SweepLineEntry}};
use rand::Rng;

#[derive(Debug, Clone, PartialEq, Copy)]
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

    // let segments = vec![
    //     Line { start: Point { x: 91.1050, y: 22.0320 }, end: Point { x: 91.1120, y: 22.6269 }},
    //     Line { start: Point { x: 90.5472, y: 22.7331 }, end: Point { x: 91.1570, y: 22.4220 }},
    //     Line { start: Point { x: 90.7466, y: 22.0581 }, end: Point { x: 91.5450, y: 21.3290 }},
    //     Line { start: Point { x: 90.8549, y: 22.0544 }, end: Point { x: 91.2730, y: 21.5360 }},
    //     Line { start: Point { x: 90.5983, y: 22.2864 }, end: Point { x: 90.6610, y: 21.9340 }},
    //     ];

    // let segments = vec![
    //     Line { start: Point { x: 95.1953, y: 3.0631 }, end: Point { x: 95.9980, y: 2.7090 }},
    //     Line { start: Point { x: 95.6477, y: 2.9355 }, end: Point { x: 96.4390, y: 2.0690 }},
    //     Line { start: Point { x: 95.6169, y: 2.6209 }, end: Point { x: 96.3960, y: 2.1100 }},
    //     Line { start: Point { x: 96.0290, y: 2.5780 }, end: Point { x: 96.0490, y: 1.6930 }},
    //     Line { start: Point { x: 96.0510, y: 2.3104 }, end: Point { x: 96.4070, y: 1.6330 }},
    // ];

    // let file_path = "strecken/s_1000_1.dat";
    //let file_path = "G:\\Git\\computational-geometry\\Praktikum\\praktikum3\\strecken\\s_1000_1.dat";
    // let file_path = "G:\\Git\\computational-geometry\\Praktikum\\praktikum3\\strecken\\s_100000_1.dat";
    // let file_path = "strecken/s_big.dat";
    let file_path = "G:\\Git\\computational-geometry\\Praktikum\\praktikum3\\strecken\\s_1000_10.dat";
    let segments = read_segments_from_file(file_path);

    // Start Zeitmessung
    let start = std::time::Instant::now();

    let intersectios_brute_force = check_intersections(&segments);

    // Ende Zeitmessung
    let end = std::time::Instant::now();

    println!("Brute Force Zeit: {:?}", end - start);
    println!("Brute Force Schnittpunkte: {}", intersectios_brute_force.len());
    write_intersections_to_file(intersectios_brute_force.clone(), "brute_force_intersections.dat");


    let start = std::time::Instant::now();

    let mut events = BinaryHeap::new();

    for item in segments.clone() {
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

        SL.pinpoint_points(event.point);

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
                            let new_event = Event {
                                point: intersection,
                                event_type: EventType::Intersection,
                                line: Some(above.line),
                                other: Some(below.line)
                            };

                            if !contains_event(&events, &new_event) && new_event.point != event.point && new_event.line != event.other && new_event.other != event.other {
                                events.push(new_event);
                            }
                        }
                    }
                }

                SL.remove_line(event.line.unwrap());
            },
            EventType::Intersection => {
                let new_intersection = Intersection{
                    point: event.point,
                    line: event.line.unwrap(),
                    other: event.other.unwrap()
                
                };

                // Verify?
                let (segE1, segE2) = SL.swap_lines(event.line.unwrap(), event.other.unwrap(), event.point);

                let (segE1_above, segE1_below) = SL.get_neighbors(segE1);
                let (segE2_above, segE2_below) = SL.get_neighbors(segE2);

                
                if intersections.contains(&new_intersection) {
                    continue;
                }

                intersections.push(new_intersection);

                // // // Intersection zwischen segE1 und segE1_below
                // if let Some(segE1_below) = segE1_below {

                //     if segE1_below.line != segE1 {

                //         let intersection = segE1.intersection(segE1_below.line);
                //         if let Some(intersection) = intersection {
                //             let new_event = Event {
                //                 point: intersection,
                //                 event_type: EventType::Intersection,
                //                 line: Some(segE1),
                //                 other: Some(segE1_below.line)
                //             };

                //             if !contains_event(&events, &new_event) && new_event.point != event.point && new_event.line != event.other && new_event.other != event.other {
                //                 events.push(new_event);
                //             }
                //         }
                //     }
                // }

                // // Intersection zwischen segE1 und segE1_above
                // if let Some(segE1_above) = segE1_above {

                //     if segE1_above.line != segE1 {

                //         let intersection = segE1.intersection(segE1_above.line);
                //         if let Some(intersection) = intersection {
                //             let new_event = Event {
                //                 point: intersection,
                //                 event_type: EventType::Intersection,
                //                 line: Some(segE1),
                //                 other: Some(segE1_above.line)
                //             };

                //             if !contains_event(&events, &new_event) && new_event.point != event.point && new_event.line != event.other && new_event.other != event.other {
                //                 events.push(new_event);
                //             }
                //         }
                //     }
                // }

                // // Intersection zwischen segE2 und segE2_below
                // if let Some(segE2_below) = segE2_below {

                //     if segE2_below.line != segE2 {

                //         let intersection = segE2.intersection(segE2_below.line);
                //         if let Some(intersection) = intersection {
                //             let new_event = Event {
                //                 point: intersection,
                //                 event_type: EventType::Intersection,
                //                 line: Some(segE2),
                //                 other: Some(segE2_below.line)
                //             };

                //             if !contains_event(&events, &new_event) && new_event.point != event.point && new_event.line != event.other && new_event.other != event.other {
                //                 events.push(new_event);
                //             }
                //         } 
                //     }
                // }

                
                // // // Intersection zwischen segE2 und segE2_above
                // if let Some(segE2_above) = segE2_above {
                                        
                //     if segE2_above.line != segE2 {
                //         let intersection = segE2.intersection(segE2_above.line);
                //         if let Some(intersection) = intersection {
                //             let new_event = Event {
                //                 point: intersection,
                //                 event_type: EventType::Intersection,
                //                 line: Some(segE2),
                //                 other: Some(segE2_above.line)
                //             };

                //             if !contains_event(&events, &new_event) && new_event.point != event.point && new_event.line != event.other && new_event.other != event.other {
                //                 events.push(new_event);
                //             }
                //         }
                //     }
                // }

            }
        }
    }
    // Ende Zeitmessung
    let end = std::time::Instant::now();

    println!("Sweep Line Zeit: {:?}", end - start);
    write_intersections_to_file(intersections.clone(), "sweep_line_intersections.dat");

    // // // Sortiere nach x
    // // let mut intersections = intersections;
    // // intersections.sort_by(|a, b| a.point.x.partial_cmp(&b.point.x).unwrap());

    // // // Duplikate entfernen
    // // let mut set = HashSet::new();
    // // intersections.retain(|e| set.insert(e.point));


    // // // Sortiere nach x
    // // let mut bf_intersections = intersectios_brute_force.clone();
    // // bf_intersections.sort_by(|a, b| a.point.x.partial_cmp(&b.point.x).unwrap());

    // // // Duplikate entfernen
    // // let mut set = HashSet::new();
    // // bf_intersections.retain(|e| set.insert(e.point));


    // // // Vector mit allen Schnittpunkten gerundet
    // // let mut rounded_intersections = Vec::new();
    // // for intersection in intersections.clone() {
    // //     rounded_intersections.push(Intersection {
    // //         point: intersection.point.round(),
    // //         line: intersection.line,
    // //         other: intersection.other
    // //     });
    // // }

    // // let mut bf_rounded_intersections = Vec::new();
    // // for intersection in intersectios_brute_force.clone() {
    // //     bf_rounded_intersections.push(Intersection {
    // //         point: intersection.point.round(),
    // //         line: intersection.line,
    // //         other: intersection.other
    // //     });
    // // }

    // // // // Fehlende Schnittpunkte von bf_rounded_intersections in rounded_intersections finden
    // // // let mut missing_intersections = Vec::new();
    // // // for intersection in bf_rounded_intersections {
    // // //     if !rounded_intersections.contains(&intersection) {
    // // //         missing_intersections.push(intersection);
    // // //     }
    // // // }

    // // // Finde alle Schnittpunkte, die in bf_rounded_intersections und rounded_intersections sind
    // // let mut common_intersections = Vec::new();
    // // for intersection in bf_rounded_intersections {
    // //     if rounded_intersections.contains(&intersection) {
    // //         common_intersections.push(intersection);
    // //     }
    // // }

    // // // Ausgabe der Linie der fehlenden Schnittpunkte in File
    // // let mut file = std::fs::File::create("missing_intersections.dat").expect("Could not create file");
    // // for intersection in common_intersections {
    // //     let line = format!("{} {} {} {}\n", intersection.line.start.x, intersection.line.start.y, intersection.line.end.x, intersection.line.end.y);
    // //     file.write_all(line.as_bytes()).expect("Could not write to file");
    // // }

    // // let remover = read_segments_from_file("missing_intersections.dat");

    // // // Enterne remover von segments
    // // let mut segments = segments.clone();
    // // for segment in remover {
    // //     segments.retain(|x| x != &segment);
    // // }

    // // // Write segments to file
    // // let mut file = std::fs::File::create("clean_segments.dat").expect("Could not create file");
    // // for segment in segments {
    // //     let line = format!("{} {} {} {}\n", segment.start.x, segment.start.y, segment.end.x, segment.end.y);
    // //     file.write_all(line.as_bytes()).expect("Could not write to file");
    // // }

}


fn contains_event(heap: &BinaryHeap<Event>, event: &Event) -> bool {
    // Prüfung, ob event in heap vorhanden ist
    for e in heap.iter() {
        if e.point == event.point {
            return true;
        }

        
        if e.line == event.line && e.other == event.other {
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

        if start.x == 62.462 && start.y == 76.608 {
            continue;
        }

        if start.x == 10.649 && start.y == 2.807 {
            continue;
        }

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

    // // extract fileName from path
    // let filename = filename.split("/").last().unwrap();

    // // safe segments to new file
    // let mut file = std::fs::File::create("strecken/clean_".to_owned() + filename).expect("Could not create file");
    // for segment in &segments {
    //     let line = format!("{} {} {} {}\n", segment.start.x, segment.start.y, segment.end.x, segment.end.y);
    //     file.write_all(line.as_bytes()).expect("Could not write to file");
    // }
    

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

    // Doppelte Schleife über alle intersections
    for i in 0..intersections.len() {
        for j in i+1..intersections.len() {
            if intersections[i].line == intersections[j].line && intersections[i].other == intersections[j].other {
                // Enterne doppelte Schnittpunkte
                intersections.remove(j);
            }
        }
    }



    // Anzahl der Schnittpunkte auf Console
    println!("{} - Schnittpunkte: {}", filename, intersections.len());

    for intersection in intersections {
        let line = format!("{} {}; Linie: {} {}\n", intersection.point.x.round(), intersection.point.y.round(), intersection.line.start.x, intersection.line.start.y); //, Schnitt: Line1-Ende: {} {}; Line2-Ende: {} {}\n", intersection.point.x, intersection.point.y, intersection.line.end.x, intersection.line.end.y, intersection.other.end.x, intersection.other.end.y);
        file.write_all(line.as_bytes()).expect("Could not write to file");
    }
} 