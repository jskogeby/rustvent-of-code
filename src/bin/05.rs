use std::ops::Range;

advent_of_code::solution!(5);

#[derive(Clone)]
struct Mapping {
    dest_start: u64,
    source_start: u64,
    range: u64,
}

struct SourceMap {
    mappings: Vec<Mapping>,
}

impl SourceMap {
    fn new(input: &str) -> Self {
        let (_header, mappings_str) = input.split_once("\n").unwrap();
        let mut mappings = Vec::new();
        for line in mappings_str.lines() {
            let mut line_itr = line.split_whitespace();
            let dest_start = line_itr.next().unwrap().parse::<u64>().unwrap();
            let source_start = line_itr.next().unwrap().parse::<u64>().unwrap();
            let range = line_itr.next().unwrap().parse::<u64>().unwrap();
            mappings.push(Mapping {
                dest_start,
                source_start,
                range,
            });
        }

        Self { mappings }
    }
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<&str>) {
    let mut iter = input.split("\n\n");
    let (_, seeds_str) = iter.next().unwrap().split_once(": ").unwrap();
    let seeds: Vec<u64> = seeds_str
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let maps = iter.collect();

    return (seeds, maps);
}

fn get_location(seed: u64, source_maps: &Vec<SourceMap>) -> u64 {
    let mut location = seed;
    for map in source_maps {
        if let Some(new_location) = map
            .mappings
            .iter()
            .find(|m| (m.source_start..(m.source_start + m.range)).contains(&location))
        {
            location = new_location.dest_start + (location - new_location.source_start);
        }
    }
    location
}

pub fn part_one(input: &str) -> Option<u64> {
    let (seeds, maps) = parse_input(input);
    let source_maps: Vec<SourceMap> = maps.iter().map(|m| SourceMap::new(m)).collect();
    let locations: Vec<u64> = seeds
        .iter()
        .map(|s| get_location(*s, &source_maps))
        .collect();
    let min_location = locations.iter().min().unwrap();
    return Some(*min_location);
}

// If r1 contains r2, then left and right will be defined
fn handle_ranges(r1: &Range<u64>, r2: Range<u64>) -> (Range<u64>, Range<u64>, Range<u64>) {
    if r1.end < r2.start {
        return (r1.start..r1.end, 1..0, 1..0);
    }
    if r2.end < r1.start {
        return (1..0, 1..0, r1.start..r1.end);
    }
    if r2.start < r1.start && r2.end > r1.end {
        return (1..0, r1.start..r1.end, 1..0);
    }
    let left = r1.start..r2.start;
    let min_start = &[r1.start, r2.start].into_iter().max().unwrap();
    let min_end = &[r1.end, r2.end].into_iter().min().unwrap();
    let union = *min_start..*min_end;
    let right = r2.end..r1.end;

    return (left, union, right);
}

fn consume_range(range: Range<u64>, mappings: &Vec<Mapping>) -> Vec<Range<u64>> {
    let mut result = Vec::new();
    if mappings.is_empty() {
        return vec![range];
    }
    let mapping = mappings.first().unwrap();
    let source_range = mapping.source_start..(mapping.source_start + mapping.range);
    let (left, union, right) = handle_ranges(&range, source_range);
    if !union.is_empty() {
        let union_start = mapping.dest_start + (union.start - mapping.source_start);
        let union_end = mapping.dest_start + (union.end - mapping.source_start);
        result.push(union_start..union_end);
    }
    if !left.is_empty() {
        let mut left_result = consume_range(left, &mappings[1..].to_vec());
        result.append(&mut left_result);
    }
    if !right.is_empty() {
        let mut right_result = consume_range(right, &mappings[1..].to_vec());
        result.append(&mut right_result);
    }

    return result;
}

fn transform_ranges(ranges: Vec<Range<u64>>, source_maps: &Vec<SourceMap>) -> Vec<Range<u64>> {
    let mut current_ranges = ranges;
    for map in source_maps {
        current_ranges = current_ranges
            .into_iter()
            .flat_map(|range| consume_range(range, &(map.mappings)))
            .collect::<Vec<Range<u64>>>();
    }
    return current_ranges;
}

pub fn part_two(input: &str) -> Option<u64> {
    let (seeds, maps) = parse_input(input);
    let ranges: Vec<Range<u64>> = seeds
        .chunks(2)
        .map(|c| *c.get(0).unwrap()..(*c.get(0).unwrap() + *c.get(1).unwrap()))
        .collect();
    let source_maps: Vec<SourceMap> = maps.iter().map(|m| SourceMap::new(m)).collect();
    let result = transform_ranges(ranges, &source_maps);
    let min = result.iter().min_by_key(|r| r.start).unwrap().start;
    Some(min)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        let expected = Some(35);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        let expected = Some(46);
        assert_eq!(result, expected);
    }
}
