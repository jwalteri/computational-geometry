
use std::{collections::BTreeSet, time::Instant};

use praktikum3::{event::{Event, EventType}, line::Line, point::Point, sweepline::SweepLine};

fn main() {

    let file_paths = vec![
        "G:\\Git\\computational-geometry\\Praktikum\\praktikum3\\strecken\\s_1000_10.dat",
        r"G:\Git\computational-geometry\Praktikum\praktikum3\src\testfiles\jw_1000_10.txt",
        r"G:\Git\computational-geometry\Praktikum\praktikum3\src\testfiles\jw_10000_10.txt",
        r"G:\Git\computational-geometry\Praktikum\praktikum3\src\testfiles\jw_100000_10.txt",
        "G:\\Git\\computational-geometry\\Praktikum\\praktikum3\\strecken\\s_1000_1.dat",

    ];

    // let segments = read_segments_from_file(r"G:\Git\computational-geometry\Praktikum\praktikum3\strecken\s_1000_10.dat");
    // let segments = read_segments_from_file(r"G:\Git\cg-3-sweep-line\data\jw_1000_10.txt");

    for file_path in file_paths {
        println!("File: {}", file_path);
        let segments = read_segments_from_file(file_path);

        let start_brute = Instant::now();
        let intersections = brute_force(&segments);
        let brute = start_brute.elapsed();
        println!("Anzahl der Schnittpunkte: {}", intersections.len());
        println!("Brute Force Zeit: {:?}", brute);

        let mut events: BTreeSet<Event> = BTreeSet::new();
        let mut intersection_points: BTreeSet<Point> = BTreeSet::new();



        for item in segments.clone() {
            events.insert(Event {
                point: item.clone().start,
                event_type: EventType::Start,
                line: Some(item.clone()),
                other: None
            });
            events.insert(Event {
                point: item.clone().end,
                event_type: EventType::End,
                line: Some(item),
                other: None
            });
        }

        let start_sweep = Instant::now();

        let mut sweep_line = SweepLine::new();

            while let Some(event) = events.pop_first() {
                sweep_line.update(event.point.x);
                match event.event_type {
                    EventType::Start => {

                        let line = event.line.unwrap();
                        sweep_line.insert(event.point.y, line.clone());

                        let (below, above) = sweep_line.get_neighbors(&line.clone());

                        if let Some(line_above) = above {
                            if let Some(intersection_point) = line.intersection(&line_above.line) {
                                add_event(intersection_point, &line, &line_above.line, &mut events, &mut intersection_points)
                            };
                        };

                        if let Some(line_below) = below {
                            if let Some(intersection_point) = line.intersection(&line_below.line) {
                                add_event(intersection_point, &line, &line_below.line, &mut events, &mut intersection_points)
                            };
                        };
                    }
                    EventType::End => {
                        let line = event.line.unwrap();
                        let (below, above) = sweep_line.get_neighbors(&line);

                        if let (Some(line_below), Some(line_above)) = (below, above)
                        {
                            if let Some(intersection_point) = line_below.line.intersection(&line_above.line) {
                                add_event(intersection_point, &line_below.line, &line_above.line, &mut events, &mut intersection_points)
                            };
                        };

                        sweep_line.remove_by_line(&line);
                    }
                    EventType::Intersection => {
                        let line = event.line.unwrap();
                        let other_line = event.other.unwrap();
                        let intersection_point = event.point;

                        let (below, lower, higher, above) = sweep_line.swap(
                            &line,
                            &other_line,
                            &intersection_point,
                        );
                        

                        if let (line, Some(line_above)) = (higher, above) {
                            if let Some(intersection_point) = line.line.intersection(&line_above.line) {
                                add_event(intersection_point, &line.line, &line_above.line, &mut events, &mut intersection_points)
                            };
                        };

                        if let (line, Some(line_below)) = (lower, below) {
                            if let Some(intersection_point) = line.line.intersection(&line_below.line) {
                                add_event(intersection_point, &line.line, &line_below.line, &mut events, &mut intersection_points)
                            };
                        };

                    }
                };
            }

        let swept = start_sweep.elapsed();

        println!("Anzahl der Schnittpunkte: {}", intersection_points.len());
        println!("SweepLine Zeit: {:?}", swept);
    }  
}

fn add_event(intersection_point: Point, line: &Line, other_line: &Line, events: &mut BTreeSet<Event>, intersection_points: &mut BTreeSet<Point>) {
    let intersection_point = intersection_point.round(9);

        if !intersection_points.contains(&intersection_point)
        {
            intersection_points.insert(intersection_point.clone());

            let intersection = Event::new(
                intersection_point,
                EventType::Intersection,
                Some(line.clone()),
                Some(other_line.clone()),
            );

            events.insert(intersection);
        }
}

fn brute_force(segments: &Vec<Line>) -> Vec<Point> {
    let mut intersections = Vec::new();

    for i in 0..segments.len() {
        for j in i+1..segments.len() {
            let intersection = segments[i].intersection(&segments[j]);
            if let Some(intersection) = intersection {
                intersections.push(intersection);
            }
        }
    }

    intersections
}

fn read_segments_from_file(filename: &str) -> Vec<Line> {
    let mut segments = Vec::new();
    let file = std::fs::read_to_string(filename).expect("Could not read file");
    for line in file.lines() {
        let mut parts = line.split_whitespace();
        let x1: f64 = parts.next().unwrap().parse().unwrap();
        let y1: f64 = parts.next().unwrap().parse().unwrap();
        let x2: f64 = parts.next().unwrap().parse().unwrap();
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

    segments
}
