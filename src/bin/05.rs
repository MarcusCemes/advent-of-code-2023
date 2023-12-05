use std::{fmt::Debug, ops::Range};

use itertools::Itertools;

advent_of_code::solution!(5);

struct Stage {
    map: Vec<Mapping>,
}

#[derive(Debug)]
struct Mapping {
    range: Range<u64>,
    offset: i64,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let mut seeds: Vec<u64> = read_seeds(lines.next().unwrap()).collect();
    let stages = parse_stages(lines.skip(1));

    for stage in stages {
        map_seeds(&mut seeds, &stage);
    }

    Some(seeds.into_iter().min().unwrap() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let mut seeds: Vec<Range<u64>> = group_seeds(read_seeds(lines.next().unwrap()));
    let stages = parse_stages(lines.skip(1));

    for stage in stages {
        let mut extra_ranges = Vec::new();

        for seed_range in &mut seeds {
            let mut new_ranges = map_seed_range(seed_range, &stage);
            *seed_range = new_ranges.pop().unwrap();

            extra_ranges.extend_from_slice(&new_ranges);
        }

        seeds.extend_from_slice(&extra_ranges);
        extra_ranges.clear();
    }

    Some(seeds.into_iter().map(|r| r.start).min().unwrap() as u32)
}

fn read_seeds(line: &str) -> impl Iterator<Item = u64> + '_ {
    let (_, seed_str) = line.split_at(7);
    seed_str.split(' ').map(|s| s.parse().unwrap())
}

fn group_seeds(seeds: impl Iterator<Item = u64>) -> Vec<Range<u64>> {
    seeds
        .chunks(2)
        .into_iter()
        .map(|mut chunk| {
            let start = chunk.next().unwrap();
            let size = chunk.next().unwrap();
            start..start + size
        })
        .collect()
}

fn parse_stages<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<Stage> {
    lines
        .group_by(|s| !s.is_empty())
        .into_iter()
        .filter_map(|(k, v)| k.then_some(v))
        .map(|mut group| {
            let _ = parse_header(group.next().unwrap()).unwrap();
            let map = group.map(parse_map).collect::<Option<_>>().unwrap();
            Stage { map }
        })
        .collect()
}

fn parse_header(line: &str) -> Option<(&str, &str)> {
    let (from_to, _) = line.split_once(' ')?;
    from_to.split_once("-to-")
}

fn parse_map(line: &str) -> Option<Mapping> {
    let (to_str, rest) = line.split_once(' ')?;
    let (from_str, size_str) = rest.split_once(' ')?;

    let from: u64 = from_str.parse().unwrap();
    let to: u64 = to_str.parse().unwrap();
    let size: u64 = size_str.parse().unwrap();

    Some(Mapping {
        range: from..from + size,
        offset: to as i64 - from as i64,
    })
}

fn map_seeds(seeds: &mut [u64], stage: &Stage) {
    for seed in seeds {
        if let Some(mapping) = stage
            .map
            .iter()
            .find(|mapping| mapping.range.contains(seed))
        {
            *seed = seed.checked_add_signed(mapping.offset).unwrap();
        }
    }
}

fn map_seed_range(seed: &Range<u64>, stage: &Stage) -> Vec<Range<u64>> {
    let mut cursor = seed.start;
    let mut new_ranges = Vec::new();

    while cursor != seed.end {
        match stage
            .map
            .iter()
            .find(|mapping| mapping.range.contains(&cursor))
        {
            Some(mapping) => {
                let end = mapping.range.end.min(seed.end);

                let new_start = cursor.checked_add_signed(mapping.offset).unwrap();
                let new_end = end.checked_add_signed(mapping.offset).unwrap();

                assert!(end > cursor);
                new_ranges.push(new_start..new_end);
                cursor = end;
            }
            None => {
                let end = stage
                    .map
                    .iter()
                    .filter(|mapping| mapping.range.start > cursor)
                    .map(|mapping| mapping.range.start)
                    .min()
                    .unwrap_or(seed.end)
                    .min(seed.end);

                assert!(end > cursor);
                new_ranges.push(cursor..end);
                cursor = end;
            }
        }

        let last = new_ranges.last().unwrap();
        if last.start >= last.end {
            panic!("Just added invalid range: {:?}", last);
        }
    }

    new_ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
