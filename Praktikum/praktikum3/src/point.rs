#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

fn round(x: f64, decimals: u32) -> f64 {
    let y = 10i32.pow(decimals) as f64;
    (x * y).round() / y
}

impl Point {
    pub fn round(&self, decimals: u32) -> Point {
        Point {
            x: round(self.x, decimals),
            y: round(self.y, decimals),
        }
    }

    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

impl Eq for Point {}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let x = f64::total_cmp(&self.x, &other.x);
        match x {
            std::cmp::Ordering::Equal => f64::total_cmp(&self.y, &other.y),
            _ => x,
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}