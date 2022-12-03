use day3::{Priority, Rucksacks};

fn main() {
    let input = include_str!("../../day3.txt");

    let rucksacks: Rucksacks = input.into();
    let priority = rucksacks.priority();

    println!("Priorities: {}", priority);
}
