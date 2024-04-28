use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let points = read_points("states/Bayern.txt")?;
    draw_polygon(&points)?;
    Ok(())
}


fn read_points<P>(filename: P) -> io::Result<Vec<(f32, f32)>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut points = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let coords: Vec<f32> = line.split(',')
                                   .map(|s| s.parse().unwrap())
                                   .collect();
        if coords.len() == 2 {
            points.push((coords[0], coords[1]));
        }
    }

    Ok(points)
}

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