use day1::{calories_per_elf, items_per_elf, top_three};

fn main() {
    let input = include_str!("../../day1.txt");

    let items_per_elf = items_per_elf(input);
    let mut calories_per_elf = calories_per_elf(&items_per_elf);
    let top_three = top_three(&mut calories_per_elf);

    println!("Sum of top 3: {}", top_three);
}
