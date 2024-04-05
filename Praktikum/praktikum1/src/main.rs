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

use std::time::{Instant, Duration};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Write;
use std::path::Path;


fn main() {

    struct Point {
        x: f64,
        y: f64,
    }

    let start_time = Instant::now();
    //let file_path = "../../../strecken/s_1000_1.dat";
    let file_path = "strecken/s_1000_1.dat";

    // init vector for saving points (x1, y1, x2, y2)
    let mut points: Vec<(f64, f64, f64, f64)> = Vec::new();

    // open file, read lines
    if let Ok(file) = File::open(file_path) {
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                // convert line to float values
                let mut values = line.split_whitespace().filter_map(|s| s.parse::<f64>().ok());

                // skip, if not 4 values
                if let (Some(x1), Some(y1), Some(x2), Some(y2)) = (values.next(), values.next(), values.next(), values.next()) {
                    points.push((x1, y1, x2, y2));
                } else {
                    println!("Ungültige Zeile übersprungen: {}", line);
                }
            }
        }
    } else {
        println!("Datei konnte nicht geöffnet werden: {}", file_path);
        return;
    }

    let read_time = Instant::now();
    let read_duration = read_time - start_time;
    let mut info_message= String::new();
    info_message.push_str("Datei eingelesen (Dauer: ");
    info_message.push_str(&read_duration.as_millis().to_string());
    info_message.push_str(" ms).\n");
    //println!("{}", info_message);

    let n = points.len();
    let mut intersections = 0;

    // line1 = p1 + t * d12     -> d12 = p2-p1
    // line2 = p3 + s * d34     -> d34 = p4-p3
    for i in 0..n - 2 {

        let (mut x1, mut y1, mut x2, mut y2) = points[i];

        let mut p1 = Point { x: x1, y: y1 };
        let mut p2 = Point { x: x2, y: y2 };
        let mut d12 = Point { x: p2.x - p1.x, y: p2.y - p1.y };

        for j in i + 1..n - 1 {
            
            let (mut x3, mut y3, mut x4, mut y4) = points[j];

            let mut p3 = Point { x: x3, y: y3 };
            let mut p4 = Point { x: x4, y: y4 };
            let mut d34 = Point { x: p4.x - p3.x, y: p4.y - p3.y };

            let mut det = d12.x * d34.y - d34.x * d12.y;
            let mut cross_prod = (p3.x - p1.x) * d34.y - (p3.y - p1.y) * d34.x;

            // lines are not parallel
            if det != 0.0 {

                let mut t = cross_prod / det;
                let mut intersection_s = Point { x: p1.x + t * d12.x, y: p1.y + t * d12.y};

                let mut s = if d34.x != 0.0 {
                    (intersection_s.x - p3.x) / d34.x
                } else {
                    (intersection_s.y - p3.y) / d34.y
                };

                // scalar of direction vector for both lines between 0 and 1 -> crosspoint = true
                if 0.0 <= t && t <= 1.0 && 0.0 <= s && s <= 1.0 {
                    intersections = intersections + 1;
                }
            }

            // lines on same straight
            else if cross_prod == 0.0 {

                // TODO:
                let partially_coincident = (p1.x >= p2.x && p1.x <= p4.x && p1.y >= p2.y && p1.y <= p4.y) || (p3.x >= p2.x && p3.x <= p4.x && p3.y >= p2.y && p3.y <= p4.y) || (p2.x >= p1.x && p2.x <= p3.x && p2.y >= p1.y && p2.y <= p3.y) || (p4.x >= p1.x && p4.x <= p3.x && p4.y >= p1.y && p4.y <= p3.y);

                if partially_coincident {
                    intersections = intersections + 1;
                }
            }

            // else: lines parallel but not on same straight
        }
    }

    // calc Duration and create message
    let calc_time = Instant::now();
    let calc_duration = calc_time - read_time;
    info_message.push_str(&intersections.to_string());
    info_message.push_str(" Schnittpunkte gefunden (Dauer: ");
    info_message.push_str(&calc_duration.as_millis().to_string());
    info_message.push_str(" ms).");
    println!("{}", info_message);

    // create .txt file in same folder with message
    let mut file_dir: Vec<&str> = file_path.split('.').collect();
    file_dir.pop();
    file_dir.push(".txt");
    let mut outcome_dir = file_dir.concat();
    println!("{}", outcome_dir);

    let mut outcome_file = File::create(outcome_dir).expect("Fehler bei erstellen der Datei!");
    outcome_file.write_all(info_message.as_bytes()).expect("Fehler bei schreiben der Datei!");
}