use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let points = read_points("states/Bayern.txt")?;


    
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

    // Ursprungskoordinaten
    let origin_x = points[0].0;
    let origin_y = points[0].1;
    
    // Konvertierung der relativen Punkte in absolute Koordinaten
    let start = points.remove(0); // Startpunkt am Anfang raus
    points.pop(); // Startpunkt am Ende raus
    let mut absolute_points: Vec<(f32, f32)> = relative_to_absolute(&points, origin_x, origin_y);
    absolute_points.insert(0, start); // Startpunkt wieder einfügen

    // Ausgabe der absoluten Punkte
    println!("Absolute Punkte:");
    for point in &absolute_points {
        println!("({}, {})", point.0, point.1);
    }

    Ok(absolute_points)
}

// DEPRECATED: Versucht ein Polygon zu zeichnen
fn draw_polygon(points: &[(f32, f32)]) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("polygon.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..1f32, 0f32..1f32)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        points.iter().cloned().cycle().take(points.len() + 1),
        &RED,
    ))?;

    root.present()?;
    Ok(())
}

// Transformiert die relativen Punkte zu absoluten Punkten
fn relative_to_absolute(relative_points: &Vec<(f32, f32)>, origin_x: f32, origin_y: f32) -> Vec<(f32, f32)> {
    let mut absolute_points = Vec::new();
    let mut current_x = origin_x;
    let mut current_y = origin_y;

    for &(x, y) in relative_points {
        let absolute_x = current_x + x;
        let absolute_y = current_y + y;
        absolute_points.push((absolute_x, absolute_y));
        current_x = absolute_x;
        current_y = absolute_y;
    }

    absolute_points
}