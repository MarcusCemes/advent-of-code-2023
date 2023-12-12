advent_of_code::solution!(12);

use std::iter::repeat;

use itertools::Itertools;

#[derive(Copy, Clone, PartialEq, Eq)]
enum SpringState {
    Working,
    Broken,
    Unknown,
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut springs = Vec::new();
    let mut counts = Vec::new();

    let result = input
        .lines()
        .map(|line| {
            let (spring_it, count_it) = parse_line(line);

            springs.truncate(0);
            springs.extend(spring_it);

            counts.truncate(0);
            counts.extend(count_it);

            valid_spring_configurations(&mut springs, &counts)
        })
        .sum();

    Some(result)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

fn parse_line(
    line: &str,
) -> (
    impl Iterator<Item = SpringState> + '_,
    impl Iterator<Item = u8> + '_,
) {
    let (spring_str, count_str) = line.split_once(' ').unwrap();

    let springs = spring_str.bytes().map(|b| SpringState::from(b));
    let counts = count_str.split(',').map(|s| s.parse().unwrap());

    (springs, counts)
}

fn valid_spring_configurations(springs: &mut [SpringState], counts: &[u8]) -> u64 {
    let unknown_indices = springs
        .iter()
        .enumerate()
        .filter(|(_, &s)| s == SpringState::Unknown)
        .map(|(i, _)| i)
        .collect_vec();

    state_variations(unknown_indices.len())
        .filter(|combinations| {
            for (&value, &index) in combinations.iter().zip_eq(unknown_indices.iter()) {
                springs[index] = value;
            }

            is_valid_state(&springs, &counts)
        })
        .count() as u64
}

fn state_variations(length: usize) -> impl Iterator<Item = Vec<SpringState>> {
    repeat([SpringState::Working, SpringState::Broken])
        .take(length)
        .multi_cartesian_product()
}

fn is_valid_state(springs: &[SpringState], counts: &[u8]) -> bool {
    springs
        .iter()
        .group_by(|&&state| state == SpringState::Broken)
        .into_iter()
        .filter_map(|(key, group)| key.then_some(group))
        .zip_longest(counts.iter())
        .all(|x| {
            x.both()
                .map(|(group, &count)| group.count() == count as usize)
                .unwrap_or(false)
        })
}

impl From<u8> for SpringState {
    fn from(value: u8) -> Self {
        match value {
            b'.' => SpringState::Working,
            b'#' => SpringState::Broken,
            b'?' => SpringState::Unknown,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const W: SpringState = SpringState::Working;
    const B: SpringState = SpringState::Broken;

    #[test]
    fn test_process_line_1() {
        let (spring_it, count_it) = parse_line(".??..??...?##. 1,1,3");
        let mut springs: Vec<_> = spring_it.collect();
        let counts: Vec<_> = count_it.collect();
        let result = valid_spring_configurations(&mut springs, &counts);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_process_line_2() {
        let (spring_it, count_it) = parse_line("?###???????? 3,2,1");
        let mut springs: Vec<_> = spring_it.collect();
        let counts: Vec<_> = count_it.collect();
        let result = valid_spring_configurations(&mut springs, &counts);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_valid_state() {
        assert!(is_valid_state(
            &[W, B, W, W, W, B, W, W, W, W, W, B, B, B],
            &[1, 1, 3]
        ));

        assert!(!is_valid_state(
            &[W, B, B, W, W, B, B, W, W, W, W, B, B, B],
            &[1, 1, 3]
        ));

        assert!(!is_valid_state(
            &[W, W, W, W, W, B, W, W, W, W, W, B, B, B],
            &[1, 1, 3]
        ));
    }

    #[test]
    fn test_state_variations() {
        let result = state_variations(8);
        assert_eq!(result.count(), 256);
    }
}
