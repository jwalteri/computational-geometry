use avl_tree::AvlTreeMap;
use std::cmp::Ordering;


// Struktur, um Punkte zu repräsentieren
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

// Struktur, um Segmente zu repräsentieren
#[derive(Debug)]
struct Segment {
    left_point: Point,
    right_point: Point,
}

impl Ord for Point {
    // Vergleiche Punkte nach x und y
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x != other.x {
            self.x.cmp(&other.x)
        } else {
            self.y.cmp(&other.y)
        }
    }
}

impl PartialOrd for Point {
    // Teilweise Ordnung für Punkte nach x und y
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    
    // Erstelle eine Liste von Punkten
    // TODO: aus aufg 1 ziehen
    let mut x: Vec<Point> = vec![
        Point { x: 1, y: 5 },
        Point { x: 3, y: 7 },
        Point { x: 5, y: 3 },
        Point { x: 2, y: 8 },
        Point { x: 4, y: 6 },
        Point { x: 6, y: 4 },
    ];

    // Sortiere x nach x-Koordinaten und dann nach y-Koordinaten
    x.sort();

    // Initialisiere Sweep Line (SL) als leeren AVL-Baum
    let mut sweep_line: AvlTreeMap<Point, Segment> = AvlTreeMap::new();

    // Initialisiere Ausgabe-Intersection-Liste L als leeren Vektor
    let mut intersection_list: Vec<Point> = Vec::new();

    // Solange x nicht leer ist
    while let Some(event) = x.pop() {
        if let Some(segment) = sweep_line.get(&event) {
            // Ereignis ist ein rechter Endpunkt
            TreatRightEndpoint(segment, &mut sweep_line);
        } else {
            // Ereignis ist ein linker Endpunkt
            let left_endpoint = event;
            let right_endpoint = find_right_endpoint(&left_endpoint, &sweep_line);
            let segment = Segment {
                left_point: left_endpoint,
                right_point: right_endpoint,
            };
            TreatLeftEndpoint(segment, &mut sweep_line);
        }
    }

    // Rückgabe der Ausgabe-Intersection-Liste L
    println!("Intersection List: {:?}", intersection_list);
}

fn TreatLeftEndpoint(segment: Segment, sweep_line: &mut AvlTreeMap<Point, Segment>) {
    // Füge das Segment zur Sweep Line hinzu
    sweep_line.insert(segment.left_point, segment);
}

fn TreatRightEndpoint(segment: &Segment, sweep_line: &mut AvlTreeMap<Point, Segment>) {
    // Entferne das Segment aus der Sweep Line
    sweep_line.remove(&segment.left_point);
}

fn find_right_endpoint(left_endpoint: &Point, sweep_line: &AvlTreeMap<Point, Segment>) -> Point {
    // Finde den rechten Endpunkt des Segments in der Sweep Line
    // Hier müsste die Implementierung je nach deinem spezifischen Datenmodell angepasst werden
    // Dies ist nur ein Platzhalter, der den nächsten rechten Endpunkt zurückgibt.
    sweep_line.iter().next().unwrap().0
}
}

