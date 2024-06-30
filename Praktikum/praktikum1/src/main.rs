// Computational Geometry Praktikum
// Aufgabe 1
// SS24 @ University of Applied Sciences Munich
// Copyright: J. Walter, L. Biege


/* DEFINITION Schnittpunkte:
T = true (-> det != 0 && 0 <= t&s <= 1)
II = false (-> det == 0 && cross_prod != 0)
Deckungsgleich = true (-> det == 0 && cross_prod == 0)
- - = false (wie Deckungsgleich UND A|B nicht in CD && C|D nicht in AB)
*/

use core::panic;
use std::f32::EPSILON;
use std::time::{Instant, Duration};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Write;
use std::path::Path;


/*
Datei eingelesen (Dauer: 96507 ms).
15 Schnittpunkte gefunden (Dauer: 18 ms).

Datei eingelesen (Dauer: 31 ms).
737 Schnittpunkte gefunden (Dauer: 1643 ms).

Datei eingelesen (Dauer: 206 ms).
77139 Schnittpunkte gefunden (Dauer: 162805 ms).
*/

const PRECISION: f64 = 0.0001;

fn main() {

    let start_time = Instant::now();
    let file_path = "strecken/s_1000_1.dat";
    // let file_path = "strecken/s_10000_1.dat";
    //let file_path = "strecken/s_100000_1.dat";

    let segments = read_segments_from_file(file_path);

    let mut intersections = 0;

    for i in 0..segments.len() - 1 {
        for j in i + 1..segments.len() {
            if let Some(intersection) = segments[i].intersect(&segments[j]) {
                intersections += 1;
                //println!("Schnittpunkt: {} {}", intersection.x, intersection.y);
            }
        }
    }

    // Ausgabe Schnittpunkte
    println!("{} Schnittpunkte gefunden", intersections);



    // Logging mithilfe von info_message
    let mut info_message = String::new();
    // Einlesen der Datei
    let (points, updated_info_message) = extract_points(file_path);
    info_message.push_str(&updated_info_message);

    // Zeitmessung: Einlesen der Datei
    let read_time = Instant::now();
    let read_duration = read_time - start_time;
    info_message.push_str("Datei eingelesen (Dauer: ");
    info_message.push_str(&read_duration.as_millis().to_string());
    info_message.push_str(" ms).\n");

    // Berechnung der Schnittpunkte
    let intersections = get_intersections(&convert_points_to_segments(&points)); //calculate_intersections(&points);

    // Zeitmessung: Berechnung der Schnittpunkte
    let calc_time = Instant::now();
    let calc_duration = calc_time - read_time;

    // Ausgabe der Anzahl der Schnittpunkte
    println!("{} Schnittpunkte gefunden (Dauer: {} ms).", intersections.len(), calc_duration.as_millis());

    // info_message.push_str(&intersections.to_string());
    // info_message.push_str(" Schnittpunkte gefunden (Dauer: ");
    // info_message.push_str(&calc_duration.as_millis().to_string());
    // info_message.push_str(" ms).");
    // println!("{}", info_message);

    // Output File abhängig von verwendetem Input anlegen
    let mut file_dir: Vec<&str> = file_path.split('.').collect();
    file_dir.pop();
    file_dir.push(".txt");
    let outcome_dir = file_dir.concat();
    println!("{}", outcome_dir);

    let mut outcome_file = File::create(outcome_dir).expect("Fehler bei erstellen der Datei!");
    outcome_file.write_all(info_message.as_bytes()).expect("Fehler bei schreiben der Datei!");
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
        segments.push(LineSegment {
                    start: Point { x: x1, y: y1 },
                    end: Point { x: x2, y: y2 },
                });
    }
    segments
}

struct Point {
    x: f64,
    y: f64,
}

struct LineSegment {
    start: Point,
    end: Point
}

// Berechnung der Schnittpunkte
fn get_intersections(segments: &Vec<LineSegment>) -> Vec<Point> {
    let mut intersections = Vec::new();
    for i in 0..segments.len() - 1 {
        for j in i + 1..segments.len() {
            if let Some(intersection) = segments[i].intersection(&segments[j]) {
                intersections.push(intersection);
            }
        }
    }
    intersections
}

