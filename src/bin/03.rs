#![feature(iter_advance_by)]

use colored::Colorize;
use std::fmt::Display;

use advent_of_code::tools::{Coords, UCoords};

advent_of_code::solution!(3);

const GEAR: u8 = b'*';

struct Schematic<'a> {
    line_width: usize,
    lines: Vec<&'a str>,
}

struct Part {
    length: usize,
    value: u32,
    coords: UCoords,
}

struct Gear {
    ratio: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let schematic = Schematic::parse_str(input);

    Some(
        schematic
            .iter_parts()
            .filter(|part| schematic.valid_part(part))
            .map(|p| p.value)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        Schematic::parse_str(input)
            .iter_gears()
            .map(|gear| gear.ratio)
            .sum(),
    )
}

impl Schematic<'_> {
    fn parse_str(input: &str) -> Schematic {
        let lines: Vec<&str> = input.lines().collect();
        let line_width = lines[0].len();
        Schematic { line_width, lines }
    }

    /// Linear scan of the schematic, yielding each found part.
    fn iter_parts(&self) -> impl Iterator<Item = Part> + '_ {
        self.lines
            .iter()
            .enumerate()
            .flat_map(|(y, line)| LinePartIterator::new(line, y))
    }

    /// Checks whether the part has any adjacent non-digit non-period characters.
    fn valid_part(&self, part: &Part) -> bool {
        self.neighbourhood(part).any(|line| {
            line.bytes()
                .any(|char| !char.is_ascii_digit() && char != b'.')
        })
    }

    /// Returns string slices that cover the immediate neighbourhood of a part.
    /// Returning string slices allows for easy and safe linear iteration, without
    /// having to worry about bounds checking.
    fn neighbourhood(&self, part: &Part) -> impl Iterator<Item = &str> + '_ {
        let t = part.coords.y.saturating_sub(1);
        let b = part.coords.y.saturating_add(1).min(self.lines.len() - 1);
        let l = part.coords.x.saturating_sub(1);
        let r = part
            .coords
            .x
            .saturating_add(part.length)
            .min(self.line_width - 1);

        self.lines[t..=b].iter().map(move |line| &line[l..=r])
    }

    /// Linear scan of schematic, yielding each found gear.
    fn iter_gears(&self) -> impl Iterator<Item = Gear> + '_ {
        self.lines.iter().enumerate().flat_map(move |(y, line)| {
            line.bytes().enumerate().filter_map(move |(x, char)| {
                (char == GEAR)
                    .then(|| self.read_gear(UCoords { x, y }))
                    .flatten()
            })
        })
    }

    /// Decodes a gear, if it is valid (has exactly two adjacent parts).
    fn read_gear(&self, at: UCoords) -> Option<Gear> {
        if self.get(at)? != GEAR {
            return None;
        }

        let mut i = 0;
        let mut ratio = 1;

        for part in self.adjacent_parts(at).take(3) {
            i += 1;
            ratio *= part.value;
        }

        (i == 2).then_some(Gear { ratio })
    }

    // Returns any parts that are adjacent to the given coordinates.
    fn adjacent_parts(&self, at: UCoords) -> impl Iterator<Item = Part> + '_ {
        let coords: Coords = at.into();

        (-1..=1)
            .flat_map(move |dy| {
                self.horizontally_adjacent_parts(Coords {
                    x: coords.x,
                    y: coords.y + dy,
                })
            })
            .flatten()
    }

    /// Given a coordinate, returns between 0 and 2 parts that can be found in
    /// a 3-wide horizontal window (to the left, in it and to the right).
    /// If the coordinate is a part, there can't be parts to the left or the right.
    fn horizontally_adjacent_parts(&self, at: Coords) -> [Option<Part>; 2] {
        match self.part_at(at) {
            Some(part) => [Some(part), None],
            None => [
                self.part_at(at + (-1, 0).into()),
                self.part_at(at + (1, 0).into()),
            ],
        }
    }

    /// Finds a part that is located at the given coordinates.
    /// Scans left and right to find the start and end of the part.
    fn part_at(&self, at: Coords) -> Option<Part> {
        let UCoords { x, y } = at.ucoords(&self.size())?;
        let line = self.lines.get(y).unwrap();

        if !line.as_bytes().get(x).unwrap().is_ascii_digit() {
            return None;
        }

        let start = line[..x]
            .rfind(|c: char| !c.is_ascii_digit())
            .map(|idx| idx + 1)
            .unwrap_or(0);

        let end = line[x..]
            .find(|c: char| !c.is_ascii_digit())
            .map(|idx| idx + x)
            .unwrap_or_else(|| line.len());

        let value = line[start..end].parse().unwrap();

        Some(Part {
            value,
            length: end - start,
            coords: UCoords { x: start, y },
        })
    }

    /// Returns the character at the given coordinates, if it's in bounds.
    fn get(&self, at: UCoords) -> Option<u8> {
        self.lines
            .get(at.y)
            .and_then(|line| line.as_bytes().get(at.x).copied())
    }

    /// Returns the size of the schematic.
    fn size(&self) -> UCoords {
        UCoords {
            x: self.line_width,
            y: self.lines.len(),
        }
    }
}

