advent_of_code::solution!(10);

use std::fmt::{Debug, Display};

use advent_of_code::tools::*;
use itertools::Itertools;

/* == Definitions == */

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

struct Maze {
    cells: Vec<Connection>,
    size: UCoords,
    start: UCoords,
}

/* == Solutions == */

pub fn part_one(input: &str) -> Option<u32> {
    let maze = Maze::parse_input(input);

    let mut path = vec![maze.start];

    let mut cursor = maze.start;
    let mut last_cursor = None;

    loop {
        let next = maze
            .connected_cells(cursor)
            .find(|c| Some(*c) != last_cursor)
            .unwrap();

        if next == maze.start {
            break;
        }

        last_cursor = Some(cursor);
        cursor = next;

        path.push(next);
    }

    Some(path.len() as u32 / 2)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

/* == Implementations == */

impl Maze {
    fn parse_input(input: &str) -> Maze {
        let mut cells = Vec::with_capacity(input.len());

        let mut height = 0;
        let mut start = None;

        for (i, line) in input.lines().enumerate() {
            height += 1;

            for (j, byte) in line.bytes().enumerate() {
                if byte == b'S' {
                    start = Some((j, i));
                }

                cells.push(byte.into());
            }
        }

        let width = cells.len() / height;

        let size = UCoords {
            x: width,
            y: height,
        };
        let start = start.map(|(x, y)| UCoords { x, y }).unwrap();

        let mut maze = Maze { cells, size, start };

        let (a, b) = maze
            .connecting_cells(start)
            .map(|ucoords| {
                let coords = Coords::from(ucoords);

                coords - Coords::from(start)
            })
            .collect_tuple()
            .unwrap();

        let start_symb = Connection::from_offsets(&[a, b]);
        maze.set(start, start_symb);
        maze
    }

    fn get(&self, coords: UCoords) -> Option<Connection> {
        let index = coords.y * self.size.x + coords.x;
        self.cells.get(index).copied()
    }

    fn set(&mut self, coords: UCoords, connection: Connection) {
        let index = coords.y * self.size.x + coords.x;
        self.cells[index] = connection;
    }

    fn connecting_cells(&self, to: UCoords) -> impl Iterator<Item = UCoords> + '_ {
        let from_coords = Coords::from(to);

        self.adjacent_cells(to)
            .flat_map(|coords| self.get(coords).map(|conn| (coords, conn)))
            .flat_map(|(ucoords, conn)| {
                conn.offsets()
                    .map(|os| os.map(|o| (ucoords, Coords::from(ucoords) + o)))
            })
            .flatten()
            .filter(move |(_, connected_cell)| *connected_cell == from_coords)
            .map(|(coords, _)| coords)
    }

    fn connected_cells(&self, at: UCoords) -> impl Iterator<Item = UCoords> + '_ {
        let from_coords = Coords::from(at);

        self.get(at)
            .into_iter()
            .flat_map(|conn| conn.offsets())
            .flatten()
            .map(move |o| from_coords + o)
            .filter_map(|coords| coords.ucoords(&self.size))
    }

    fn adjacent_cells(&self, at: UCoords) -> impl Iterator<Item = UCoords> + '_ {
        let coords = Coords::from(at);

        (-1_i64..=1)
            .cartesian_product(-1..=1)
            .filter(|(i, j)| i != j)
            .map(move |(i, j)| coords + (i, j).into())
            .filter_map(|coords| coords.ucoords(&self.size))
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.cells.chunks(self.size.x).try_for_each(|line| {
            line.iter().try_for_each(|&c| write!(f, "{}", c))?;
            writeln!(f)
        })
    }
}

impl Connection {
    fn from_offsets(offsets: &[Coords; 2]) -> Connection {
        match offsets.iter().sorted().collect_tuple().unwrap() {
            (Coords { x: -1, y: 0 }, Coords { x: 1, y: 0 }) => Connection::Horizontal,
            (Coords { x: 0, y: -1 }, Coords { x: 1, y: 0 }) => Connection::NorthEast,
            (Coords { x: -1, y: 0 }, Coords { x: 0, y: -1 }) => Connection::NorthWest,
            (Coords { x: 0, y: 1 }, Coords { x: 1, y: 0 }) => Connection::SouthEast,
            (Coords { x: -1, y: 0 }, Coords { x: 0, y: 1 }) => Connection::SouthWest,
            (Coords { x: 0, y: -1 }, Coords { x: 0, y: 1 }) => Connection::Vertical,
            _ => Connection::None,
        }
    }

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

impl Display for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Connection::Horizontal => '-',
            Connection::NorthEast => 'L',
            Connection::NorthWest => 'J',
            Connection::SouthEast => 'F',
            Connection::SouthWest => '7',
            Connection::Vertical => '|',
            Connection::None => '.',
        };

        write!(f, "{}", c)
    }
}

/* == Tests ==  */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
