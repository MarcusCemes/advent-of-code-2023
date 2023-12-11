advent_of_code::solution!(6);

/* == Definitions == */

struct Race {
    distance: u64,
    time: u64,
}

pub fn part_one(input: &str) -> Option<u32> {
    let solution = parse_separate(input)
        .map(|game| number_winning_possibilities(&game))
        .product();

    Some(solution)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(number_winning_possibilities(&parse_joined(input)))
}

/* == Parsing == */

fn parse_separate(input: &str) -> impl Iterator<Item = Race> + '_ {
    let (distance_str, time_str) = parse_input(input);

    iter_numbers(distance_str)
        .zip(iter_numbers(time_str))
        .map(|(distance, time)| Race { time, distance })
}

fn parse_joined(input: &str) -> Race {
    let (distance_str, time_str) = parse_input(input);
    let time = joined_numbers(time_str);
    let distance = joined_numbers(distance_str);
    Race { time, distance }
}

fn parse_input(input: &str) -> (&str, &str) {
    let mut lines = input.lines().map(|l| l.split_at(9).1);
    let time_str = lines.next().unwrap();
    let distance_str = lines.next().unwrap();
    (distance_str, time_str)
}

fn iter_numbers(input: &str) -> impl Iterator<Item = u64> + '_ {
    input.split_whitespace().map(|s| s.parse().unwrap())
}

fn joined_numbers(input: &str) -> u64 {
    input
        .split_whitespace()
        .map(|s| (s.len() as u32, s.parse::<u64>().unwrap()))
        .fold(0, |acc, (s, n)| 10_u64.pow(s) * acc + n)
}

/* == Functions == */

/// Grug tried counting the number of winning possibilities by hand,
/// but he got bored after a while. He decided to write a program
/// to do it for him, but he didn't know how to solve quadratic equations.
/// So he asked his friend, who was a mathematician, to help him.
/// His friend told him that the number of winning possibilities is
/// given by the number of natural numbers x that solve the inequation
///  x * (t - x) - d > 0, where x is the time pressing the button, t is the time
/// of the race and d is the record distance. Grug was happy with this answer,
/// and he wrote a program to solve it. Computer didn't get bored, unlike Grug.
/// Grug was able to win the race.
fn number_winning_possibilities(race: &Race) -> u32 {
    let a = -1.0;
    let b = race.time as f64;
    let c = -(race.distance as f64);

    let (x1, x2) = solutions_of_quadratic_equation(a, b, c);

    let mut diff = x2.floor() - x1.ceil() - 1.0;
    diff += if x1.trunc() != x1 { 1.0 } else { 0.0 };
    diff += if x2.trunc() != x2 { 1.0 } else { 0.0 };

    diff as u32
}

/// Computes the solutions of the quadratic equation ax^2 + bx + c = 0.
/// The computation is done using f64, as f32 was not sufficient for the input.
fn solutions_of_quadratic_equation(a: f64, b: f64, c: f64) -> (f64, f64) {
    let discriminant = b * b - 4.0 * a * c;
    debug_assert!(discriminant > 0.0);
    let sqrt_discriminant = discriminant.sqrt();
    let two_a = 2.0 * a;
    let x1 = (-b + sqrt_discriminant) / two_a;
    let x2 = (-b - sqrt_discriminant) / two_a;
    (x1, x2)
}

/* == Tests == */

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::template::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example(DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example(DAY));
        assert_eq!(result, Some(71503));
    }
}
