use std::fs;

struct Elf {
    id: i32,
    calories: i32,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't find input.txt");
    let mut counter = 1;
    let mut elvens: Vec<Elf> = input
        .split("\n\n")
        .map(|block| {
            let mut acc = 0;
            for line in block.lines() {
                acc += line.parse::<i32>().unwrap();
            }
            counter += 1;
            Elf {
                id: counter,
                calories: acc,
            }
        })
        .collect();

    elvens.sort_by(|a, b| b.calories.cmp(&a.calories));
    let elf = elvens.first().unwrap();
    println!(
        "Elf with most calories: {} with {} calories",
        elf.id, elf.calories
    );

    let backups = elvens
        .get(0..3)
        .unwrap()
        .iter()
        .fold(0, |acc, elf| acc + elf.calories);
    println!("Backup calories: {}", backups);
}
