advent_of_code::solution!(3);

struct Point {
    value_id: String,
    x: i32,
    y: i32,
    gear: bool,
}

struct Matrix {
    matrix: Vec<Vec<char>>,
}

impl Matrix {
    fn get(&self, x: i32, y: i32) -> Option<char> {
        if self.matrix.len() <= y as usize || self.matrix[y as usize].len() <= x as usize {
            return None;
        }
        Some(self.matrix[y as usize][x as usize])
    }
}

struct Number {
    value: i32,
    width: i32,
    x: i32,
    y: i32,
}

impl Number {
    fn adjacents(&self, matrix: &Matrix) -> Vec<Point> {
        let mut adj_vec: Vec<Point> = Vec::new();
        for y in (self.y - 1)..(self.y + 2) {
            for x in self.x - 1..self.x + self.width + 1 {
                if matrix.get(x, y).is_none() {
                    continue;
                }
                if y.eq(&self.y) && x.ge(&self.x) && x.lt(&(self.x + self.width)) {
                    continue;
                }
                let val = matrix.get(x, y).unwrap();
                if !val.is_numeric() && !val.eq(&'.') {
                    adj_vec.push(Point {
                        x,
                        y,
                        gear: val.eq(&'*'),
                        value_id: self.value.to_string() + "," + &x.to_string() + &y.to_string(),
                    });
                }
            }
        }
        adj_vec
    }
}

fn handle_line(line: &str, y: i32) -> Vec<Number> {
    let numbers = line
        .match_indices(|c: char| c.is_numeric())
        .collect::<Vec<(usize, &str)>>();
    let mut current_number = "".to_string();
    let mut num_vec: Vec<Number> = Vec::new();
    for (i, (index, number)) in numbers.iter().enumerate() {
        if current_number.is_empty() {
            current_number.push_str(number);
        }
        if numbers.get(i + 1).is_some() && numbers.get(i + 1).unwrap().0.eq(&(index + 1)) {
            current_number.push_str(numbers.get(i + 1).unwrap().1);
        } else {
            let val = current_number.parse::<i32>().unwrap();
            let width = current_number.len();
            let x = index - (width - 1);
            num_vec.push(Number {
                value: val,
                width: width as i32,
                x: x as i32,
                y,
            });
            current_number = "".to_string();
        }
    }

    num_vec
}

pub fn part_one(input: &str) -> Option<i32> {
    let schema_matrix = Matrix {
        matrix: input.lines().map(|line| line.chars().collect()).collect(),
    };
    let mut numbers: Vec<Number> = Vec::new();
    for (y, line) in input.lines().enumerate() {
        numbers.append(&mut handle_line(line, y as i32));
    }

    let valid_numbers = numbers
        .iter()
        .filter(|n| n.adjacents(&schema_matrix).len() > 0);

    Some(valid_numbers.map(|n| n.value).sum())
}

fn split_n_multiply(input: Vec<Point>) -> i32 {
    let a = input.get(0).unwrap();
    let b = input.get(1).unwrap();
    let (val1, _) = a.value_id.split_once(",").unwrap();
    let (val2, _) = b.value_id.split_once(",").unwrap();

    val1.parse::<i32>().unwrap() * val2.parse::<i32>().unwrap()
}

pub fn part_two(input: &str) -> Option<i32> {
    let schema_matrix = Matrix {
        matrix: input.lines().map(|line| line.chars().collect()).collect(),
    };
    let mut numbers: Vec<Number> = Vec::new();
    for (y, line) in input.lines().enumerate() {
        numbers.append(&mut handle_line(line, y as i32));
    }

    let valid_numbers: Vec<Point> = numbers
        .iter()
        .map(|e| e.adjacents(&schema_matrix))
        .flatten()
        .collect();

    let gears: Vec<&Point> = valid_numbers
        .iter()
        .filter(|e| {
            e.gear
                && valid_numbers
                    .iter()
                    .filter(|ee| ee.x.eq(&e.x) && ee.y.eq(&e.y))
                    .count()
                    == 2
        })
        .collect();

    let sum = gears.iter().fold(0, |acc, e| {
        acc + split_n_multiply(
            gears
                .iter()
                .filter(|ee| ee.x.eq(&e.x) && ee.y.eq(&e.y))
                .map(|ee| Point {
                    x: ee.x,
                    y: ee.y,
                    gear: ee.gear,
                    value_id: ee.value_id.clone(),
                })
                .collect(),
        )
    });

    // Divide by 2 because we are counting each gear twice
    Some(sum / 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        let expected = 4361;
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        let expected = 467835;
        assert_eq!(result, Some(expected));
    }
}
