pub use self::item::*;
pub use self::rucksack::*;

mod item;
mod rucksack;

pub trait Priority {
    fn priority(&self) -> usize;
}

pub struct Rucksacks {
    pub rucksacks: Vec<Rucksack>,
}

impl Rucksacks {
    pub fn shared_item(&self) -> Option<Item> {
        if self.rucksacks.len() <= 1 {
            return None;
        }

        let rucksack = self.rucksacks.get(0).unwrap();

        for item in rucksack.all_items() {
            if self
                .rucksacks
                .iter()
                .skip(1)
                .all(|rucksack| rucksack.all_items().any(|other| other == item))
            {
                return Some(*item);
            }
        }

        None
    }
}

impl Priority for Rucksacks {
    fn priority(&self) -> usize {
        self.rucksacks
            .iter()
            .map(|rucksack| rucksack.priority())
            .sum()
    }
}

impl From<&str> for Rucksacks {
    fn from(input: &str) -> Self {
        Rucksacks {
            rucksacks: input
                .lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
                .map(|line| line.into())
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn priority() {
        let input = r#"
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
        "#;

        let rucksacks: Rucksacks = input.into();

        assert_eq!(157, rucksacks.priority());
    }

    #[test]
    fn shared_item() {
        let rucksacks = Rucksacks {
            rucksacks: vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp".into(),
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".into(),
                "PmmdzqPrVvPwwTWBwg".into(),
            ],
        };

        let shared_item = rucksacks.shared_item().unwrap();

        assert_eq!(Item('r'), shared_item);
    }

    #[test]
    fn trait_from_str() {
        let input = r#"
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
        "#;

        let rucksacks: Rucksacks = input.into();

        assert_eq!(6, rucksacks.rucksacks.len());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Rucksacks>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Rucksacks>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Rucksacks>();
    }
}
