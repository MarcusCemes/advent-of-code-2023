advent_of_code::solution!(12);

use std::iter::once;

use itertools::Itertools;

/* == Definitions == */

#[derive(Copy, Clone)]
enum Spring {
    Working,
    Broken,
    Unknown,
}

struct Branch {
    group: u8,
    length: u8,
    permutations: u64,
}

/* == Solutions == */

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, 1))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, 5))
}

fn solve(input: &str, scale: usize) -> u64 {
    input
        .lines()
        .map(|line| {
            let (springs, counts) = parse_line(line, scale);
            valid_spring_arrangement_count(&springs, &counts)
        })
        .sum()
}

/* == Input parsing == */

/// Parses an input line into the springs and groups that it represents.
/// If the scale is greater than 1, the line will be repeated that many times,
/// interspersed with an unknown spring.
fn parse_line(line: &str, scale: usize) -> (Vec<Spring>, Vec<u8>) {
    let (spring_str, count_str) = line.split_once(' ').unwrap();
    let spring_it = spring_str.bytes().map(|b| Spring::from(b));
    let count_it = count_str.split(',').map(|s| s.parse().unwrap());

    let mut springs = Vec::from_iter(spring_it);
    let mut groups = Vec::from_iter(count_it);

    if scale > 1 {
        let original_springs = springs.clone();
        let original_groups = groups.clone();

        for _ in 1..scale {
            springs.push(Spring::Unknown);
            springs.extend_from_slice(&original_springs);
            groups.extend_from_slice(&original_groups);
        }
    }

    (springs, groups)
}

/* == Functions == */

/// So... I had to look this one up. I'm not ashamed to admit it.
/// It turns out that brute-forcing this problem is not a good idea.
///
/// Instead, we can use a "dynamic programming" approach to solve it,
/// by intelligently considering a single spring and exploring the state
/// space of possible arrangements, pruning whole subtrees of possibilities
/// that violate the group constraints.
///
/// The algorithm works by considering each spring in turn, starting with a single
/// branch (defined by a `group` index and current spring `length`). For each
/// step, we iterative over all stored branches and consider whether the spring is
/// working broken.
///
/// If it's working, we complete the previous broken spring (if there is one) by
/// incrementing the `group` index (to start searching for the next group). If it's
/// broken, we extend the `length` counter, or drop the branch if it exceeds the
/// current group constraint. In the case that the spring is unknown, we do both
/// (an unknown spring effectively splits our decision tree in two, it creates two
/// valid configurations). At the end of the algorithm, we finalise each branch
/// (filter those that found all groups correctly) and sum their permutations.
///
/// To reuse computation, we can regroup branches that have the same `group` and
/// `length`. For this, we introduce a `permutations` counter which represents the
/// number of paths leading to this state (like roads rejoining, before they split again).
///
/// The result is that you can compute 850.5 trillion valid spring configurations
/// in about 15 μs. Definitely worth it. Algorithm inspired by u/KayZGames on
/// Reddit (https://redd.it/18gomx5)
fn valid_spring_arrangement_count(springs: &[Spring], groups: &[u8]) -> u64 {
    let mut previous_branches = vec![Branch::default()];
    let mut current_branches = Vec::new();

    // Adding an extra working spring as the end ensures that all branches
    // are finalised at the end by terminating any pending broken springs.
    let spring_it = springs.iter().chain(once(&Spring::Working));

    for spring in spring_it {
        for branch in &previous_branches {
            match spring {
                // In the case that it's a broken spring...
                Spring::Broken => {
                    if branch.group as usize != groups.len()
                        && branch.length < groups[branch.group as usize]
                    {
                        // ...and it's not the expected length, extend it
                        current_branches.push(Branch {
                            length: branch.length + 1,
                            ..*branch
                        })
                    }
                }

                // In the case that...
                Spring::Unknown => {
                    // ...the spring is broken...
                    if branch.group as usize != groups.len()
                        && branch.length < groups[branch.group as usize]
                    {
                        // ...and it's not the expected length, extend it
                        current_branches.push(Branch {
                            length: branch.length + 1,
                            ..*branch
                        })
                    }

                    // ...but may also be working...
                    if branch.length == 0 {
                        // ...and the previous was working, just keep going
                        current_branches.push(Branch { ..*branch });
                    } else {
                        // ...and the previous was broken, terminate it and advance group
                        if branch.length == groups[branch.group as usize] {
                            current_branches.push(Branch {
                                group: branch.group + 1,
                                length: 0,
                                ..*branch
                            });
                        }
                    }
                }

                // In the case that it's a working spring...
                Spring::Working => {
                    if branch.length == 0 {
                        // ...and the previous was working, just keep going
                        current_branches.push(Branch { ..*branch });
                    } else {
                        // ...and the previous was broken, terminate it  and advance group
                        if branch.length == groups[branch.group as usize] {
                            current_branches.push(Branch {
                                group: branch.group + 1,
                                length: 0,
                                ..*branch
                            });
                        }
                    }
                }
            }
        }

        regroup_branches(&mut previous_branches, &mut current_branches);
    }

    // Valid arrangements are those that have found all groups
    previous_branches
        .iter()
        .filter(|b| b.group as usize == groups.len())
        .map(|b| b.permutations)
        .sum()
}

/// To avoid redundant computation, branches are regrouped at the end of each
/// stage on the condition that they have the same amount and group counters.
///
/// This is done by an in-place sort and a single pass using `group_by()`, summing
/// the permutations into a single new branch.
///
/// All new branches are then stored in the previous buffer (which is cleared),
/// and the current buffer is cleared for the next iteration.
fn regroup_branches(previous: &mut Vec<Branch>, current: &mut Vec<Branch>) {
    previous.truncate(0);
    current.sort_unstable_by_key(|b| (b.group, b.length));

    let grouped_branches = Itertools::group_by(current.iter(), |b| (b.group, b.length));

    let merged_branches = grouped_branches
        .into_iter()
        .map(|((group, amount), items)| Branch {
            length: amount,
            group,
            permutations: items.map(|b| b.permutations).sum(),
        });

    previous.extend(merged_branches);
    current.truncate(0);
}

impl Default for Branch {
    fn default() -> Self {
        Self {
            length: 0,
            group: 0,
            permutations: 1,
        }
    }
}

impl From<u8> for Spring {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Spring::Working,
            b'#' => Spring::Broken,
            b'?' => Spring::Unknown,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const W: Spring = Spring::Working;
    const B: Spring = Spring::Broken;
    const U: Spring = Spring::Unknown;

    #[test]
    fn test_arrangement_count_1() {
        let springs = [W, U, U, W, W, U, U, W, W, W, U, B, B, W];
        let counts = [1, 1, 3];
        let result = valid_spring_arrangement_count(&springs, &counts);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_arrangement_count_2() {
        let springs = [U, B, B, B, U, U, U, U, U, U, U, U];
        let groups = [3, 2, 1];
        let result = valid_spring_arrangement_count(&springs, &groups);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_arrangement_large_scale() {
        let result = part_two("#??#???.??#?#?#??#?. 6,8,2");
        assert_eq!(result, Some(16));
    }
}
