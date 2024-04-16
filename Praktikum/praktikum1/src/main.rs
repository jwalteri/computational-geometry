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


fn main() {

    let start_time = Instant::now();
    //let file_path = "strecken/s_1000_1.dat";
    //let file_path = "strecken/s_10000_1.dat";
    let file_path = "strecken/s_100000_1.dat";

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
    let intersections = calculate_intersections(&points);

    // Zeitmessung: Berechnung der Schnittpunkte
    let calc_time = Instant::now();
    let calc_duration = calc_time - read_time;

    // Ausgabe der Anzahl der Schnittpunkte
    info_message.push_str(&intersections.to_string());
    info_message.push_str(" Schnittpunkte gefunden (Dauer: ");
    info_message.push_str(&calc_duration.as_millis().to_string());
    info_message.push_str(" ms).");
    println!("{}", info_message);

    // Output File abhängig von verwendetem Input anlegen
    let mut file_dir: Vec<&str> = file_path.split('.').collect();
    file_dir.pop();
    file_dir.push(".txt");
    let outcome_dir = file_dir.concat();
    println!("{}", outcome_dir);

    let mut outcome_file = File::create(outcome_dir).expect("Fehler bei erstellen der Datei!");
    outcome_file.write_all(info_message.as_bytes()).expect("Fehler bei schreiben der Datei!");
}

struct Point {
    x: f64,
    y: f64,
}

fn multiplyPoints(p1: Point, p2: Point) -> Point {
    Point { x: p1.x * p2.x, y: p1.y * p2.y }
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

        if p1.x == p2.x && p1.y == p2.y {
            println!("Punkt {} ist ein Punkt und keine Linie", i);
        }


        // Richtungsvektor des aktuellen Linienabschnitts
        let d12 = Point { x: p2.x - p1.x, y: p2.y - p1.y };

        // Iteriere über die verbleibenden Punkte
        for j in i + 1..n { //- 1 {
            
            // Linienabschnitt zum Vergleichen bestimmen
            let (x3, y3, x4, y4) = points[j];
            let q1 = Point { x: x3, y: y3 };
            let q2 = Point { x: x4, y: y4 };

            if q1.x == q2.x && q1.y == q2.y {
                println!("Punkt {} ist ein Punkt und keine Linie", j);
            }

            
            // Richtungsvektor des Linienabschnitt zum Vergleichen bestimmen
            let d34 = Point { x: q2.x - q1.x, y: q2.y - q1.y };

             // Determinante bestimmen für Überprüfung, ob Linienabschnitte parallel sind
            let det = d12.x * d34.y - d34.x * d12.y;

            // Kreuzprodukt für die Überprüfung auf Kollinearität
            let cross_prod = (q1.x - p1.x) * d34.y - (q1.y - p1.y) * d34.x;

            // Wenn Linien nicht parallel sind
            if det != 0.0 {

                // Parameter für den Schnittpunkt des ersten Linienabschnitts
                let t = cross_prod / det;
                // Berechnung des Schnittpunkts
                let intersection_s = Point { x: p1.x + t * d12.x, y: p1.y + t * d12.y};

                // Berechnung des Parameters für den Schnittpunkt des zweiten Linienabschnitts
                let s = if d34.x != 0.0 {
                    (intersection_s.x - q1.x) / d34.x
                } else {
                    (intersection_s.y - q1.y) / d34.y
                };

                let epsilon = 0.0;

                // Wenn die Parameter zwischen 0 und 1 liegen, gibt es einen Schnittpunkt
                if epsilon <= t && t <= 1.0 && 
                epsilon <= s && s <= 1.0 {
                    // Inkrementiere die Anzahl der Schnittpunkte
                    intersections = intersections + 1;
                    //println!("I {}, J {}", i, j);
                }
            }

            // Wenn Linien kollinear sind
            else if cross_prod == 0.0 {

                // Teilweise überlappende Linienabschnitte?
                let partially_coincident = //is_inside(&p1, &p2, q2, q1);
                segments_overlap(&p1, &p2, &q1, &q2);

                if partially_coincident {
                    intersections = intersections + 1;
                    //println!("I {}, J {}", i, j);
                }
            }
        }
    }

    intersections
}

fn is_inside(p1: &Point, p2: &Point, q2: Point, q1: Point) -> bool {
    let partially_coincident = 
    (p1.x >= p2.x && p1.x <= q2.x && p1.y >= p2.y && p1.y <= q2.y) || 
    (q1.x >= p2.x && q1.x <= q2.x && q1.y >= p2.y && q1.y <= q2.y) || 
    (p2.x >= p1.x && p2.x <= q1.x && p2.y >= p1.y && p2.y <= q1.y) || 
    (q2.x >= p1.x && q2.x <= q1.x && q2.y >= p1.y && q2.y <= q1.y);
    partially_coincident
}


fn is_inside2(punkt: &Point, start: &Point, end: &Point) -> bool {
    (start.x <= punkt.x && punkt.x <= end.x || start.x >= punkt.x && punkt.x >= end.x) &&
    (start.y <= punkt.y && punkt.y <= end.y || start.y >= punkt.y && punkt.y >= end.y)
}

fn segments_overlap(start1: &Point, end1: &Point, start2: &Point, end2: &Point) -> bool {
let p1_inside_segment2 = is_inside2(start2, start1, end1) || is_inside2(end2, start1, end1);
let p2_inside_segment1 = is_inside2(start1, start2, end2) || is_inside2(end1, start2, end2);
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




#[test]
fn test_is_inside() {
    let p1 = Point { x: 1.0, y: 1.0 };
    let p2 = Point { x: 2.0, y: 2.0 };
    let q1 = Point { x: 0.0, y: 0.0 };
    let q2 = Point { x: 3.0, y: 3.0 };

    assert_eq!(segments_overlap(&p1, &p2, &q2, &q1), true);
    // Add more test cases here
}

#[test]
fn test_is_inside_negativ() {
    let p1 = Point { x: 1.0, y: 1.0 };
    let p2 = Point { x: 2.0, y: 2.0 };
    let q1 = Point { x: 0.0, y: 0.0 };
    let q2 = Point { x: 0.99, y: 0.99 };

    assert_eq!(segments_overlap(&p1, &p2, &q2, &q1), false);
    // Add more test cases here
}

#[test]
fn test_calculate_intersections() {
    let points = &[
        (0.0, 0.0, 2.0, 2.0),  // Orange
        (1.0, 1.0, 3.0, 3.0),  // Grüne (kollinear zu Linie 1)
        (1.0, 2.0, 2.0, 1.0),  // Rot (schneidet Linie 1 und Linie 2)
        (4.0, 4.0, 5.0, 5.0),  // Lila (parallel zu Linie 1 und Linie 2)
        (0.0, 0.0, -2.0, -2.0), // Schwarz
        (2.0, 4.0, 4.0, 2.0),  // Grau
        (-0.5, -0.5, -1.5, -1.5), // Pink
        (-2.0, -2.0, -1.0, -3.0) // Gift
    ];

    assert_eq!(calculate_intersections(points), 7);
}

#[test]
fn test_simple_schnitt() {
    let points = &[
        (0.0, 0.0, 2.0, 2.0), 
        (0.0, 2.0, 2.0, 0.0)];

    assert_eq!(calculate_intersections(points), 1);
}

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
fn test_punkt_in_punktXY() {
    let points = &[
        (1.0, 1.0, 1.0, 1.0), 
        (1.0, 1.0, 1.0, 1.0)];

    assert_eq!(calculate_intersections(points), 1);
}