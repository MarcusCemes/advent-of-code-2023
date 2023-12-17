advent_of_code::solution!(17);

use std::{cmp::Ordering, collections::BinaryHeap, ops::Range};

use advent_of_code::tools::*;

/* == Definitions == */

const DIRECTIONS: usize = 4;
const CRUCIBLE_RANGE: Range<u8> = 0..4;
const ULTRA_CRUCIBLE_RANGE: Range<u8> = 4..11;

struct City {
    blocks: Vec<u8>,
    size: UCoords,
}

/// A branch is a possible path through the city, with a position, direction
/// and length. The loss is the amount of heat lost following this path.
#[derive(Clone)]
struct Branch {
    direction: Coords,
    length: u8,
    loss: u32,
    position: UCoords,
}

/// Fast storage of visited branch states in a linear array. Uses a 16
/// bit integer storing each possible length good cache locality.
/// The maximum crucible length is therefore 15.
struct VisitationMatrix {
    map: Vec<u16>,
    step: usize,
}

/* == Solutions == */

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, CRUCIBLE_RANGE)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, ULTRA_CRUCIBLE_RANGE)
}

/// Solves the problem by exploring the city with a greedy algorithm,
/// similar to the dynamic programming approach used in Day 12.
fn solve(input: &str, turn_range: Range<u8>) -> Option<u32> {
    let city = parse_input(input);
    let end = UCoords::new(city.size.x - 1, city.size.y - 1);

    let mut visit_map = VisitationMatrix::new(&city.size);
    let mut heap = BinaryHeap::from_iter(city.starting_vectors());

    while let Some(branch) = heap.pop() {
        for new_branch in branch.next_branches(&city, &turn_range) {
            if new_branch.position == end {
                if new_branch.length >= turn_range.start {
                    return Some(new_branch.loss);
                }

                continue;
            }

            if visit_map.visit(
                &new_branch.position,
                &new_branch.direction,
                new_branch.length,
            ) {
                heap.push(new_branch);
            }
        }
    }

    panic!("No solution found!");
}

/* == Input parsing == */

fn parse_input(input: &str) -> City {
    let mut height = 0;

    let blocks = input
        .lines()
        .inspect(|_| height += 1)
        .flat_map(|line| line.bytes().map(|b| b - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let size = UCoords::new(blocks.len() / height, height);
    City { blocks, size }
}

/* == Implementations == */

impl City {
    fn get(&self, coords: &UCoords) -> Option<u8> {
        let index = coords.x + coords.y * self.size.x;
        self.blocks.get(index).copied()
    }

    fn starting_vectors(&self) -> impl Iterator<Item = Branch> + '_ {
        [(0, 1), (1, 0)].into_iter().map(|(x, y)| {
            let position = UCoords::new(x as usize, y as usize);

            Branch {
                direction: Coords::new(x, y),
                position,
                length: 1,
                loss: self.get(&position).unwrap() as u32,
            }
        })
    }
}

impl Branch {
    /// Returns an iterator of next possible branches, given the city map and the
    /// state of the current branch.
    fn next_branches<'a>(
        &'a self,
        city: &'a City,
        turn_range: &Range<u8>,
    ) -> impl Iterator<Item = Branch> + 'a {
        let Coords { x, y } = self.direction;

        let straight_branch = (self.length + 1 < turn_range.end)
            .then_some(())
            .map(|_| (self.length + 1, self.direction));

        let lateral_branches = (self.length >= turn_range.start)
            .then_some(())
            .into_iter()
            .flat_map(move |_| [(y, x), (-y, -x)].map(|(x, y)| (1, Coords::new(x, y))));

        lateral_branches
            .chain(straight_branch)
            .flat_map(|(length, direction)| unsafe {
                let position = (direction + self.position.into()).ucoords(&city.size)?;
                let loss = self.loss + city.get(&position).unwrap_unchecked() as u32;

                Some(Branch {
                    length,
                    direction,
                    position,
                    loss,
                })
            })
    }
}

impl VisitationMatrix {
    fn new(size: &UCoords) -> Self {
        Self {
            map: vec![0; size.x * size.y * DIRECTIONS],
            step: size.x,
        }
    }

    fn visit(&mut self, at: &UCoords, direction: &Coords, length: u8) -> bool {
        debug_assert!(length < 16);

        let index = at.y * 4 * self.step + at.x * 4 + direction_id(direction);

        // Fetch the bit, set it and return whether it was not set before
        let mask = 1 << length;
        let bits = self.map[index];
        self.map[index] = bits | mask;

        (bits & mask) == 0
    }
}

fn direction_id(direction: &Coords) -> usize {
    match direction {
        Coords { x: 0, y: 1 } => 0,
        Coords { x: 1, y: 0 } => 1,
        Coords { x: 0, y: -1 } => 2,
        Coords { x: -1, y: 0 } => 3,
        _ => unreachable!(),
    }
}

/* == Trait implementations == */

impl PartialEq for Branch {
    fn eq(&self, other: &Self) -> bool {
        self.loss == other.loss
    }
}

impl Eq for Branch {}

impl PartialOrd for Branch {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.loss.cmp(&self.loss))
    }
}

impl Ord for Branch {
    fn cmp(&self, other: &Self) -> Ordering {
        other.loss.cmp(&self.loss)
    }
}

/* == Tests == */

/* == Tests == */
#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::template::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example(DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example(DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_ultra_crucible() {
        let result = part_two(&read_example_part(DAY, 2));
        assert_eq!(result, Some(71));
    }
}
