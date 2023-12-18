use std::iter;

use itertools::Itertools;

use super::Coords;

/// Computes the enclosed area of a polygon, given its vertices,
/// using the shoelace formula (fast determinant-based version).
pub fn enclosed_area(mut path: impl Iterator<Item = Coords>) -> u64 {
    let first = path.next().unwrap_or_default();

    let sum: i64 = iter::once(first)
        .chain(path)
        .chain(iter::once(first))
        .map(Coords::from)
        .tuple_windows()
        .map(|(a, b)| a.x * b.y - b.x * a.y)
        .sum();

    sum.unsigned_abs() / 2
}

#[cfg(test)]
mod tests {
    use super::enclosed_area;
    use crate::tools::*;

    #[test]
    fn test_enclosed_area() {
        let path = [(0, 0), (0, 2), (2, 2), (2, 0)]
            .iter()
            .map(|&(x, y)| Coords::new(x, y));

        let area = enclosed_area(path);
        assert_eq!(area, 4);
    }
}
