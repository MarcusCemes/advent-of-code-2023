advent_of_code::solution!(14);

use std::{
    collections::{BTreeMap, BTreeSet},
    hash::{DefaultHasher, Hash, Hasher},
};

use advent_of_code::tools::{Coords, UCoords};

/* == Definitions == */

const SPIN_CYCLES: usize = 1_000_000_000;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Occupation {
    Empty,
    Fixed,
    Rolling,
}

enum Direction {
    East,
    North,
    South,
    West,
}

struct Platform {
    occupation: Vec<Occupation>,
    size: UCoords,
}

/* == Solutions == */

pub fn part_one(input: &str) -> Option<u64> {
    let mut platform = Platform::parse_str(input);
    let load = platform.tilt_platform(Direction::North);
    Some(load)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut platform = Platform::parse_str(input);

    let mut hashes = BTreeMap::new();
    let mut load = 0;

    // Keep spinning the cycle and saving the load
    for i in 0..SPIN_CYCLES {
        load = platform.spin_platform();
        let hash = platform.hash();

        // Thankfully, a cycle is always found before the end...
        // match history.iter().enumerate().find(|(_, (h, _))| *h == hash) {
        match hashes.get(&hash) {
            None => {
                hashes.insert(hash, (i, platform.north_beam_load()));
            }

            Some((j, _)) => {
                // Compute the expected load after SPIN_CYCLES by exploiting
                // the cyclic nature and the saved loads after each cycle.
                let index = j - 1 + (SPIN_CYCLES - i) % (i - j);

                load = *hashes
                    .values()
                    .find(|(i, _)| *i == index)
                    .map(|(_, l)| l)
                    .unwrap();

                break;
            }
        }
    }

    Some(load)
}

/* == Implementations == */

impl Platform {
    fn parse_str(input: &str) -> Platform {
        let mut occupation = Vec::new();
        let mut height = 0;

        for line in input.lines() {
            occupation.extend(line.bytes().map(Occupation::from));
            height += 1;
        }

        let width = occupation.len() / height;

        Platform {
            occupation,
            size: UCoords::new(width, height),
        }
    }

    /// Tilts the platform North, West, South and East, returning the
    /// final load north beam load.
    fn spin_platform(&mut self) -> u64 {
        self.tilt_platform(Direction::North);
        self.tilt_platform(Direction::West);
        self.tilt_platform(Direction::South);
        self.tilt_platform(Direction::East)
    }

    /// Tilts the platform in the given direction, returning the load
    /// of the north beam.
    fn tilt_platform(&mut self, direction: Direction) -> u64 {
        // Not really sure how to do this with less code, without macros
        // or dynamic dispatching/allocation. This works fine.
        match direction {
            Direction::East => (0..self.size.y)
                .map(|y| {
                    let minimum = UCoords::new(self.size.x - 1, y).into();
                    let gradient = Coords::new(-1, 0);
                    self.process_line(minimum, gradient)
                })
                .sum(),

            Direction::North => (0..self.size.x)
                .map(|x| {
                    let minimum = UCoords::new(x, 0).into();
                    let gradient = Coords::new(0, 1);
                    self.process_line(minimum, gradient)
                })
                .sum(),

            Direction::South => (0..self.size.x)
                .map(|x| {
                    let minimum = UCoords::new(x, self.size.y - 1).into();
                    let gradient = Coords::new(0, -1);
                    self.process_line(minimum, gradient)
                })
                .sum(),

            Direction::West => (0..self.size.y)
                .map(|y| {
                    let minimum = UCoords::new(0, y).into();
                    let gradient = Coords::new(1, 0);
                    self.process_line(minimum, gradient)
                })
                .sum(),
        }
    }

    /// Shifts all rolling stones in a single line of the platform.
    /// The minimum should be the coordinates at the bottom of the slope,
    /// with the gradient being a unit vector pointing up the slope.
    ///
    /// This algorithm works by iterating over each free spot, peeking ahead
    /// to find the next rolling stone and swapping them. Encountering fixed
    /// objects during the peek will jump the search to the spot after it.
    ///
    /// Efficiently collects and returns the final load of the line, **computed
    /// in the direction of the gradient** (not always the north beam!).
    fn process_line(&mut self, minimum: Coords, gradient: Coords) -> u64 {
        let mut at = minimum;

        // Determine the distance based on platform size (should be square anyway...)
        let mut distance = match gradient {
            Coords { x: 0, y: _ } => self.size.y,
            Coords { x: _, y: 0 } => self.size.x,
            _ => unreachable!(),
        } as u64;

        // Collect the load during the pass
        let mut load = 0;

        'outer: loop {
            // Check the current position
            match at.ucoords(&self.size).and_then(|x| Some((x, self.get(x)?))) {
                Some((at_ucoords, Occupation::Empty)) => {
                    // If it's empty, search for the nearest rolling stone

                    let mut cursor = at + gradient;

                    loop {
                        match cursor
                            .ucoords(&self.size)
                            .and_then(|x| Some((x, self.get(x)?)))
                        {
                            // Rolling stone found, roll it to current position
                            Some((cursor_ucoords, Occupation::Rolling)) => {
                                self.set(cursor_ucoords, Occupation::Empty);
                                self.set(at_ucoords, Occupation::Rolling);
                                load += distance;
                                break;
                            }

                            // Next object is static, move search to after it
                            Some((_, Occupation::Fixed)) => {
                                distance -= (cursor - at).norm_l1();
                                at = cursor;
                                break;
                            }

                            // Keep going...
                            Some((_, Occupation::Empty)) => (),

                            // Cursor left the map, we're done here
                            None => break 'outer,
                        }

                        cursor += gradient;
                    }
                }

                // We're on a rolling stone, add it to the load
                Some((_, Occupation::Rolling)) => load += distance,

                // Immovable object
                Some((_, Occupation::Fixed)) => (),

                // Reached the end, all done
                None => break 'outer,
            }

            distance -= 1;
            at += gradient;
        }

        load
    }

    /// Computes the load of the north beam, by multiplying each rolling stone by
    /// the distance to south end of the platform.
    fn north_beam_load(&self) -> u64 {
        (0..self.size.y)
            .map(|y| {
                let distance = self.size.y - y;

                distance
                    * self.occupation[y * self.size.x..(y + 1) * self.size.x]
                        .iter()
                        .filter(|&&x| x == Occupation::Rolling)
                        .count()
            })
            .sum::<usize>() as u64
    }

    fn get(&self, coords: UCoords) -> Option<Occupation> {
        let index = coords.y * self.size.x + coords.x;
        self.occupation.get(index).copied()
    }

    fn set(&mut self, coords: UCoords, value: Occupation) {
        let index = coords.y * self.size.x + coords.x;
        self.occupation[index] = value;
    }

    /// Computes a hash of the platform's current state using the default algorithm.
    fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::default();
        self.occupation.hash(&mut hasher);
        hasher.finish()
    }
}

impl From<u8> for Occupation {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Occupation::Empty,
            b'#' => Occupation::Fixed,
            b'O' => Occupation::Rolling,
            _ => unreachable!(),
        }
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
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example(DAY));
        assert_eq!(result, Some(64));
    }
}
