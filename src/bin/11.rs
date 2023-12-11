advent_of_code::solution!(11);

use advent_of_code::tools::*;
use itertools::Itertools;

/* == Definitions == */

struct Map {
    empty_cols: Vec<bool>,
    empty_rows: Vec<bool>,
    galaxies: Vec<UCoords>,
}

/* == Solutions == */

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, 2))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, 1_000_000))
}

fn solve(input: &str, expansion_coefficient: u64) -> u64 {
    Map::from_str(input)
        .distances_l1(expansion_coefficient)
        .sum()
}

/* == Implementations == */

impl Map {
    /// Parse the input, collecting information about galaxy locations and empty
    /// rows and columns in a single pass. The `empty_rows` and `empty_cols` vectors
    /// have the same size as the input, containing a boolean of whether than row or
    /// column is empty.
    fn from_str(input: &str) -> Map {
        let row_size = input.lines().next().unwrap().len();

        // Store a vector of booleans indicating whether each column/row is empty.
        // This provides O(1) lookup using a range, rather than O(log n).
        let mut empty_cols = vec![true; row_size];
        let mut empty_rows = Vec::new();
        let mut galaxies = Vec::new();

        for (y, line) in input.lines().enumerate() {
            let mut row_empty = true;

            let zipped_iterator = line.bytes().enumerate().zip(empty_cols.iter_mut());

            for ((x, symbol), empty_column) in zipped_iterator {
                if symbol == b'#' {
                    row_empty = false;
                    *empty_column = false;
                    galaxies.push(UCoords::new(x, y));
                }
            }

            empty_rows.push(row_empty);
        }

        Map {
            empty_cols,
            empty_rows,
            galaxies,
        }
    }

    /// Returns an iterator of distances between all pairs of galaxies using the L1
    /// norm (Manhattan distance), applying a universe expansion penalty based on the
    /// provided coefficient (a coefficient of 2 will double the distance of each empty
    /// row and column between the galaxies).
    fn distances_l1(&self, expansion_coefficient: u64) -> impl Iterator<Item = u64> + '_ {
        self.galaxy_pairs_iter().map(move |(a, b)| {
            let col_span = a.x.min(b.x)..a.x.max(b.x);
            let row_span = a.y.min(b.y)..a.y.max(b.y);

            let empty_rows = self.empty_rows[row_span].iter().filter(|&&y| y).count();
            let empty_columns = self.empty_cols[col_span].iter().filter(|&&x| x).count();
            let penalty = (expansion_coefficient - 1) * (empty_rows + empty_columns) as u64;

            (Coords::from(a) - Coords::from(b)).norm_l1() + penalty
        })
    }

    /// Returns an iterator of all pairs of galaxies.
    fn galaxy_pairs_iter(&self) -> impl Iterator<Item = (UCoords, UCoords)> + '_ {
        self.galaxies
            .iter()
            .dropping_back(1)
            .enumerate()
            .flat_map(move |(i, a)| self.galaxies[i + 1..].iter().map(move |b| (*a, *b)))
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
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_expansion_10() {
        let input = &read_example(DAY);
        assert_eq!(solve(input, 10), 1030);
    }

    #[test]
    fn test_expansion_100() {
        let input = &read_example(DAY);
        assert_eq!(solve(input, 100), 8410);
    }
}
