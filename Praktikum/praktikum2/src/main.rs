use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::vec;
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Speichere die Koordinaten der Bundesländer aus Deutschland in jeweils eigene .txt-Dateien
    //save_coordinates_from_each_state();

    // Lese die Dateien im Verzeichnis "states" ein
    let states_path = "states";
    let dir_entries = std::fs::read_dir(states_path)?;

    // Erstelle einen Vektor für die Dateinamen (Bundesländer) der .txt-Dateien
    let file_ending = ".txt".to_string();
    let mut states: Vec<String> = Vec::new();
    get_files_with_ending(&mut states, dir_entries, &file_ending)?;
    
    let mut germany_plot: vec::Vec<Vec<(f32, f32)>> = Vec::new();

    for state in states {

        let filename = format!("states/{}", state);
        let mut state_points: Vec<Vec<(f32, f32)>> = relative_file_to_absolute_vector(format!("{}{}", &filename, ".txt"));

        // Zeichne jedes Bundesland einzeln in "state.png"
        let mut tmp: vec::Vec<Vec<(f32, f32)>> = Vec::new();
        for p in &mut state_points {
            tmp.push(p.clone())
        }
        draw_polygon(format!("{}{}", &filename, ".png"), tmp)?;


        // Starte bei Punkt 0 und Ursprung
        let ursprung = (0.0, 0.0);

        // Gesamtfläche
        let mut d_ges = 0.0;
        let mut t_ges = 0.0;

        for connected_points in &mut state_points {
            // Wähle Punkte n und n+1 aus letztem points-Vector
            for i in 0..connected_points.len() - 1 {

                // Punkte ausgabe von points[i], points[i + 1]
                //println!("Punkt {}: ({},{}), Punkt {}: ({},{})", i, points[i].0, points[i].1, i + 1, points[i + 1].0, points[i + 1].1);

                // Berechne ccw für Punkt n und n+1
                let sign = ccw(ursprung, connected_points[i], connected_points[i + 1]);

            // Flächeninhalt berechnen
            let dArea = (points[i].0 * points[i + 1].1) - (points[i + 1].0 * points[i].1) / 2.0;

            // Trapezfläche berechnen
            let tArea = (points[i].1 + points[i + 1].1) / 2.0 * (points[i].0 - points[i + 1].0);


            // Summiere Flächen auf
            d_ges = d_ges + sign * dArea;
            t_ges = t_ges + sign * tArea;
        }

        // Shoelace-Formel
        let shoelace_area = shoelace_formel(&points);

        // Dreiecks-Shoelace-Formel
        let dreieck_shoelace_formel_area = dreieck_shoelace_formel(&points);

        // Füge Punkte den Plot hinzu
        points_to_plot.push(points);

        println!("Dreiecksfläche von {}: {}", state, d_ges.abs());
        println!("Trapezfläche von {}: {}", state, t_ges.abs());
        println!("Trapezfläche von {}: {}", state, t_ges.abs());
        println!("Shoelace-Formel von {}: {}", state, shoelace_area.abs());
        println!("dreieck_shoelace_formel von {}: {}", state, dreieck_shoelace_formel_area.abs());
    }

    draw_polygon("Deutschland.png".to_owned(), germany_plot)?;


    // Berechne ccw für Punkt n und n+1
    // Berechne Fläche: A = (x_n * y_n+1) - (x_n+1 * y_n)/2
    // Summiere Flächen auf: Ages += ccw * A



    Ok(())
}

// Lese die "ending" Dateien im Verzeichnis "dir_entries" ein und übergebe die Dateinamen als Vektor
fn get_files_with_ending(states: &mut Vec<String>, dir_entries: std::fs::ReadDir, ending: &String) -> Result<(), Box<dyn std::error::Error>> {

    for entry in dir_entries {
        let entry = entry?;
        let file_name = entry.file_name();
        if let Some(file_str) = file_name.to_str() {
            if file_str.ends_with(ending) {
                if let Some(file_stem) = Path::new(file_str).file_stem() {
                    if let Some(file_name_str) = file_stem.to_str() {
                        states.push(file_name_str.to_string());
                    }
                }
            }
fn dreieck_shoelace_formel(points: &[(f32, f32)]) -> f32 {
    let mut area: f32 = 0.0;
    let n = points.len();
    for i in 0..n {
        let j = (i + 1) % n;
        area += (points[i].0 * points[j].1) - (points[j].0 * points[i].1);
    }
    area.abs() / 2.0
}

fn shoelace_formel(points: &[(f32, f32)]) -> f32 {
    let mut area = 0.0;
    let n = points.len();
    for i in 0..n {
        let j = (i + 1) % n;
        area += points[i].0 * points[j].1;
        area -= points[j].0 * points[i].1;
    }
    area.abs() / 2.0
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
    Ok(())
}

// TODO: ~low prio~ Speichert die Koordinaten der Bundesländer aus DeutschlandMitStaedten.svg in jeweils einer "state".txt Datei
fn save_coordinates_from_each_state(){
    
}

// DEPRECATED: Versucht ein Polygon zu zeichnen
fn draw_polygon(name: String, points: Vec<Vec<(f32, f32)>>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(&name, (2000, 2000)).into_drawing_area();
    root.fill(&WHITE)?;

    // Flip the y-axis of input data
    //let points: Vec<(f32, f32)> = points.iter().map(|(x, y)| (*x, -1.0 *y)).collect();

    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .top_x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-10f32..810f32, 810f32..-10f32)?;

    chart.configure_mesh().draw()?;


    for point in points {
        chart.draw_series(LineSeries::new(
            point.iter().cloned().cycle().take(point.len() + 1),
            &RED,
        ))?;
    }

    root.present()?;
    Ok(())
}

// Lese die Datei Zeile für Zeile ein und konvertiere die relativen Koordinaten in absolute Koordinaten bzw. behalte die absoluten Koordinaten
fn relative_file_to_absolute_vector(filename: String) -> Vec<Vec<(f32, f32)>> {

    // Vektor für die absoluten Punkte
    let mut p_connected = Vec::new();
    let mut p_state = Vec::new();

    // Öffne die Datei
    if let Ok(file) = File::open(&filename) {
        // Erstelle einen Pufferleser, um die Datei zeilenweise zu lesen
        let reader = BufReader::new(file);

        // Last point
        let mut last_point = (0.0, 0.0);

        // Durchlaufe jede Zeile in der Datei
        for line in reader.lines() {
            if let Ok(line) = line {
                // Überprüfe das Format der Zeile
                let (x, y) = match line.chars().next() {             
                    Some('l') => { // Relative Koordinaten
                        let parts: Vec<&str> = line[1..].split(',').collect();
                        (parts[0].parse::<f32>().unwrap() + last_point.0, parts[1].parse::<f32>().unwrap() + last_point.1)
                    },
                    Some('L') | Some('M') => { // Absolute Koordinaten
                        let parts: Vec<&str> = line[1..].split(',').collect();
                        (parts[0].parse::<f32>().unwrap(), parts[1].parse::<f32>().unwrap())
                    },
                    Some('z') => { // Neues Polygon
                        p_state.push(p_connected.clone());
                        p_connected.clear();
                        continue;
                    },
                    _ => {
                        println!("Unbekanntes Format in file {}", &filename);
                        continue;
                    }
                };

                p_connected.push((x, y));
                last_point = (x, y);
            }
        }
        if p_connected.len() > 0 { // falls letzte line kein "z" -> push letzte Punkte
            p_state.push(p_connected.clone());
        }
    } else {
        println!("Die Datei konnte nicht geöffnet werden.");
    }

    return p_state;
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

