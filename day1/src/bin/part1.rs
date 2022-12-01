use day1::{calories_per_elf, items_per_elf, maximum_calories};

fn main() {
    let input = include_str!("../../day1.txt");

    let items_per_elf = items_per_elf(input);
    let calories_per_elf = calories_per_elf(&items_per_elf);
    let maximum_calories = maximum_calories(&calories_per_elf);

    println!("Maximum calories: {}", maximum_calories);
}
