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

    let mut state_vector: Vec<State> = Vec::new();

    for state in states {
        let filename = format!("states/{}", state);
        let state_points: Vec<Vec<(f32, f32)>> = relative_file_to_absolute_vector(format!("{}{}", &filename, ".txt"));

        let mut state = State {
            name: state.clone(),
            polygon: Polygon {
                points: state_points[0].clone()
            },
            insel: Vec::new(),
            loch: Vec::new()
        };

        //state.set_holes_and_islands(state_points[1..].to_vec());
        state.set_holes_and_islands(state_points);

        state_vector.push(state);
    }

    for state in &state_vector {
        println!("Bundesland: {}", state.name);
        println!("Fläche: {}", state.get_area());
        state.draw()?;
    }

    for city in get_cities() {
        for state in &state_vector {
            if state.is_point_inside((city.x, city.y)) {
                println!("Die Stadt {} liegt in {}", city.name, state.name);
            }
        }
    }

    // Alle Punkte zu Deutschland zusammenfassen
    for state in &state_vector {
        for points in state.get_points() {
            for p in points {
                germany_plot.push(vec![p.clone()]);
            }
        }
    }
    
    draw_polygon("Deutschland.png".to_owned(), germany_plot)?;


    println!("asdasd");


/* 
    for state in states {

        println!("Bundesland: {}", state);

        // Zeichne jedes Bundesland einzeln in "state.png"s
        let mut tmp: vec::Vec<Vec<(f32, f32)>> = Vec::new();
        for p in &state_points {
            tmp.push(p.clone())
        }
        draw_polygon(format!("{}{}", &filename, ".png"), tmp)?;

        // Gesamtfläche
        let mut s_ges = 0.0; // Shoelace-Formel
        //let mut ds_ges = 0.0; // Dreiecks-Shoelace-Formel

            let len = state_points.clone().len();

            // Kiel fehlt
            // Potsdam fehlt
        for (i, e) in state_points.iter().enumerate() {
            let part_size = shoelace_formel(e);

            let mut is_hole = false;
            for (j, el) in state_points.iter().enumerate() {
                if i != j {
                    is_hole = point_inside_polygon(state_points[i][0], e);
                    
                    if is_hole {
                        println!("Hole gefunden in {}!", state);
                        break;
                    }
                }
            }
            
            if is_hole {
                s_ges -= part_size;
            } else {
                s_ges += part_size;
                check_cities(e)
            }

            // Füge die Punkte zum Deutschland-Plot hinzu
            germany_plot.push(e.clone());
        }

        println!("Shoelace-Formel von {}: {}", state, s_ges.abs());
    }

        draw_polygon("Deutschland.png".to_owned(), germany_plot)?;
        */
    Ok(())
}

struct City {
    name: String,
    x: f32,
    y: f32,
}

// Get Cities
fn get_cities() -> Vec<City> {
    let file = File::open("cities/cities.txt").expect("Konnte Datei nicht öffnen");
    let reader = BufReader::new(file);

    let mut cities = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 3 {
                let name = parts[0].to_string();
                if let (Ok(x), Ok(y)) = (parts[1].parse::<f32>(), parts[2].parse::<f32>()) {
                    let city = City { name, x, y };
                    cities.push(city);
                } else {
                    eprintln!("Fehler beim Parsen der Koordinaten für Stadt: {}", name);
                }
            } else {
                eprintln!("Ungültiges Format für Zeile: {}", line);
            }
        }
    }

    cities
}

// TODO: Fang bei den kleinsten Polygonen an und arbeite dich zu den größten vor
// Gefundene Städte aus Vektor entfernen
fn check_cities(polygon: &[(f32, f32)]) {
    let file = File::open("cities/cities.txt").expect("Konnte Datei nicht öffnen");
    let reader = BufReader::new(file);

    let mut cities = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 3 {
                let name = parts[0].to_string();
                if let (Ok(x), Ok(y)) = (parts[1].parse::<f32>(), parts[2].parse::<f32>()) {
                    let city = City { name, x, y };
                    cities.push(city);
                } else {
                    eprintln!("Fehler beim Parsen der Koordinaten für Stadt: {}", name);
                }
            } else {
                eprintln!("Ungültiges Format für Zeile: {}", line);
            }
        }
    }

    for city in cities {
        let point = (city.x, city.y);
        let inside = point_inside_polygon(point, polygon);
        if inside {
            println!("Die Stadt {} liegt hier!", city.name);
        }
    }
}

fn point_inside_polygon(point: (f32, f32), polygon: &[(f32, f32)]) -> bool {
    let mut inside = false;
    let n = polygon.len();
    let mut j = n - 1;

    for i in 0..n {
        let (xi, yi) = polygon[i];
        let (xj, yj) = polygon[j];

        if (yi < point.1 && yj >= point.1 || yj < point.1 && yi >= point.1)
            && (xi <= point.0 || xj <= point.0)
        {
            if xi + (point.1 - yi) / (yj - yi) * (xj - xi) < point.0 {
                inside = !inside;
            }
        }
        j = i;
    }
    inside
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
        }
    }
    Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_inside_polygon_1() {
    }
}

struct Polygon {
    points: Vec<(f32, f32)>
}

struct State {
    name: String,
    polygon: Polygon,
    insel: Vec<Polygon>,
    loch: Vec<Polygon>
}

impl State {
    fn set_holes_and_islands(&mut self, points: Vec<Vec<(f32, f32)>>) {
        for (i, e) in points.iter().enumerate() {
            let mut is_hole = false;

            if i == 0 {
                continue;
            }
            is_hole = point_inside_polygon(e[0], &points[0]);
            
            if is_hole {
                self.loch.push(Polygon {
                    points: e.clone()
                });
            } else {
                self.insel.push(Polygon {
                    points: e.clone()
                });
            }
        }
    }

    fn get_area(&self) -> f32 {
        let mut area = 0.0;
        area += shoelace_formel(&self.polygon.points);
        for insel in &self.insel {
            area += shoelace_formel(&insel.points);
        }
        for loch in &self.loch {
            area -= shoelace_formel(&loch.points);
        }
        area
    }

    // Funktion: Punkt in Polygon
    fn is_point_inside(&self, point: (f32, f32)) -> bool {
        let mut inside = point_inside_polygon(point, &self.polygon.points);

        for loch in &self.loch {
            inside = point_inside_polygon(point, &loch.points);
            if inside {
                return false;
            }
        }

        for insel in &self.insel {
            inside = point_inside_polygon(point, &insel.points);
            if inside {
                return true;
            }
        }

        inside
    }

    // Get points for drawing
    fn get_points(&self) -> Vec<Vec<(f32, f32)>> {
        let mut points = Vec::new();
        points.push(self.polygon.points.clone());
        for insel in &self.insel {
            points.push(insel.points.clone());
        }
        for loch in &self.loch {
            points.push(loch.points.clone());
        }
        points
    }

    // Draw myself
    fn draw(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut points = Vec::new();
        points.push(self.polygon.points.clone());
        for insel in &self.insel {
            points.push(insel.points.clone());
        }
        for loch in &self.loch {
            points.push(loch.points.clone());
        }
        draw_polygon(format!("states/{}.png", self.name), points)?;
        Ok(())
    }
}