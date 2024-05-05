advent_of_code::solution!(2);

struct Hand {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    id: u32,
    hands: Vec<Hand>,
}

impl Game {
    fn new(line: &str) -> Self {
        let (a, b) = line.split_once(": ").unwrap();
        let id = a.split(" ").nth(1).unwrap().parse::<u32>().unwrap();
        let mut hands = Vec::new();
        for hand_str in b.split("; ") {
            let mut hand = Hand {
                red: 0,
                green: 0,
                blue: 0,
            };
            for color in hand_str.split(", ") {
                let (value, color) = color.split_once(" ").unwrap();
                let value = value.parse::<u32>().unwrap();
                match color {
                    "red" => hand.red = value,
                    "green" => hand.green = value,
                    "blue" => hand.blue = value,
                    _ => panic!("Unknown color: {}", color),
                }
            }
            hands.push(hand);
        }
        Self { id, hands }
    }
    fn get_reds(&self) -> u32 {
        self.hands.iter().map(|hand| hand.red).max().unwrap_or(0)
    }
    fn get_greens(&self) -> u32 {
        self.hands.iter().map(|hand| hand.green).max().unwrap_or(0)
    }
    fn get_blues(&self) -> u32 {
        self.hands.iter().map(|hand| hand.blue).max().unwrap_or(0)
    }
    fn min_reds(&self) -> u32 {
        self.hands.iter().map(|hand| hand.red).max().unwrap_or(0)
    }
    fn min_greens(&self) -> u32 {
        self.hands.iter().map(|hand| hand.green).max().unwrap_or(0)
    }
    fn min_blues(&self) -> u32 {
        self.hands.iter().map(|hand| hand.blue).max().unwrap_or(0)
    }
}

fn possible(game: &Game) -> bool {
    let reds = game.get_reds();
    let greens = game.get_greens();
    let blues = game.get_blues();
    if reds > 12 || greens > 13 || blues > 14 {
        return false;
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let games = input.lines().map(Game::new).filter(possible);
    let sum = games.map(|game| game.id).sum();
    Some(sum)
}

fn fewest_power(game: Game) -> u32 {
    let reds = game.min_reds();
    let greens = game.min_greens();
    let blues = game.min_blues();
    reds * greens * blues
}

pub fn part_two(input: &str) -> Option<u32> {
    let sum = input.lines().map(Game::new).map(fewest_power).sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        let expected = Some(8);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        let expected = Some(2286);
        assert_eq!(result, expected);
    }
}
