// Computational Geometry Praktikum
// Aufgabe 1
// SS24 @ University of Applied Sciences Munich
// Copyright: J. Walter, L. Biege


use core::panic;
use std::time::Instant;
use std::fs::File;
use std::io::{BufRead, BufReader};

use geo::{Intersects, Line};


fn main() {

    // let file_path = "strecken/s_1000_1.dat";
    let file_path = "strecken/s_10000_1.dat";
    // let file_path = "strecken/s_100000_1.dat";

    // Geo-Lib
    // Berechnung der Schnittpunkte mithilfe der Geo-Lib
    // Hinweis im Praktikum gehört, dass Geo-Lib für die Berechnung der Schnittpunkte verwendet werden kann

    // Zeitmessung
    let geo_read_file_start = Instant::now();
    let points_2d = extract_points_2d(file_path);
    let geo_read_file_end = Instant::now();
    let geo_read_duration = geo_read_file_end - geo_read_file_start;

    println!("Datei eingelesen (Geo-Lib).");
    println!("\tDauer: {} ms.", geo_read_duration.as_millis());

    let geo_start_time = Instant::now();

    let mut geo_count = 0;
    for i in 0..points_2d.len() {
        for j in i+1..points_2d.len() {
            let intersection = points_2d[i].intersects(&points_2d[j]);
            if intersection {
                geo_count += 1;
            }
        }
    }

    // Zeitmessung
    let geo_end_time = Instant::now();

    println!("{} Schnittpunkte gefunden (Geo-Lib).", geo_count);
    let geo_duration = geo_end_time - geo_start_time;
    println!("\tDauer: {} ms.", geo_duration.as_millis());

    //
    // Unsere Implementierung
    //
    let file_time_start = Instant::now();

    // Logging mithilfe von info_message
    let mut info_message = String::new();
    // Einlesen der Datei
    let (points, updated_info_message) = extract_points(file_path);
    let segments = &convert_points_to_segments(&points);
    let file_time_end = Instant::now();
    let file_duration = file_time_end - file_time_start;

    println!("Datei eingelesen (Unsere Implementierung).");
    println!("\tDauer: {} ms.", file_duration.as_millis());

    let algo_time_start = Instant::now();
    // Berechnung der Schnittpunkte
    let intersections = get_intersections(segments); //calculate_intersections(&points);
    let algo_time_end = Instant::now();
    let algo_duration = algo_time_end - algo_time_start;

    println!("{} Schnittpunkte gefunden (Unsere Implementierung).", intersections);
    println!("\tDauer: {} ms.", algo_duration.as_millis());

}

///
/// Zwei Linien liegen auf der selben Linie -> colinear
/// Um Schnittpunkt zu erkennen:
/// Prüft, ob ein Segment im Bereich eines anderen Segments liegt
/// Quellen: 
/// - Hinweis auf Notwendigkeit im Praktikum bekommen
/// - https://docs.rs/intersect2d/
/// - https://stackoverflow.com/questions/563198/how-do-you-detect-where-two-line-segments-intersect/565282#565282
fn colinear_intersection(p1: &Point, p2: &Point, q1: &Point, q2: &Point) -> bool {

    // Überlappen sich die x-Koordinaten der Segmente?
    if q1.x.min(q2.x) >= p1.x.min(p2.x) && q1.x.min(q2.x) <= p1.x.max(p2.x) {
        return true;
    }

    // Überlappen sich die y-Koordinaten der Segmente?
    if q1.y.min(q2.y) <= p1.y.max(p2.y) && q1.y.max(q2.y) >= p1.y.min(p2.y) {
        return true;
    }

    return false
}

// &[(f64, f64, f64, f64)] to Linesegments
fn convert_points_to_segments(points: &[(f64, f64, f64, f64)]) -> Vec<LineSegment> {
    let mut segments = Vec::new();
    for (x1, y1, x2, y2) in points {
        segments.push(LineSegment {
            start: Point { x: *x1, y: *y1 },
            end: Point { x: *x2, y: *y2 },
        });
    }
    segments
}


#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

struct LineSegment {
    start: Point,
    end: Point
}

// Berechnung der Schnittpunkte
fn get_intersections(segments: &Vec<LineSegment>) -> u32 {
    let mut intersections = 0;

    for i in 0..segments.len() {
        for j in i+1..segments.len() {
            let intersection = segments[i].intersection(&segments[j]);
            if intersection {
                intersections += 1;
            }
        }
    }

    intersections
}

fn ccw(p: &Point, q: &Point, r: &Point) -> f64 {
    (p.x * q.y - p.y * q.x) + (q.x * r.y - q.y * r.x) + (p.y * r.x - p.x * r.y)
}

impl LineSegment {

