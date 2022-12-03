use day3::{Priority, Rucksacks};

fn main() {
    let input = include_str!("../../day3.txt");

    let sanitized_input: Vec<&str> = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();

    let groups: Vec<Rucksacks> = sanitized_input
        .chunks(3)
        .map(|chunk| Rucksacks {
            rucksacks: vec![
                (*chunk.first().unwrap()).into(),
                (*chunk.get(1).unwrap()).into(),
                (*chunk.get(2).unwrap()).into(),
            ],
        })
        .collect();

    let shared_items_priority: usize = groups
        .iter()
        .map(|rucksacks| rucksacks.shared_item().unwrap().priority())
        .sum();

    println!("Priority of shared items: {}", shared_items_priority);
}
