advent_of_code::solution!(9);

/* == Definitions == */

enum Order {
    Normal,
    Reverse,
}

/* == Solutions == */

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, Order::Normal)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, Order::Reverse)
}

/* == Input parsing == */

fn parse_input(input: &str, order: Order) -> impl Iterator<Item = Vec<i32>> + '_ {
    input.lines().map(move |line| {
        let parsed_line = line.split_whitespace().map(|s| s.parse().unwrap());

        match order {
            Order::Normal => parsed_line.collect(),
            Order::Reverse => parsed_line.rev().collect(),
        }
    })
}

/* == Functions == */

fn solve(input: &str, order: Order) -> Option<u32> {
    let result: i32 = parse_input(input, order)
        .map(|mut v| extrapolate(&mut v))
        .sum();

    Some(result as u32)
}

/// Extrapolate sequence by iteratively mutating slices of it
/// in-place until the last slice is all zeroes. Returns the next
/// predicted value in the sequence.
fn extrapolate(sequence: &mut [i32]) -> i32 {
    let end = sequence.len();
    let mut accumulator = 0;

    for i in 1..end {
        accumulator += sequence[end - 1];

        for j in (i..end).rev() {
            sequence[j] -= sequence[j - 1];
        }

        if sequence[i..end].iter().all(|&n| n == 0) {
            return accumulator;
        }
    }

    unreachable!();
}

/* == Tests == */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
