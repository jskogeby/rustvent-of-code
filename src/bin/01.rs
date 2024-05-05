advent_of_code::solution!(1);

fn handle_line(acc: u32, line: &str) -> u32 {
    let a = line.chars().find(|c| c.is_numeric()).unwrap();
    let b = line.chars().rev().find(|c| c.is_numeric()).unwrap();

    let mut num_str = "".to_owned();
    num_str.push(a);
    num_str.push(b);
    let num = num_str.parse::<u32>().unwrap();

    return acc + num;
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input.lines().fold(0, handle_line);
    return Some(result);
}

fn handle_line2(acc: u32, line: &str) -> u32 {
    let words = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let head_idx = line
        .find(|c: char| c.is_numeric())
        .unwrap_or(line.len() - 1);
    let head = &line[..head_idx];
    let matching_words = words
        .iter()
        .filter(|w| head.contains(*w))
        .map(|w: &&str| head.match_indices(w).min_by_key(|e| e.0).unwrap());

    let first_match = matching_words.min_by_key(|e| e.0);

    let first_num_string: String;
    if first_match.is_none() {
        let to_int = line.chars().nth(head_idx).unwrap();
        first_num_string = to_int.to_string();
    } else {
        let (_, word) = first_match.unwrap();
        let num = words.iter().position(|w| *w == word).unwrap();
        first_num_string = num.to_string();
    }

    let tail_idx = line.len()
        - 1
        - line
            .chars()
            .rev()
            .position(|c: char| c.is_numeric())
            .unwrap_or(line.len() - 1);
    let tail = &line[tail_idx..];
    let matching_words = words
        .iter()
        .filter(|w| tail.contains(*w))
        .map(|w: &&str| tail.match_indices(w).max_by_key(|e| e.0).unwrap());

    let last_match = matching_words.max_by_key(|e| e.0);

    let last_num_string: String;
    if last_match.is_none() {
        let to_int = line.chars().nth(tail_idx).unwrap();
        last_num_string = to_int.to_string();
    } else {
        let (_, word) = last_match.unwrap();
        let num = words.iter().position(|w| *w == word).unwrap();
        last_num_string = num.to_string();
    }

    let num_str = first_num_string + &last_num_string;
    let num = num_str.parse::<u32>().unwrap();
    return acc + num;
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input.lines().fold(0, handle_line2);
    return Some(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        let expected = 142;
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        let expected = 281;
        assert_eq!(result, Some(expected));
    }
}
