use std::ops::Range;

use itertools::Itertools;

advent_of_code::solution!(5);

/* == Definitions == */

struct Stage(Vec<Mapping>);

struct Mapping {
    range: Range<u64>,
    offset: i64,
}

struct MappedRangeIterator<'a> {
    seed: &'a Range<u64>,
    stage: &'a Stage,
    cursor: u64,
}

/* == Solutions == */

pub fn part_one(input: &str) -> Option<u32> {
    let (mut seeds, stages) = parse_input_seeds(input);

    for stage in stages {
        for seed in &mut seeds {
            *seed = map_seed(*seed, &stage);
        }
    }

    Some(seeds.into_iter().min().unwrap() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut seeds, stages) = parse_input_seed_ranges(input);
    let mut new_ranges = Vec::new();

    for stage in stages {
        for seed_range in &mut seeds {
            // Temporary reusable buffer required as seed_range is borrowed by the loop
            new_ranges.extend(MappedRangeIterator::new(seed_range, &stage));

            // Take one of the new ranges and overwrite the old range, this will
            // overwrite all old seed ranges by time the loop finishes
            *seed_range = new_ranges.pop().unwrap();
        }

        seeds.extend_from_slice(&new_ranges);
        new_ranges.clear();
    }

    Some(seeds.into_iter().map(|r| r.start).min().unwrap() as u32)
}

/* == Parsing == */

fn parse_input_seeds(input: &str) -> (Vec<u64>, Vec<Stage>) {
    let mut lines = input.lines();
    let seed_str = &lines.next().unwrap()[7..];
    (parse_seeds(seed_str).collect(), parse_stages(lines))
}

fn parse_input_seed_ranges(input: &str) -> (Vec<Range<u64>>, Vec<Stage>) {
    let mut lines = input.lines();
    let seed_str = &lines.next().unwrap()[7..];
    let seeds = parse_seed_ranges(seed_str);
    (seeds, parse_stages(lines))
}

fn parse_seed_ranges(seeds: &str) -> Vec<Range<u64>> {
    parse_seeds(seeds)
        .chunks(2)
        .into_iter()
        .map(|chunk| chunk.collect_tuple().map(|(a, b)| a..a + b).unwrap())
        .collect()
}

fn parse_seeds(seeds: &str) -> impl Iterator<Item = u64> + '_ {
    seeds.split(' ').map(|s| s.parse().unwrap())
}

fn parse_stages<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<Stage> {
    lines
        .group_by(|s| !s.is_empty())
        .into_iter()
        .filter_map(|(k, v)| k.then_some(v))
        .map(|group| Stage(group.skip(1).map(Mapping::parse_str).collect()))
        .collect()
}

/* == Functions ==  */

fn map_seed(seed: u64, stage: &Stage) -> u64 {
    stage
        .find_map(seed)
        .map(|m| seed.wrapping_add_signed(m.offset))
        .unwrap_or(seed)
}

/* == Implementations == */

impl Mapping {
    fn parse_str(input: &str) -> Mapping {
        input
            .split(' ')
            .flat_map(|s| s.parse::<u64>())
            .collect_tuple()
            .map(|(t, f, s)| Mapping {
                range: f..f + s,
                offset: t as i64 - f as i64,
            })
            .unwrap()
    }
}

impl Stage {
    fn find_map(&self, seed: u64) -> Option<&Mapping> {
        self.0.iter().find(|mapping| mapping.range.contains(&seed))
    }

    fn find_next_map(&self, seed: u64) -> Option<&Mapping> {
        self.0
            .iter()
            .filter(|mapping| mapping.range.start > seed)
            .min_by_key(|mapping| mapping.range.start)
    }
}

impl MappedRangeIterator<'_> {
    fn new<'a>(seed: &'a Range<u64>, stage: &'a Stage) -> MappedRangeIterator<'a> {
        MappedRangeIterator {
            seed,
            stage,
            cursor: seed.start,
        }
    }
}

impl Iterator for MappedRangeIterator<'_> {
    type Item = Range<u64>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.seed.end {
            return None;
        }

        // Find the end of an intersecting mapping with its offset,
        // or the start of the next one with identity mapping (no offset).
        let (next_range, offset) = match self.stage.find_map(self.cursor) {
            Some(mapping) => (Some(mapping.range.end), mapping.offset),
            None => {
                let next = self.stage.find_next_map(self.cursor).map(|m| m.range.start);
                (next, 0)
            }
        };

        // Compute the next cursor, clamping it to the end of the seed range
        let next_cursor = next_range
            .map(|r| r.min(self.seed.end))
            .unwrap_or(self.seed.end);

        // Apply the offset to the range and return it from the iterator
        let start = self.cursor.wrapping_add_signed(offset);
        let end = next_cursor.wrapping_add_signed(offset);

        self.cursor = next_cursor;
        Some(start..end)
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
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example(DAY));
        assert_eq!(result, Some(46));
    }
}