// ccw
fn ccw(a: &Point, b: &Point, c: &Point) -> f64 {
    (c.y - a.y) * (b.x - a.x) - (b.y - a.y) * (c.x - a.x)
}

impl LineSegment {


    fn intersection(&self, other: &LineSegment) -> Option<Point> {
        let p1 = &self.start;
        let p2 = &self.end;
        let q1 = &other.start;
        let q2 = &other.end;
        let ccwq1 = ccw(p1, p2, q1);
        let ccwq2 = ccw(p1, p2, q2);
        if ccwq1 * ccwq2 > 0.0 {
        return None;
        }
        let ccwp1 = ccw(q1, q2, p1);
        let ccwp2 = ccw(q1, q2, p2);
        if ccwp1 * ccwp2 > 0.0 {
        return None;
        }
        if ccwq1 == 0.0 && ccwq2 == 0.0 && ccwp1 == 0.0 && ccwp2 == 0.0 {
        println!("Two colinear lines were detected!");
        }
        // Determine intersection point
        let r_ab = (ccwq2 / ccwq1).abs();
        let a = r_ab / (r_ab + 1.0);
        let i_x = q2.x + a * (q1.x - q2.x);
    
        let i_y = q2.y + a * (q1.y - q2.y);
        Some(Point { x: i_x, y: i_y })
    }

