use std::hash::Hasher;
use std::hash::Hash;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }

    // Korrdinaten auf 5 Kommastellen runden
    pub fn round(&self) -> Point {
        Point {
            x: (self.x * 100000.0).round() / 100000.0,
            y: (self.y * 100000.0).round() / 100000.0,
        }
    }
}

// Eq für Point implementieren
impl Eq for Point {}

// Hash für Point implementieren
impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
    }
}