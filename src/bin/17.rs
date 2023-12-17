#![feature(test)]

extern crate test;

advent_of_code::solution!(17);

use std::{collections::HashSet, ops::Range};

use advent_of_code::tools::*;
use itertools::Itertools;

const CRUCIBLE_RANGE: Range<u8> = 0..4;
const ULTRA_CRUCIBLE_RANGE: Range<u8> = 4..11;

struct City {
    blocks: Vec<u8>,
    size: UCoords,
}

#[derive(Clone)]
struct Branch {
    direction: Coords,
    length: u8,
    loss: u32,
    position: UCoords,
}

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

    let mut visited = HashSet::new();
    let end = UCoords::new(city.size.x - 1, city.size.y - 1);

    // Stores a sorted list of branches to explore
    let mut branches: Vec<Branch> = city
        .starting_vectors()
        .sorted_by_key(|b| b.loss)
        .rev()
        .collect();

    while let Some(branch) = branches.pop() {
        for new_branch in branch.next_branches(&city, &turn_range) {
            if new_branch.position == end {
                if new_branch.length >= turn_range.start {
                    return Some(new_branch.loss);
                }

                continue;
            }

            // If the branch hasn't been visited, insert it in the right place
            if visited.insert(new_branch.id()) {
                let index = branches
                    .iter()
                    .enumerate()
                    .rev()
                    .find(|(_, branch)| branch.loss >= new_branch.loss)
                    .map(|(i, _)| i)
                    .unwrap_or(0);

                branches.insert(index, new_branch);
            }
        }
    }

    panic!("No solution found!");
}

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
        let straight_branch =
            (self.length + 1 < turn_range.end)
                .then_some(())
                .and_then(|_| unsafe {
                    let position = self.next_position(self.direction, city)?;
                    let loss = self.loss + city.get(&position).unwrap_unchecked() as u32;

                    Some(Branch {
                        length: self.length + 1,
                        direction: self.direction,
                        position,
                        loss,
                    })
                });

        let Coords { x, y } = self.direction;

        let lateral_branches = (self.length >= turn_range.start)
            .then_some(())
            .into_iter()
            .flat_map(move |_| unsafe {
                [Coords::new(y, x), Coords::new(-y, -x)]
                    .into_iter()
                    .flat_map(|offset| {
                        let position = self.next_position(offset, city)?;
                        let loss = self.loss + city.get(&position).unwrap_unchecked() as u32;

                        Some(Branch {
                            length: 1,
                            direction: offset,
                            position,
                            loss,
                        })
                    })
            });

        lateral_branches.into_iter().chain(straight_branch)
    }

    fn next_position(&self, offset: Coords, city: &City) -> Option<UCoords> {
        (Coords::from(self.position) + offset).ucoords(&city.size)
    }

    /// Fits all useful information to identify the branch in a single u64.
    fn id(&self) -> u64 {
        let mut id = 0;

        id |= ((self.direction.x + 1) as u64) << 62;
        id |= ((self.direction.y + 1) as u64) << 60;
        id |= (self.length as u64) << 32;
        id |= (self.position.x as u64) << 16;
        id |= self.position.y as u64;

        id
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

    #[bench]
    fn profile_exploration(b: &mut test::Bencher) {
        let input = read_input(DAY);
        let city = parse_input(&input);

        let mut visited = HashSet::new();

        let saved_branches: Vec<Branch> = city
            .starting_vectors()
            .sorted_by_key(|b| b.loss)
            .rev()
            .collect();

        let mut branches = saved_branches.clone();

        let turn_range = ULTRA_CRUCIBLE_RANGE;

        b.iter(|| {
            if branches.is_empty() {
                branches = saved_branches.clone();
            }

            let branch = branches.pop().unwrap();

            for new_branch in branch.next_branches(&city, &turn_range) {
                if visited.insert(new_branch.id()) {
                    let index = branches
                        .iter()
                        .enumerate()
                        .rev()
                        .find(|(_, branch)| branch.loss >= new_branch.loss)
                        .map(|(i, _)| i)
                        .unwrap_or(0);

                    branches.insert(index, new_branch);
                }
            }
        })
    }
}
