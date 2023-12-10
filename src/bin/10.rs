#![feature(test)]

extern crate test;

advent_of_code::solution!(10);

use std::{fmt::Debug, iter};

use advent_of_code::tools::*;
use itertools::{unfold, Itertools};

/* == Definitions == */

const START: u8 = b'S';

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Connection {
    Horizontal,
    None,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Vertical,
}

struct Maze<'a> {
    lines: Vec<&'a str>,
    size: UCoords,
    start: UCoords,
}

/* == Solutions == */

/// The distance to the farthest cell is half the length of the path.
pub fn part_one(input: &str) -> Option<u32> {
    Some(Maze::parse_input(input).find_path().count() as u32 / 2)
}

/// An elegant solution that makes use of the area of the polygon, formed
/// by the path using the shoelace formula, in order to compute the number of
/// fully enclosed cells within the closed loop. This is efficient to compute,
/// but requires a correcting factor to account for the fractional area of the
/// cells that are only partially enclosed by the path.
///
/// This fractional area is half of the length of the path (since each cell is a
/// square of side length 1), minus an additional 1 to compensate for the 4 left/right
/// angles containing an area of 0.25 that are required to close the loop. This works for
/// any loop, as any additional left turns compensate the additional right turns,
/// and vice versa.
pub fn part_two(input: &str) -> Option<u32> {
    let mut length = 0;

    let maze = Maze::parse_input(input);
    let path = maze.find_path().inspect(|_| length += 1);
    let area = enclosed_area(path);

    Some(area - length / 2 + 1)
}

/* == Implementations == */

impl Maze<'_> {
    fn parse_input(input: &str) -> Maze {
        let lines: Vec<_> = input.lines().collect();
        let size = UCoords::new(lines[0].len(), lines.len());

        let start = lines
            .iter()
            .enumerate()
            .find_map(|(y, line)| {
                line.bytes()
                    .find_position(|&c| c == START)
                    .map(|(x, _)| UCoords::new(x, y))
            })
            .unwrap();

        Maze { lines, size, start }
    }

    fn get(&self, coords: UCoords) -> Connection {
        self.lines[coords.y].as_bytes()[coords.x].into()
    }

    fn find_path(&self) -> impl Iterator<Item = UCoords> + '_ {
        let last = self.start;
        let current = self.connecting_cells(self.start).next().unwrap();

        iter::once(last).chain(iter::once(current)).chain(unfold(
            (last, current),
            |(last, current)| {
                let next = self.connected_cells(*current).find(|c| c != last).unwrap();

                if next == self.start {
                    None
                } else {
                    *last = *current;
                    *current = next;
                    Some(next)
                }
            },
        ))
    }

    /// Returns the cells that are connected to the current cell, based on the symbol
    /// of *adjacent* cells. This is the inverse of `connected_cells()`, and is used
    /// to compute unknown connection types.
    fn connecting_cells(&self, to: UCoords) -> impl Iterator<Item = UCoords> + '_ {
        let from_coords = Coords::from(to);

        // Iterate over adjacent cells that are connections, compute their connection
        // offsets and find the one that leads back to the current cell.
        self.adjacent_cells(to)
            .flat_map(|ucoords| {
                let offsets = self.get(ucoords).offsets();
                offsets.map(|os| os.map(|o| (ucoords, Coords::from(ucoords) + o)))
            })
            .flatten()
            .filter(move |(_, connected_cell)| *connected_cell == from_coords)
            .map(|(coords, _)| coords)
    }

    /// Returns the cells that the current cell, based on the symbol of the *current* cell.
    fn connected_cells(&self, at: UCoords) -> impl Iterator<Item = UCoords> + '_ {
        self.get(at)
            .offsets()
            .into_iter()
            .map(move |offsets| offsets.map(|o| Coords::from(at) + o))
            .flatten()
            .filter_map(|coords| coords.ucoords(&self.size))
    }

    /// Returns all adjacent cells that are in the bounds of the maze.
    fn adjacent_cells(&self, at: UCoords) -> impl Iterator<Item = UCoords> + '_ {
        let coords = Coords::from(at);

        (-1_i64..=1)
            .cartesian_product(-1..=1)
            .filter(|(i, j)| i != j)
            .map(move |(i, j)| coords + (i, j).into())
            .filter_map(|coords| coords.ucoords(&self.size))
    }
}

impl Connection {
    fn offsets(&self) -> Option<[Coords; 2]> {
        let c = Coords::new;

        Some(match self {
            Connection::Horizontal => [c(-1, 0), c(1, 0)],
            Connection::NorthEast => [c(0, -1), c(1, 0)],
            Connection::NorthWest => [c(-1, 0), c(0, -1)],
            Connection::SouthEast => [c(0, 1), c(1, 0)],
            Connection::SouthWest => [c(-1, 0), c(0, 1)],
            Connection::Vertical => [c(0, -1), c(0, 1)],
            Connection::None => return None,
        })
    }
}

impl From<u8> for Connection {
    fn from(value: u8) -> Self {
        match value {
            b'-' => Connection::Horizontal,
            b'L' => Connection::NorthEast,
            b'J' => Connection::NorthWest,
            b'F' => Connection::SouthEast,
            b'7' => Connection::SouthWest,
            b'|' => Connection::Vertical,
            _ => Connection::None,
        }
    }
}

/* == Functions == */

/// Computes the enclosed area of a polygon, given its vertices,
/// using the shoelace formula (fast determinant-based version).
fn enclosed_area(mut path: impl Iterator<Item = UCoords>) -> u32 {
    let first = path.next().unwrap();

    let sum: i64 = iter::once(first)
        .chain(path)
        .chain(iter::once(first))
        .map(Coords::from)
        .tuple_windows()
        .map(|(a, b)| a.x * b.y - b.x * a.y)
        .sum();

    sum.abs() as u32 / 2
}

/* == Tests ==  */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_part(DAY, 1));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two_a() {
        let result = part_two(&advent_of_code::template::read_part(DAY, 1));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two_b() {
        let result = part_two(&advent_of_code::template::read_part(DAY, 2));
        assert_eq!(result, Some(4));
    }

    #[bench]
    fn profile_input_parsing(b: &mut test::Bencher) {
        let input = advent_of_code::template::read_file("inputs", DAY);
        b.iter(|| Maze::parse_input(&input));
    }

    #[bench]
    fn profile_path_finding(b: &mut test::Bencher) {
        let input = advent_of_code::template::read_file("inputs", DAY);
        let maze = Maze::parse_input(&input);
        b.iter(|| maze.find_path().next());
    }

    #[bench]
    fn profile_connected_cells(b: &mut test::Bencher) {
        let input = advent_of_code::template::read_file("inputs", DAY);
        let maze = Maze::parse_input(&input);
        let cursor = UCoords::new(90, 86);
        b.iter(|| maze.connected_cells(cursor).last());
    }

    #[bench]
    fn profile_enclosed_area(b: &mut test::Bencher) {
        let input = advent_of_code::template::read_file("inputs", DAY);
        let path: Vec<_> = Maze::parse_input(&input).find_path().collect();
        b.iter(|| enclosed_area(path.iter().copied()));
    }
}
