use crate::{Item, Priority};
use std::iter::Iterator;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Rucksack {
    pub compartment1: Vec<Item>,
    pub compartment2: Vec<Item>,
}

impl Rucksack {
    pub fn all_items(&self) -> impl Iterator<Item = &Item> {
        self.compartment1.iter().chain(self.compartment2.iter())
    }

    fn misplaced_items(&self) -> Vec<Item> {
        let mut misplaced_items = Vec::new();

        for item_in_compartment1 in self.compartment1.iter() {
            if self.compartment2.contains(item_in_compartment1)
                && !misplaced_items.contains(item_in_compartment1)
            {
                misplaced_items.push(*item_in_compartment1);
            }
        }

        misplaced_items
    }
}

impl Priority for Rucksack {
    fn priority(&self) -> usize {
        self.misplaced_items()
            .iter()
            .map(|item| item.priority())
            .sum()
    }
}

impl From<&str> for Rucksack {
    fn from(input: &str) -> Self {
        let items: Vec<char> = input.chars().collect();
        let count = items.len();

        let mut chunks = items.chunks(count / 2);

        let compartment1 = chunk_to_compartment(chunks.next().unwrap());
        let compartment2 = chunk_to_compartment(chunks.next().unwrap());

        Rucksack {
            compartment1,
            compartment2,
        }
    }
}

fn chunk_to_compartment(chunk: &[char]) -> Vec<Item> {
    chunk.iter().map(|char| Item(*char)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn priority() {
        let rucksack: Rucksack = "PmmdzqPrVvPwwTWBwg".into();

        assert_eq!(42, rucksack.priority());
    }

    #[test]
    fn trait_from_str() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp";

        let rucksack: Rucksack = input.into();

        assert_eq!(
            Rucksack {
                compartment1: vec![
                    Item('v'),
                    Item('J'),
                    Item('r'),
                    Item('w'),
                    Item('p'),
                    Item('W'),
                    Item('t'),
                    Item('w'),
                    Item('J'),
                    Item('g'),
                    Item('W'),
                    Item('r'),
                ],
                compartment2: vec![
                    Item('h'),
                    Item('c'),
                    Item('s'),
                    Item('F'),
                    Item('M'),
                    Item('M'),
                    Item('f'),
                    Item('F'),
                    Item('F'),
                    Item('h'),
                    Item('F'),
                    Item('p'),
                ]
            },
            rucksack
        );
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Rucksack>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Rucksack>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Rucksack>();
    }
}
