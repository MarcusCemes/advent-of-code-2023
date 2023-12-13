advent_of_code::solution!(13);

use std::mem;

use advent_of_code::tools::*;
use itertools::unfold;

/* == Definitions == */

const HORIZONTAL_MULTIPLIER: usize = 100;

struct Maze<'a> {
    lines: Vec<&'a str>,
}

/* == Solutions == */

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input, 0))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(input, 1))
}

fn solve(input: &str, errors: u32) -> u32 {
    parse_input(input)
        .map(|maze| {
            maze.horizontal_reflection(errors)
                .map(|x| HORIZONTAL_MULTIPLIER * x)
                .or_else(|| maze.vertical_reflection(errors))
                .unwrap() as u32
        })
        .sum()
}

/* == Input parsing == */

/// Stream the input into a sequence of mazes in a single pass.
fn parse_input(input: &str) -> impl Iterator<Item = Maze> {
    unfold((input.lines(), Vec::new(), false), |(lines, acc, ended)| {
        if *ended {
            return None;
        }

        match lines.next() {
            Some(line) => {
                if !line.is_empty() {
                    acc.push(line);
                    return Some(None);
                }
            }
            None => {
                *ended = true;
            }
        }

        let mut lines = Vec::new();
        mem::swap(&mut lines, acc);
        Some(Some(Maze { lines }))
    })
    .flatten()
}

/* == Implementations == */

impl Maze<'_> {
    fn size(&self) -> UCoords {
        UCoords {
            x: self.lines.first().map_or(0, |f| f.len()),
            y: self.lines.len(),
        }
    }

    /// Returns the index of the row that can be cut to get the given
    /// number of reflection errors.
    fn horizontal_reflection(&self, errors: u32) -> Option<usize> {
        for cut in 1..self.size().y {
            let size = cut.min(self.size().y - cut);

            let rows_above = self.lines[cut - size..cut].iter().rev();
            let rows_below = self.lines[cut..cut + size].iter();

            let found_errors = rows_above
                .zip(rows_below)
                .flat_map(|(a, b)| a.bytes().zip(b.bytes()).filter(|(a, b)| a != b))
                .count() as u32;

            if found_errors == errors {
                return Some(cut);
            }
        }

        None
    }

    /// Returns the index of the column that can be cut to get the given
    /// number of reflection errors.
    fn vertical_reflection(&self, errors: u32) -> Option<usize> {
        for cut in 1..self.size().x {
            let size = cut.min(self.size().x - cut);

            let found_errors = (0..size)
                .flat_map(|i| {
                    self.iter_column(cut - i - 1)
                        .zip(self.iter_column(cut + i))
                        .filter(|(a, b)| a != b)
                })
                .count() as u32;

            if found_errors == errors {
                return Some(cut);
            }
        }

        None
    }

    /// Returns an iterator over the characters of the given column.
    fn iter_column(&self, i: usize) -> impl Iterator<Item = u8> + '_ {
        self.lines.iter().map(move |line| line.as_bytes()[i])
    }
}

/* == Tests == */

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::template::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example(DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example(DAY));
        assert_eq!(result, Some(400));
    }
}
