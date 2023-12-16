advent_of_code::solution!(16);

use std::{
    collections::BTreeSet,
    hash::{DefaultHasher, Hash, Hasher},
    mem,
};

use advent_of_code::tools::*;

/* == Definitions == */

struct Map {
    size: UCoords,
    tiles: Vec<Tile>,
}

struct Tile {
    contents: TileType,
    energised: bool,
}

enum TileType {
    Empty,
    Splitter(SplitterOrientation),
    Mirror(MirrorOrientation),
}

enum SplitterOrientation {
    Vertical,
    Horizontal,
}

#[derive(PartialEq)]
enum MirrorOrientation {
    NorthEast,
    NorthWest,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Beam {
    origin: UCoords,
    direction: Coords,
}

/* == Solutions == */

pub fn part_one(input: &str) -> Option<u32> {
    let result = solve_beam(
        &mut Map::parse_str(input),
        Beam {
            origin: UCoords::new(0, 0),
            direction: Coords::new(1, 0),
        },
    );

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = Map::parse_str(input);

    starting_beams(&map.size.clone())
        .map(|b| {
            map.reset();
            solve_beam(&mut map, b)
        })
        .max()
}

fn solve_beam(map: &mut Map, beam: Beam) -> u32 {
    let mut cache = BTreeSet::<u64>::new();

    let mut beams = vec![beam];

    while let Some(mut beam) = beams.pop() {
        let hash = beam.hash();

        if cache.contains(&hash) {
            continue;
        } else {
            cache.insert(hash);
        }

        map.get_mut(&beam.origin).unwrap().energised = true;

        let new_beam = beam.process_tile(map.get(&beam.origin).unwrap());

        if beam.move_beam(&map.size) {
            beams.push(beam);
        }

        if let Some(mut new_beam) = new_beam {
            if new_beam.move_beam(&map.size) {
                beams.push(new_beam);
            }
        }
    }

    map.tiles.iter().filter(|t| t.energised).count() as u32
}

/* == Implementations == */

impl Map {
    fn parse_str(input: &str) -> Map {
        let mut height = 0;

        let tiles: Vec<Tile> = input
            .lines()
            .flat_map(|line| {
                height += 1;
                line.bytes().map(|b| b.into())
            })
            .collect();

        Map {
            size: UCoords::new(tiles.len() / height, height),
            tiles,
        }
    }

    fn get(&self, coords: &UCoords) -> Option<&Tile> {
        let index = coords.y * self.size.x + coords.x;
        self.tiles.get(index)
    }

    fn get_mut(&mut self, coords: &UCoords) -> Option<&mut Tile> {
        let index = coords.y * self.size.x + coords.x;
        self.tiles.get_mut(index)
    }

    fn reset(&mut self) {
        for tile in self.tiles.iter_mut() {
            tile.energised = false;
        }
    }
}

impl Beam {
    /// Processes the tile at the beam's origin, mutating the beam's
    /// direction if necessary, also returning an optional new secondary
    /// beam if a beam splitter is encountered that should also be processed.
    fn process_tile(&mut self, tile: &Tile) -> Option<Beam> {
        match &tile.contents {
            TileType::Empty => None,

            TileType::Mirror(orientation) => {
                mem::swap(&mut self.direction.x, &mut self.direction.y);

                if *orientation == MirrorOrientation::NorthEast {
                    self.direction.x *= -1;
                    self.direction.y *= -1;
                }

                None
            }

            TileType::Splitter(direction) if self.should_split(direction) => {
                mem::swap(&mut self.direction.x, &mut self.direction.y);
                let mut new_beam = *self;

                new_beam.direction.x *= -1;
                new_beam.direction.y *= -1;

                Some(new_beam)
            }

            _ => None,
        }
    }

    fn should_split(&self, tile: &SplitterOrientation) -> bool {
        match tile {
            SplitterOrientation::Vertical => self.direction.x != 0,
            SplitterOrientation::Horizontal => self.direction.y != 0,
        }
    }

    /// Moves the beam one step in the current direction, returning
    /// true if the beam is still in the map bounds.
    fn move_beam(&mut self, bounds: &UCoords) -> bool {
        let new_coords = Coords::from(self.origin) + self.direction;

        match new_coords.ucoords(bounds) {
            Some(new_ucoords) => {
                self.origin = new_ucoords;
                true
            }
            None => false,
        }
    }

    fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::default();
        self.origin.hash(&mut hasher);
        self.direction.hash(&mut hasher);
        hasher.finish()
    }
}

/* == Functions == */

/// Returns an iterator for all possible beam starting positions at the
/// edges of the map.
fn starting_beams(size: &UCoords) -> impl Iterator<Item = Beam> + '_ {
    let top_row = (0..size.x).map(|x| ((x, 0), (0, 1)));
    let bottom_row = (0..size.x).map(|x| ((x, size.y - 1), (0, -1)));
    let left_column = (0..size.y).map(|y| ((0, y), (1, 0)));
    let right_column = (0..size.y).map(|y| ((size.x - 1, y), (-1, 0)));

    top_row
        .chain(bottom_row)
        .chain(left_column)
        .chain(right_column)
        .map(|(origin, direction)| Beam {
            origin: origin.into(),
            direction: direction.into(),
        })
}

/* == Trait implementations == */

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        let contents = match value {
            b'.' => TileType::Empty,
            b'/' => TileType::Mirror(MirrorOrientation::NorthEast),
            b'\\' => TileType::Mirror(MirrorOrientation::NorthWest),
            b'-' => TileType::Splitter(SplitterOrientation::Horizontal),
            b'|' => TileType::Splitter(SplitterOrientation::Vertical),
            _ => panic!(),
        };

        Tile {
            contents,
            energised: false,
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
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example(DAY));
        assert_eq!(result, Some(51));
    }
}
