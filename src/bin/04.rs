use std::collections::{HashMap, HashSet};

advent_of_code::solution!(4);

struct CardCollection {
    cards: Vec<Card>,
}

impl CardCollection {
    fn calculate_total(&self) -> u32 {
        let mut copy_map: HashMap<u32, u32> = HashMap::new();
        for card in &self.cards {
            let copies = card.get_copies();
            for i in (card.id + 1)..(card.id + copies + 1) {
                let current_card_copies = copy_map.get(&card.id).unwrap_or(&0);
                let prev_copies = copy_map.get(&i).unwrap_or(&0);
                copy_map.insert(i, prev_copies + current_card_copies + 1);
            }
        }
        self.cards.len() as u32 + copy_map.values().sum::<u32>()
    }
}

struct Card {
    id: u32,
    winning: HashSet<u32>,
    actual: HashSet<u32>,
}

impl Card {
    fn new(line: &str) -> Self {
        let (card_str, numbers) = line.split_once(": ").unwrap();
        let id = card_str.split_whitespace().nth(1).unwrap();
        let (win_str, act_str) = numbers.split_once(" | ").unwrap();
        let winning: HashSet<u32> = win_str
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let actual: HashSet<u32> = act_str
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        Self {
            id: id.parse::<u32>().unwrap(),
            winning,
            actual,
        }
    }
    fn get_points(&self) -> u32 {
        let winning_actual: HashSet<&u32> = self
            .actual
            .iter()
            .filter(|num| self.winning.contains(num))
            .collect();
        let base: u32 = 2;
        if winning_actual.is_empty() {
            return 0;
        }
        base.pow((winning_actual.len() - 1) as u32)
    }
    fn get_copies(&self) -> u32 {
        let winning_actual: HashSet<&u32> = self
            .actual
            .iter()
            .filter(|num| self.winning.contains(num))
            .collect();
        winning_actual.len() as u32
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let sum_points = input
        .lines()
        .map(Card::new)
        .fold(0, |acc, card| acc + card.get_points());

    Some(sum_points)
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = input.lines().map(Card::new).collect::<Vec<Card>>();

    Some(CardCollection { cards }.calculate_total())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        let expected = 13;
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        let expected = 30;
        assert_eq!(result, Some(expected));
    }
}
