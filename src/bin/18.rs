advent_of_code::solution!(18);

use advent_of_code::tools::{algorithms::enclosed_area, *};

/* == Definitions == */

struct Instruction {
    direction: Direction,
    length: u32,
}

enum Direction {
    Down,
    Left,
    Right,
    Up,
}

/* == Solutions == */

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, parse_instruction_1))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, parse_instruction_2))
}

/// Re-uses the shoelace formula from Day 10 to compute the area of the polygon.
fn solve(input: &str, parser: impl Fn(&str) -> Instruction) -> u64 {
    let mut length = 0;

    let path = input
        .lines()
        .map(parser)
        .scan(Coords::default(), |coords, instruction| {
            length += instruction.length as u64;
            *coords += instruction.direction.offset(instruction.length as i64);
            Some(*coords)
        });

    enclosed_area(path) + length / 2 + 1
}

/* == Input parsing == */

fn parse_instruction_1(line: &str) -> Instruction {
    let (direction, length, _) = parse_line(line);
    Instruction {
        direction: Direction::parse_digit(direction),
        length: length.parse().unwrap(),
    }
}

fn parse_instruction_2(line: &str) -> Instruction {
    let (_, _, hex) = parse_line(line);
    let direction_byte = (hex.as_bytes()[7] as char).to_digit(16).unwrap();

    Instruction {
        direction: Direction::parse_hex(direction_byte as u8),
        length: u32::from_str_radix(&hex[2..7], 16).unwrap(),
    }
}

fn parse_line(line: &str) -> (u8, &str, &str) {
    let mut parts = line.split_ascii_whitespace();

    (
        parts.next().unwrap().bytes().next().unwrap(),
        parts.next().unwrap(),
        parts.next().unwrap(),
    )
}

/* == Implementations == */

impl Direction {
    fn parse_digit(c: u8) -> Self {
        match c {
            b'U' => Self::Up,
            b'D' => Self::Down,
            b'L' => Self::Left,
            b'R' => Self::Right,
            _ => unreachable!(),
        }
    }

    fn parse_hex(src: u8) -> Self {
        match src {
            0 => Self::Left,
            1 => Self::Down,
            2 => Self::Right,
            3 => Self::Up,
            _ => unreachable!(),
        }
    }

    fn offset(&self, factor: i64) -> Coords {
        match self {
            Self::Up => Coords::new(0, -factor),
            Self::Down => Coords::new(0, factor),
            Self::Left => Coords::new(-factor, 0),
            Self::Right => Coords::new(factor, 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::template::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example(DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example(DAY));
        assert_eq!(result, Some(952408144115));
    }
}
