advent_of_code::solution!(4);

const SIZE_HINT: usize = 200;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| count_score(winning_count(line)))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut extra_counts: Vec<u32> = vec![0; SIZE_HINT];
    let mut count = 0;

    for (i, line) in input.lines().enumerate() {
        let instances = extra_counts.get(i).unwrap_or(&0) + 1;
        let winners = winning_count(line);

        // Removed dynamic resizing in favour of fixed SIZE_HINT
        // let new_size = extra_counts.len().max(i + winners) + 1;
        // extra_counts.resize(new_size, 0);

        for j in 1..=winners {
            extra_counts[i + j] += instances;
        }

        count += instances;
    }

    Some(count)
}

fn winning_count(line: &str) -> usize {
    let (_, rest) = line.split_once(": ").unwrap();
    let (target, cards) = rest.split_once(" | ").unwrap();

    let targets: Vec<u8> = target
        .split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    cards
        .split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .filter(|n| targets.contains(n))
        .count()
}

fn count_score(count: usize) -> u32 {
    match count {
        0 => 0,
        n => 1 << (n - 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::template::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example(DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example(DAY));
        assert_eq!(result, Some(30));
    }
}
