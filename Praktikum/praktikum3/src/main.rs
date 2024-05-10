use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    
    let file_path = "strecken/s_1000_1.dat";
    //let file_path = "strecken/s_10000_1.dat";
    //let file_path = "strecken/s_100000_1.dat";
    let (points, _) = extract_points(file_path);

    let mut lines: Vec<Line> = points
    .into_iter()
    .map(|(start_x, start_y, end_x, end_y)| Line {
        start: Point { x: start_x, y: start_y },
        end: Point { x: end_x, y: end_y },
    })
    .collect();

    lines = vec![
        Line { start: Point { x: 1.0, y: 1.0 }, end: Point { x: 5.0, y: 5.0 } },
        Line { start: Point { x: 2.0, y: 4.0 }, end: Point { x: 6.0, y: 1.0 } },
        Line { start: Point { x: 3.0, y: 2.0 }, end: Point { x: 7.0, y: 6.0 } },
    ];

    // Berechne die Schnittpunkte der Linien
    let intersections = sweep_line_algorithm(&lines);

    // Gib die Schnittpunkte aus
    for intersection in intersections {
        println!("Intersection: {:?}", intersection);
    }
}

fn extract_points(file_path: &str) -> (Vec<(f64, f64, f64, f64)>, String) {
    let mut points: Vec<(f64, f64, f64, f64)> = Vec::new();
    let mut info_message = String::new();
    info_message.push_str("Ungültige Zeilen: ");

    if let Ok(file) = File::open(file_path) {
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                let mut values = line.split_whitespace().filter_map(|s| s.parse::<f64>().ok());

                if let (Some(x1), Some(y1), Some(x2), Some(y2)) = (values.next(), values.next(), values.next(), values.next()) {
                    points.push((x1, y1, x2, y2));
                } else {
                    println!("Ungültige Zeile übersprungen: {}", line);
                    info_message.push_str(&line.to_string());
                    info_message.push_str(", ");
                }
            }
        }
    } else {
        panic!("Datei konnte nicht geöffnet werden: {}", file_path);
    }

    info_message.push_str("\n");

    (points, info_message)
}

// Definition eines Punktes
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

// Definition einer Linie
#[derive(Debug, Clone, Copy)]
struct Line {
    start: Point,
    end: Point,
}

// Hilfsstruktur zum Halten von Informationen über eine Linie, die von der Sweep-Line durchquert wird
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Event {
    StartLine { index: usize, point: Point },
    EndLine { index: usize, point: Point },
}

// Hilfsfunktion zum Vergleich von Fließkommazahlen mit einer Genauigkeitstoleranz
fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < f64::EPSILON
}

// Implement Eq trait for Point struct
impl Eq for Point {}

// Implement PartialEq trait for Point struct
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        approx_eq(self.x, other.x) && approx_eq(self.y, other.y)
    }
}

// Hilfsfunktion zur Überprüfung, ob ein Punkt auf einer Linie liegt
fn point_on_line(point: Point, line: Line) -> bool {
    let dx1 = line.end.x - line.start.x;
    let dy1 = line.end.y - line.start.y;
    let dx2 = point.x - line.start.x;
    let dy2 = point.y - line.start.y;
    approx_eq(dx1 * dy2, dy1 * dx2)
}

// Hilfsfunktion zur Überprüfung, ob ein Punkt auf einer horizontalen Linie liegt
fn point_on_horizontal_line(point: Point, line: Line) -> bool {
    approx_eq(point.y, line.start.y) && ((point.x >= line.start.x && point.x <= line.end.x) || (point.x >= line.end.x && point.x <= line.start.x))
}

// Hilfsfunktion zur Überprüfung, ob ein Punkt auf einer vertikalen Linie liegt
fn point_on_vertical_line(point: Point, line: Line) -> bool {
    approx_eq(point.x, line.start.x) && ((point.y >= line.start.y && point.y <= line.end.y) || (point.y >= line.end.y && point.y <= line.start.y))
}

// Hilfsfunktion zur Berechnung des Schnittpunktes zweier Linien
fn calculate_intersection_point(line1: Line, line2: Line) -> Option<Point> {
    let x1 = line1.start.x;
    let y1 = line1.start.y;
    let x2 = line1.end.x;
    let y2 = line1.end.y;
    let x3 = line2.start.x;
    let y3 = line2.start.y;
    let x4 = line2.end.x;
    let y4 = line2.end.y;

    let denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

    if approx_eq(denominator, 0.0) {
        None // Linien sind parallel oder identisch
    } else {
        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denominator;
        let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / denominator;

        if t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0 {
            Some(Point {
                x: x1 + t * (x2 - x1),
                y: y1 + t * (y2 - y1),
            })
        } else {
            None // Schnittpunkt liegt außerhalb der Linienabschnitte
        }
    }
}

// Hauptfunktion des Sweep-Line-Algorithmus zur Berechnung der Schnittpunkte
fn sweep_line_algorithm(lines: &[Line]) -> Vec<Point> {
    let mut events: Vec<Event> = Vec::new();

    // Erzeuge Ereignisse für den Anfang und das Ende jeder Linie
    for (index, &line) in lines.iter().enumerate() {
        if line.start.y > line.end.y {
            events.push(Event::StartLine {
                index,
                point: line.end,
            });
            events.push(Event::EndLine {
                index,
                point: line.start,
            });
        } else {
            events.push(Event::StartLine {
                index,
                point: line.start,
            });
            events.push(Event::EndLine {
                index,
                point: line.end,
            });
        }
    }

    // Sortiere die Ereignisse nach y-Koordinate
    events.sort_by(|a, b| {
        let ay = match a {
            Event::StartLine { point, .. } => point.y,
            Event::EndLine { point, .. } => point.y,
        };
        let by = match b {
            Event::StartLine { point, .. } => point.y,
            Event::EndLine { point, .. } => point.y,
        };
        ay.partial_cmp(&by).unwrap()
    });

    let mut active_lines: Vec<usize> = Vec::new();
    let mut intersections: Vec<Point> = Vec::new();

    for event in events {
        match event {
            Event::StartLine { index, .. } => {
                active_lines.push(index);
            }
            Event::EndLine { index, point } => {
                active_lines.retain(|&i| i != index);
                for &i in &active_lines {
                    if let Some(intersection_point) = calculate_intersection_point(lines[i], Line { start: point, end: point }) {
                        intersections.push(intersection_point);
                    }
                }
            }
        }
    }

    intersections
}