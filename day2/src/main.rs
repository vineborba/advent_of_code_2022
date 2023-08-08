use std::{collections::HashMap, fs};

// Result points
// loss = 0
// draw = 3
// win = 6

// Choice points
// rock = 1
// paper = 2
// scissors = 3

// Choices
// rock = A
// paper = B
// scissor = C

// Results
// draw = Y
// lose = X
// win = Z

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't find input.txt");
    let mut results = HashMap::new();
    // Draws
    results.insert("A X", 4);
    results.insert("B Y", 5);
    results.insert("C Z", 6);
    // Wins
    results.insert("A Y", 8);
    results.insert("B Z", 9);
    results.insert("C X", 7);
    // Losses
    results.insert("A Z", 3);
    results.insert("B X", 1);
    results.insert("C Y", 2);

    let final_score = input
        .lines()
        .fold(0, |acc, line| acc + results.get(line).unwrap());
    println!("Part 1 final score is: {}", final_score);

    let mut results = HashMap::new();
    // Draws
    results.insert("A Y", 4);
    results.insert("B Y", 5);
    results.insert("C Y", 6);
    // Wins
    results.insert("A Z", 8);
    results.insert("B Z", 9);
    results.insert("C Z", 7);
    // Losses
    results.insert("A X", 3);
    results.insert("B X", 1);
    results.insert("C X", 2);

    let final_score = input
        .lines()
        .fold(0, |acc, line| acc + results.get(line).unwrap());
    println!("Part 2 final score is: {}", final_score);
}