/// Iterator that does a linear scan of the schematic, line by line,
/// yielding each found part.
struct LinePartIterator<'a> {
    offset: usize,
    window: Option<&'a str>,
    y: usize,
}

impl LinePartIterator<'_> {
    fn new(line: &str, y: usize) -> LinePartIterator {
        LinePartIterator {
            offset: 0,
            window: if line.is_empty() { None } else { Some(line) },
            y,
        }
    }
}

impl Iterator for LinePartIterator<'_> {
    type Item = Part;

    fn next(&mut self) -> Option<Self::Item> {
        let window = self.window?;

        let idx = window.find(|c: char| c.is_ascii_digit())?;
        self.offset += idx;

        let (_, tail) = window.split_at(idx);

        let (value, rest) = match tail.split_once(|c: char| !c.is_ascii_digit()) {
            Some((value, rest)) => (value, Some(rest)),
            None => (tail, None),
        };

        let part = Part {
            value: value.parse().unwrap(),
            length: value.len(),
            coords: UCoords {
                x: self.offset,
                y: self.y,
            },
        };

        self.window = rest;
        self.offset += value.len() + 1;

        Some(part)
    }
}

/* == Visualisation == */

impl Display for Schematic<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, line) in self.lines.iter().enumerate() {
            let mut bytes = line.bytes().enumerate();

            while let Some((x, char)) = bytes.next() {
                if let Some(part) = self.part_at(UCoords { x, y }.into()) {
                    let value = part.value.to_string();
                    let valid = self.valid_part(&part);
                    let stylised = if valid { value.green() } else { value.red() };

                    write!(f, "{stylised}")?;
                    let _ = bytes.advance_by(part.length.saturating_sub(1));
                } else {
                    write!(f, "{}", char as char)?;
                }
            }

            writeln!(f)?;
        }
        Ok(())
    }
}

/* == Tests == */

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::template::*;

    #[test]
    fn test_part_one_a() {
        let result = part_one(&read_example_part(DAY, 1));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_one_b() {
        let result = part_one(&read_example_part(DAY, 2));
        assert_eq!(result, Some(413));
    }

    #[test]
    fn test_part_one_c() {
        let result = part_one(&read_example_part(DAY, 3));
        assert_eq!(result, Some(925));
    }

    #[test]
    fn test_part_one_d() {
        let result = part_one(&read_example_part(DAY, 4));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two_a() {
        let result = part_two(&read_example_part(DAY, 1));
        assert_eq!(result, Some(467835));
    }

    #[test]
    fn test_part_two_b() {
        let result = part_two(&read_example_part(DAY, 4));
        assert_eq!(result, Some(442));
    }
}
