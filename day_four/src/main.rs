use std::fs;

struct Range(u32, u32);

impl Range {
    fn new(range: &str) -> Self {
        match range.split_once('-') {
            Some((start, end)) => Range(start.parse::<u32>().unwrap(), end.parse::<u32>().unwrap()),
            _ => Range(0, 0),
        }
    }

    fn contains(&self, other: &Range) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.0 <= other.0 && self.1 >= other.0 && self.1 <= other.1
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("can't open input file");

    let mut contained_counter = 0;
    let mut overlaped_counter = 0;
    for line in content.lines() {
        if let Some((r1, r2)) = line.split_once(',') {
            let first_range = Range::new(r1);
            let second_range = Range::new(r2);
            if first_range.contains(&second_range) || second_range.contains(&first_range) {
                contained_counter += 1;
                overlaped_counter += 1;
            } else if first_range.overlaps(&second_range) || second_range.overlaps(&first_range) {
                overlaped_counter += 1;
            }
        }
    }
    println!("Fully contained: {contained_counter}");
    println!("Overlaped: {overlaped_counter}");
}
