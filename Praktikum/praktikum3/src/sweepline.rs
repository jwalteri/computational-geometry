use std::collections::BTreeSet;
use core::ops::Bound::Unbounded;
use std::ops::Bound::Excluded;
use crate::{line::{Line, SortableLine}, point::Point};

pub struct SweepLine {
    pub segments: BTreeSet<SortableLine> 
}

impl SweepLine {
    pub fn new() -> Self {
        Self {
            segments: BTreeSet::<SortableLine>::new()
        } 
    }

    pub fn insert(&mut self, y: f64, line: Line) {
        self.segments.insert(SortableLine {line: line, index: y});
    }

    pub fn remove(&mut self, line: &SortableLine) {
        self.segments.remove(line); 
    }

    pub fn remove_by_line(&mut self, line: &Line) {
        let copy = self.segments.clone();
        let element = copy.iter().find(|x| x.line == *line).unwrap();

        self.segments.remove(element); 
    }

    pub fn update(&mut self, x: f64) {
        let mut tmp = BTreeSet::new();
        for line in self.segments.iter() {
            let mut new_line = line.clone();
            new_line.index = new_line.line.y(x);
            tmp.insert(new_line);
        }

        self.segments = tmp;
    }

    pub fn get_neighbors(&self, line: &Line) -> (Option<SortableLine>, Option<SortableLine>) { 
        let element = self.segments.iter().find(|x| x.line == *line).unwrap();

        // Find the line above and below the target line
        let above = self.segments.range((Unbounded, Excluded(element))).next_back();
        let below = self.segments.range((Excluded(element), Unbounded)).next();

        (below.cloned(), above.cloned())
    }

    pub fn find_by_line(&self, line: &Line) -> Option<SortableLine> {
        self.segments.iter().find(|x| x.line == *line).cloned()
    }

    pub fn swap(&mut self, line1: &Line, line2: &Line, intersection_point: &Point) -> (Option<SortableLine>, SortableLine, SortableLine, Option<SortableLine>) {

        let l1 = self.find_by_line(line1).unwrap();
        let l2 = self.find_by_line(line2).unwrap();

        // Hinweis: Delta auf x rechnen! Das is es!
        let delta = 1e-9;

        self.remove(&l1);
        self.remove(&l2);

        let mut l1 = l1.clone();
        let mut l2 = l2.clone();
        l1.index = line1.y(intersection_point.x + delta);
        l2.index = line2.y(intersection_point.x + delta);

        self.segments.insert(l1.clone());
        self.segments.insert(l2.clone());

        let lower_index = l1.index.min(l2.index);
        let higher_index = l1.index.max(l2.index);

        let copy = self.segments.clone();        
        let lower = copy.iter().find(|x| x.index == lower_index).unwrap();
        let higher = copy.iter().find(|x| x.index == higher_index).unwrap();

        let (below, _) = self.get_neighbors(&lower.line);
        let (_, above) = self.get_neighbors(&higher.line);

        (below, lower.clone(), higher.clone(), above)
    }
}
