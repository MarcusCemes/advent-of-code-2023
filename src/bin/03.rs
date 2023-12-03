use colored::Colorize;
use std::fmt::{Debug, Display};

advent_of_code::solution!(3);

#[derive(Debug)]
struct Part {
    length: usize,
    value: u32,
    x: usize,
    y: usize,
}

struct Schematic<'a> {
    line_width: usize,
    lines: Vec<&'a str>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let schematic = Schematic::parse_str(input);

    Some(
        schematic
            .parts()
            .filter(|part| schematic.valid_part(part))
            .map(|p| p.value)
            .sum(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

impl Schematic<'_> {
    fn parse_str(input: &str) -> Schematic {
        let lines: Vec<&str> = input.lines().collect();
        let line_width = lines[0].len();
        Schematic { line_width, lines }
    }

    // fn at(&self, x: usize, y: usize) -> Option<u8> {
    //     self.lines
    //         .get(y)
    //         .and_then(|line| line.as_bytes().get(x).copied())
    // }

    fn parts(&self) -> impl Iterator<Item = Part> + '_ {
        self.lines
            .iter()
            .enumerate()
            .flat_map(|(y, line)| LinePartIterator::new(line, y))
    }

    fn valid_part(&self, part: &Part) -> bool {
        let valid = self.neighbourhood(part).any(|line| {
            line.bytes().any(|char| {
                // print!("{}", char as char);
                !char.is_ascii_digit() && char != b'.'
            })
        });

        // if valid {
        //     println!(" ok");
        // } else {
        //     println!(" bad");
        // }

        valid
    }

    fn neighbourhood(&self, part: &Part) -> impl Iterator<Item = &str> + '_ {
        let t = part.y.saturating_sub(1);
        let b = part.y.saturating_add(1).min(self.lines.len() - 1);
        let l = part.x.saturating_sub(1);
        let r = part.x.saturating_add(part.length).min(self.line_width - 1);

        // print!(" ({}, {}) -> ({}, {}) ", l, t, r, b);
        let n = self.lines[t..=b].iter().map(move |line| {
            // println!("{} -> {}", line, &line[l..=r]);
            &line[l..=r]
        });

        // println!("{:?}", n.clone().join(","));
        n
    }
}

impl Display for Schematic<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parts: Vec<Part> = self.parts().collect();

        for (y, line) in self.lines.iter().enumerate() {
            let mut bytes = line.bytes().enumerate();

            while let Some((x, char)) = bytes.next() {
                if char.is_ascii_digit() {
                    if let Some(part) = parts.iter().find(|part| part.x == x && part.y == y) {
                        if self.valid_part(part) {
                            write!(f, "{}", part.value.to_string().green())?;
                        } else {
                            write!(f, "{}", part.value.to_string().red())?;
                        }

                        for _ in 0..(part.length.saturating_sub(2)) {
                            bytes.next();
                        }
                    }
                } else {
                    write!(f, "{}", char as char)?;
                }
            }

            writeln!(f)?;
        }
        Ok(())
    }
}

impl Debug for Schematic<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.lines {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

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
            x: self.offset,
            y: self.y,
        };

        // print!("{} ({},{})", part.value, part.x, part.y);

        self.window = rest;
        self.offset += value.len() + 1;

        Some(part)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_a() {
        let result = part_one(&advent_of_code::template::read_case(DAY, 1));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_one_b() {
        let result = part_one(&advent_of_code::template::read_case(DAY, 2));
        assert_eq!(result, Some(413));
    }

    #[test]
    fn test_part_one_c() {
        let result = part_one(&advent_of_code::template::read_case(DAY, 3));
        assert_eq!(result, Some(925));
    }

    #[test]
    fn test_part_one_d() {
        let result = part_one(&advent_of_code::template::read_case(DAY, 4));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_case(DAY, 1));
        assert_eq!(result, None);
    }
}
