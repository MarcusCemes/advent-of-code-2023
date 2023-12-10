advent_of_code::solution!(10);

use std::fmt::Debug;

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

/// The distance to the farthest cell is half the length of the path.
pub fn part_one(input: &str) -> Option<u32> {
    let path = find_path(input);
    Some(path.len() as u32 / 2)
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
    let path = find_path(input);
    let area = enclosed_area(&path);
    Some(area - path.len() as u32 / 2 + 1)
}

/// Follows the path of the maze, starting from the start cell.
/// Returns the path as a vector of coordinates, including the start cell.
fn find_path(input: &str) -> Vec<UCoords> {
    let maze = Maze::parse_input(input);

    let mut path = vec![maze.start];

    let mut cursor = maze.start;
    let mut last_cursor = None;

    loop {
        // The connected cells iterator *should* yield two items,
        // choose the one that we haven't just visited.
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

    path
}

/* == Implementations == */

impl Maze {
    fn parse_input(input: &str) -> Maze {
        // Rough estimation of the required capacity (overshoots a little)
        let mut cells = Vec::with_capacity(input.len());

        let mut height = 0;
        let mut start = None;

        // Stream the input, constructing the matrix
        for (i, line) in input.lines().enumerate() {
            for (j, byte) in line.bytes().enumerate() {
                if byte == b'S' {
                    start = Some(UCoords::new(j, i));
                }

                cells.push(byte.into());
            }

            height += 1;
        }

        // The width and height are now known (the maze is square)
        let width = cells.len() / height;
        let size = UCoords::new(width, height);

        // The start is guaranteed to be somewhere, once
        let start = start.unwrap();

        let mut maze = Maze { cells, size, start };

        // Now that we know the position of the starting cell,
        // we can patch its value to make a continuous path.
        // Find the two adjacent cells that point to it, and use
        // the offsets to determine the connection type.
        let (a, b) = maze
            .connecting_cells(start)
            .map(|ucoords| Coords::from(ucoords) - Coords::from(start))
            .collect_tuple()
            .unwrap();

        maze.set(start, Connection::from_offsets(&[a, b]));
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

    /// Returns the cells that are connected to the current cell, based on the symbol
    /// of *adjacent* cells. This is the inverse of `connected_cells()`, and is used
    /// to compute unknown connection types.
    fn connecting_cells(&self, to: UCoords) -> impl Iterator<Item = UCoords> + '_ {
        let from_coords = Coords::from(to);

        // Iterate over adjacent cells that are connections, compute their connection
        // offsets and find the one that leads back to the current cell.
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

    /// Returns the cells that the current cell, based on the symbol of the *current* cell.
    fn connected_cells(&self, at: UCoords) -> impl Iterator<Item = UCoords> + '_ {
        let from_coords = Coords::from(at);

        self.get(at)
            .into_iter()
            .flat_map(|conn| conn.offsets())
            .flatten()
            .map(move |o| from_coords + o)
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
    /// Determine the connection type, based on the offsets of the two adjacent cells.
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

    /// Returns the offsets of a given connection type.
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
/// using the shoelace formula.
fn enclosed_area(path: &[UCoords]) -> u32 {
    path.iter()
        .chain(path.iter().take(1))
        .tuple_windows()
        .map(|(a, b)| determinant((*a).into(), (*b).into()))
        .sum::<i64>()
        .abs() as u32
        / 2
}

/// Computes the determinant of two 2D column vectors in a matrix.
fn determinant(a: Coords, b: Coords) -> i64 {
    a.x * b.y - a.y * b.x
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
}