    fn intersect(&self, other: &LineSegment) -> Option<Point> {
        let LineSegment { start: p1, end: p2} = self;
        let LineSegment { start: p3, end: p4} = other;

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

    fn round_to_4_decimals(num: f64) -> f64 {
        (num * 10000.0).round() / 10000.0
    }
}
pub fn calculate_intersections(points: &[(f64, f64, f64, f64)]) -> usize {

    let n = points.len();
    // Initialisiere Anzahl der Schnittpunkte mit 0
    let mut intersections = 0;
    // line1 = p1 + t * d12     -> d12 = p2-p1
    // line2 = p3 + s * d34     -> d34 = p4-p3

    // Berechne Schnittpunkte zwischen Linienabschnitten
    for i in 0..n - 1 { // Iteriere über alle Punkte außer den letzten beiden

        // Aktuellen Linienabschnitt bestimmen
        let (x1, y1, x2, y2) = points[i];
        let p1 = Point { x: x1, y: y1 };
        let p2 = Point { x: x2, y: y2 };

        // Richtungsvektor des aktuellen Linienabschnitts
        let d12 = Point { x: p2.x - p1.x, y: p2.y - p1.y };

        // Iteriere über die verbleibenden Punkte
        for j in i + 1..n { //- 1 {
            
            // Linienabschnitt zum Vergleichen bestimmen
            let (x3, y3, x4, y4) = points[j];
            let q1 = Point { x: x3, y: y3 };
            let q2 = Point { x: x4, y: y4 };
            
            // Richtungsvektor des Linienabschnitt zum Vergleichen bestimmen
            let d34 = Point { x: q2.x - q1.x, y: q2.y - q1.y };

             // Determinante bestimmen für Überprüfung, ob Linienabschnitte parallel sind
            let det = d12.x * d34.y - d34.x * d12.y;

            // Kreuzprodukt für die Überprüfung auf Kollinearität
            let cross_prod = (q1.x - p1.x) * d34.y - (q1.y - p1.y) * d34.x;

            // Wenn Linien nicht parallel sind
            //if det != 0.0 {
            if det.abs() > PRECISION {

                // Parameter für den Schnittpunkt des ersten Linienabschnitts
                let t = cross_prod / det;
                // Berechnung des Schnittpunkts
                let intersection_s = Point { x: p1.x + t * d12.x, y: p1.y + t * d12.y};

                // Berechnung des Parameters für den Schnittpunkt des zweiten Linienabschnitts
                let s = if d34.x != 0.0 { // 0 weil DivisionByZero
                    (intersection_s.x - q1.x) / d34.x
                } else {
                    (intersection_s.y - q1.y) / d34.y
                };

                // Wenn die Parameter zwischen 0 und 1 liegen, gibt es einen Schnittpunkt
                // TODO: Prüfen, ob das 0 - PRECISION sinn macht
                if - PRECISION <= t && t <= 1.0 + PRECISION && 
                - PRECISION <= s && s <= 1.0 + PRECISION {
                    // Inkrementiere die Anzahl der Schnittpunkte
                    intersections = intersections + 1;
                    //println!("Schnittpunkt: I {}, J {}", i, j);
                    println!("Schnittpunkt: {} {}", intersection_s.x, intersection_s.y);
                }
            }

            // Wenn Linien kollinear sind
            // else if cross_prod == 0.0 {
            else if cross_prod.abs() < PRECISION {

                // Teilweise überlappende Linienabschnitte?
                let partially_coincident = segments_overlap(&p1, &p2, &q1, &q2);

                if partially_coincident {
                    intersections = intersections + 1;
                    //println!("I {}, J {}", i, j);
                }
            }
        }
    }

    intersections
}

// Überprüft, ob ein Punkt auf der Strecke liegt
fn is_inside(punkt: &Point, start: &Point, end: &Point) -> bool {
    (start.x <= punkt.x && punkt.x <= end.x || start.x >= punkt.x && punkt.x >= end.x) &&
    (start.y <= punkt.y && punkt.y <= end.y || start.y >= punkt.y && punkt.y >= end.y)
}

// Prüft, ob sich eines der Segmente überlappen
fn segments_overlap(start1: &Point, end1: &Point, start2: &Point, end2: &Point) -> bool {
let p1_inside_segment2 = is_inside(start2, start1, end1) || is_inside(end2, start1, end1);
let p2_inside_segment1 = is_inside(start1, start2, end2) || is_inside(end1, start2, end2);
p1_inside_segment2 || p2_inside_segment1
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
        let points = &[
            (0.0, 0.0, 2.0, 2.0), 
            (1.0, 3.0, 2.0, 2.0)];

        assert_eq!(calculate_intersections(points), 1);
    }
    
#[test]
fn test_gleicher_endpunkt2() {
    let points = &[
        (0.0, 0.0, 2.0, 2.0), 
        (3.0, 1.0, 2.0, 2.0)];

    assert_eq!(calculate_intersections(points), 1);
}

#[test]
fn test_schnitt_im_endpunkt() {
    let points = &[
        (0.0, 0.0, 2.0, 2.0), 
        (1.0, 3.0, 3.0, 1.0)];

    assert_eq!(calculate_intersections(points), 1);
}

#[test]
fn test_senkrecht_waagerecht() {
    let points = &[
        (1.0, 2.0, 3.0, 2.0), 
        (2.0, 3.0, 2.0, 1.0)];

    assert_eq!(calculate_intersections(points), 1);
}

#[test]
fn test_kollinear_schnitt() {
    let points = &[
        (1.0, 1.0, 2.0, 2.0), 
        (3.0, 3.0, 2.0, 2.0)];

    assert_eq!(calculate_intersections(points), 1);
}

#[test]
fn test_strecke_in_strecke() {
    let points = &[
        (1.0, 1.0, 3.0, 3.0), 
        (4.0, 4.0, 2.0, 2.0)];

    assert_eq!(calculate_intersections(points), 1);
}

#[test]
fn test_kollinear_kein_schnitt() {
    let points = &[
        (1.0, 1.0, 2.0, 2.0), 
        (3.0, 3.0, 4.0, 4.0)];

    assert_eq!(calculate_intersections(points), 0);
}

#[test]
fn test_punkt_strecke() {
    let points = &[
        (1.0, 1.0, 3.0, 3.0), 
        (2.0, 2.0, 2.0, 2.0)];

    assert_eq!(calculate_intersections(points), 1);
}

#[test]
fn test_punkt_in_punkt() {
    let points = &[
        (1.0, 1.0, 1.0, 1.0), 
        (1.0, 1.0, 1.0, 1.0)];

    assert_eq!(calculate_intersections(points), 1);
}

#[test]
fn test_1000_file() {
    let file_path = "strecken/s_1000_1.dat";
    let (points, _) = extract_points(file_path);
    assert_eq!(calculate_intersections(&points), 11);
}
}