    /// Quellen: 
    /// - https://docs.rs/intersect2d/
    /// - https://stackoverflow.com/questions/563198/how-do-you-detect-where-two-line-segments-intersect/565282#565282
    pub fn intersection(&self, other: &LineSegment) -> bool {
        let p1 = &self.start;
        let p2 = &self.end;
        let q1 = &other.start;
        let q2 = &other.end;

        // Orientierung von q1 zu Linie p1p2
        let q1_to_p1p2 = ccw(p1, p2, q1);

        // Orientierung von q2 zu Linie p1p2
        let q2_to_p1p2 = ccw(p1, p2, q2);

        // Wenn beide Orientierungen das gleiche Vorzeichen haben,
        // dann liegen q1 und q2 auf der gleichen Seite der Linie p1p2
        // => Die Linien können sich nicht schneiden
        if q1_to_p1p2 * q2_to_p1p2 > 0.0 {
            return false;
        }

        // Orientierung von p1 zu Linie q1q2
        let p1_to_q1q2 = ccw(q1, q2, p1);

        // Orientierung von p2 zu Linie q1q2
        let p2_to_q1q2 = ccw(q1, q2, p2);

        // Gleiches Prinzip wie oben
        if p1_to_q1q2 * p2_to_q1q2 > 0.0 {
            return false;
        }

        // Wenn alle Punkte kollinear sind -> Schnitt?
        if q1_to_p1p2 == 0.0 && q2_to_p1p2 == 0.0 && p1_to_q1q2 == 0.0 && p2_to_q1q2 == 0.0 {
            return colinear_intersection(p1, p2, q1, q2);

        }

        return true;
    }

}

// Liest die Punkte aus einer Datei ein und gibt sie als intersect2d::Line Vector zurück
fn extract_points_2d(file_path: &str) -> Vec<Line> {
    let mut points: Vec<Line> = Vec::new();

    if let Ok(file) = File::open(file_path) {
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                let mut values = line.split_whitespace().filter_map(|s| s.parse::<f64>().ok());

                if let (Some(x1), Some(y1), Some(x2), Some(y2)) = (values.next(), values.next(), values.next(), values.next()) {
                    points.push(Line::new([x1, y1], [x2, y2]));
                } else {
                    println!("Ungültige Zeile übersprungen: {}", line);
                }
            }
        }
    } else {
        panic!("Datei konnte nicht geöffnet werden: {}", file_path);
    }

    points
}


/// Liest die Punkte aus einer Datei ein und gibt sie als Vektor zurück.
/// Gibt außerdem eine Info-Nachricht zurück, die ungültige Zeilen enthält.
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





#[cfg(test)]
mod tests{

    use super::*;
    #[test]
    fn test_gleicher_endpunkt() {

        assert_eq!(LineSegment {
            start: Point { x: 0.0, y: 0.0 },
            end: Point { x: 2.0, y: 2.0 }
        }.intersection(&LineSegment {
            start: Point { x: 1.0, y: 3.0 },
            end: Point { x: 2.0, y: 2.0 }
        }), true);
    }
    
#[test]
fn test_gleicher_endpunkt2() {
    assert_eq!(LineSegment {
        start: Point { x: 0.0, y: 0.0 },
        end: Point { x: 2.0, y: 2.0 }
    }.intersection(&LineSegment {
        start: Point { x: 3.0, y: 1.0 },
        end: Point { x: 2.0, y: 2.0 }
    }), true);
}

#[test]
fn test_schnitt_im_endpunkt() {
    assert_eq!(LineSegment {
        start: Point { x: 0.0, y: 0.0 },
        end: Point { x: 2.0, y: 2.0 }
    }.intersection(&LineSegment {
        start: Point { x: 1.0, y: 3.0 },
        end: Point { x: 3.0, y: 1.0 }
    }), true);
}

#[test]
fn test_senkrecht_waagerecht() {
    assert_eq!(LineSegment {
        start: Point { x: 1.0, y: 2.0 },
        end: Point { x: 3.0, y: 2.0 }
    }.intersection(&LineSegment {
        start: Point { x: 2.0, y: 3.0 },
        end: Point { x: 2.0, y: 1.0 }
    }), true);
}

#[test]
fn test_kollinear_schnitt() {
    assert_eq!(LineSegment {
        start: Point { x: 1.0, y: 1.0 },
        end: Point { x: 2.0, y: 2.0 }
    }.intersection(&LineSegment {
        start: Point { x: 3.0, y: 3.0 },
        end: Point { x: 2.0, y: 2.0 }
    }), true);
}

#[test]
fn test_strecke_in_strecke() {
    assert_eq!(LineSegment {
        start: Point { x: 1.0, y: 1.0 },
        end: Point { x: 3.0, y: 3.0 }
    }.intersection(&LineSegment {
        start: Point { x: 4.0, y: 4.0 },
        end: Point { x: 2.0, y: 2.0 }
    }), true);
}

#[test]
fn test_kollinear_kein_schnitt() {
    assert_eq!(LineSegment {
        start: Point { x: 1.0, y: 1.0 },
        end: Point { x: 2.0, y: 2.0 }
    }.intersection(&LineSegment {
        start: Point { x: 3.0, y: 3.0 },
        end: Point { x: 4.0, y: 4.0 }
    }), false);
}

#[test]
fn test_punkt_strecke() {
    assert_eq!(LineSegment {
        start: Point { x: 1.0, y: 1.0 },
        end: Point { x: 3.0, y: 3.0 }
    }.intersection(&LineSegment {
        start: Point { x: 2.0, y: 2.0 },
        end: Point { x: 2.0, y: 2.0 }
    }), true);
}

#[test]
fn test_punkt_in_punkt() {
    assert_eq!(LineSegment {
        start: Point { x: 1.0, y: 1.0 },
        end: Point { x: 1.0, y: 1.0 }
    }.intersection(&LineSegment {
        start: Point { x: 1.0, y: 1.0 },
        end: Point { x: 1.0, y: 1.0 }
    }), true);
}

}
