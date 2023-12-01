const LUT: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let digits = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>();
            10 * digits.first().unwrap() + digits.last().unwrap()
        })
        .reduce(|a, b| a + b)
}

pub fn part_two(input: &str) -> Option<u32> {
    input.lines().map(parse_line).reduce(|a, b| a + b)
}

fn parse_line(line: &str) -> u32 {
    let mut it = (0..line.len()).filter_map(|i| parse_slice(&line[i..]));

    let first = it.next().unwrap();
    let last = it.last().unwrap_or(first);

    10 * first + last
}

fn parse_slice(slice: &str) -> Option<u32> {
    for (word, number) in LUT.iter() {
        if slice.starts_with(*word) {
            return Some(*number);
        }
    }

    slice.chars().next().unwrap().to_digit(10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
