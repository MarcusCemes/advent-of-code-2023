advent_of_code::solution!(1);

const LUT: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(parse_line_1).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().map(parse_line_2).sum())
}

fn parse_line_1(line: &str) -> u32 {
    let first = line.chars().find_map(|c| c.to_digit(10));
    let last = line.chars().rev().find_map(|c| c.to_digit(10));
    10 * first.unwrap() + last.unwrap()
}

fn parse_line_2(line: &str) -> u32 {
    let first = find_pattern(0..line.len(), line);
    let last = find_pattern((0..line.len()).rev(), line);
    10 * first + last
}

fn find_pattern(mut it: impl Iterator<Item = usize>, line: &str) -> u32 {
    it.find_map(|i| compare_slice(&line[i..])).unwrap()
}

fn compare_slice(slice: &str) -> Option<u32> {
    LUT.iter()
        .enumerate()
        .find(|(_, pattern)| slice.starts_with(*pattern))
        .map(|(i, _)| i as u32 + 1)
        .or_else(|| slice.chars().next().unwrap().to_digit(10))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_part(DAY, 1));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_part(DAY, 2));
        assert_eq!(result, Some(281));
    }
}
