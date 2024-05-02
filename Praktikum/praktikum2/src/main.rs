use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::thread::current;
use plotters::prelude::*;
use xml::reader::XmlEvent;
use xml::EventReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    blub();

    let states = vec![
        "Thueringen",
        "Bayern",
        ];

        /*
                "Sachsen",
        "Sachsen-Anhalt",
        "Niedersachsen",
        "Mecklenburg-Vorpommern",
        "Hessen",
        "Hamburg",
        "Bremen",
        "Brandenburg",
        "Berlin",
        "Bayern",
        "Baden-Wuerttemberg",
        "Nordrhein-Westfalen",
        "Rheinland-Pfalz",
        "Saarland",
        "Schleswig-Holstein"
         */

    for state in states {

        let filename = format!("states/{}", state);
        let points = read_points(format!("{}{}", &filename, ".txt"))?;

        draw_polygon(format!("{}{}", &filename, ".png"), &points)?;


        // Starte bei Punkt 0 und Ursprung
        let ursprung = (0.0, 0.0);

        // Gesamtfläche
        let mut a_ges = 0.0;

        // Wähle Punkte n und n+1
        for i in 0..points.len() - 1 {

            // Punkte ausgabe von points[i], points[i + 1]
            //println!("Punkt {}: ({},{}), Punkt {}: ({},{})", i, points[i].0, points[i].1, i + 1, points[i + 1].0, points[i + 1].1);

            // Berechne ccw für Punkt n und n+1
            let sign = ccw(ursprung, points[i], points[i + 1]);

            // Flächeninhalt berechnen
            let area = (points[i].0 * points[i + 1].1) - (points[i + 1].0 * points[i].1) / 2.0;

            // Summiere Flächen auf
            a_ges = a_ges + sign * area;
        }

        println!("Fläche von {}: {}", state, a_ges.abs());
    }


    // Berechne ccw für Punkt n und n+1
    // Berechne Fläche: A = (x_n * y_n+1) - (x_n+1 * y_n)/2
    // Summiere Flächen auf: Ages += ccw * A



    Ok(())
}


fn read_points<P>(filename: P) -> io::Result<Vec<(f32, f32)>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut points: Vec<(f32, f32)> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let coords: Vec<f32> = line.split(',')
                                   .map(|s| s.parse().unwrap())
                                   .collect();
        if coords.len() == 2 {
            points.push((coords[0], coords[1]));
        }
    }

    // Konvertierung der relativen Punkte in absolute Koordinaten
    let absolute_points: Vec<(f32, f32)> = relative_to_absolute(&points);

    Ok(absolute_points)
}

// DEPRECATED: Versucht ein Polygon zu zeichnen
fn draw_polygon(name: String, points: &[(f32, f32)]) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(&name, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..1000f32, 0f32..1000f32)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        points.iter().cloned().cycle().take(points.len() + 1),
        &RED,
    ))?;

    root.present()?;
    Ok(())
}

// Transformiert die relativen Punkte zu absoluten Punkten
fn relative_to_absolute(relative_points: &Vec<(f32, f32)>) -> Vec<(f32, f32)> {
    // Ursprungskoordinaten
    let origin_x = relative_points[0].0;
    let origin_y = relative_points[0].1;
    

    let mut absolute_points = Vec::new();
    let mut current_x = origin_x;
    let mut current_y = origin_y;

    let firstPoint = relative_points[0];
    let mut f2Point: (f32, f32) = (0.0,0.0);
    let mut f2Point2: (f32, f32) = (0.0,0.0);

    // Bayern
    if firstPoint == (393.093,474.992) {
        f2Point = (275.0,712.497);
        f2Point2 = (275.0,574.206);
    }
    // Thüringen
    if firstPoint == (312.004,351.725) {
        f2Point = (265.601,388.0);
        f2Point2 = (100000.0,1000000.0);
    }




    for &(x, y) in relative_points {
        if firstPoint == (x, y) {
            current_x = firstPoint.0;
            current_y = firstPoint.1;
            absolute_points.push((current_x, current_y));
            continue;
        }

        if f2Point == (x, y) {
            current_x = f2Point.0;
            current_y = f2Point.1;
            absolute_points.push((current_x, current_y));
            continue;
        }

        if f2Point2 == (x, y) {
            current_x = f2Point2.0;
            current_y = f2Point2.1;
            absolute_points.push((current_x, current_y));
            continue;
        }

        let absolute_x = current_x + x;
        let absolute_y = current_y + y;
        absolute_points.push((absolute_x, absolute_y));
        current_x = absolute_x;
        current_y = absolute_y;
    }

    absolute_points
}

fn ccw(p1: (f32, f32), p2: (f32, f32), p3: (f32, f32)) -> f32 {
    // Berechne die Vektoren von Punkt1 nach Punkt2 und von Punkt2 nach Punkt3
    let vector1 = (p2.0 - p1.0, p2.1 - p1.1);
    let vector2 = (p3.0 - p2.0, p3.1 - p2.1);
    
    // Berechne das Kreuzprodukt
    let cross_product = vector1.0 * vector2.1 - vector1.1 * vector2.0;
    
    // Überprüfe das Vorzeichen des Kreuzprodukts
    if cross_product > 0.0 {
        1.0  // Gegen den Uhrzeigersinn
    } else if cross_product < 0.0 {
        -1.0  // Im Uhrzeigersinn
    } else {
        0.0  // Kollinear (liegen auf einer Linie)
    }
}

fn blub() {
   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ccw() {
        let p1 = (0.0, 0.0);
        let p2 = (1.0, 1.0);
        let p3 = (2.0, 0.0);

        assert_eq!(ccw(p1, p2, p3), -1.0);
    }
}

