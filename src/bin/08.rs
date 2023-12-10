advent_of_code::solution!(8);

use std::{collections::BTreeMap, mem};

/* == Definitions == */

const START: &str = "AAA";
const END: &str = "ZZZ";

#[derive(Copy, Clone)]
enum Cmd {
    Left,
    Right,
}

struct Map<'a> {
    directions: &'a str,
    nodes: BTreeMap<NodeId, Directions>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct NodeId(u32);

struct Directions {
    left: NodeId,
    right: NodeId,
}

/* == Solutions == */

pub fn part_one(input: &str) -> Option<u32> {
    let map = Map::parse_str(input);

    let mut cmd_cycle = map.commands().cycle();
    let destination = NodeId::parse_str(END);
    let mut steps = 0;

    let mut current_node = NodeId::parse_str(START);

    while current_node != destination {
        let command = cmd_cycle.next().unwrap();
        current_node = map.nodes.get(&current_node).unwrap().apply(command);
        steps += 1;
    }

    Some(steps)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = Map::parse_str(input);

    // This part makes a lot of assumptions on the problem input.
    // Each path is cyclic and has a single start and single end node
    // at equal distances from each other, at multiples of the number
    // of commands. This is very fast to compute (and feasible, given the
    // problem parameters), but is not a general solution. It works... okay?
    map.nodes
        .keys()
        .filter(|node| node.is_start())
        .map(|&node| map.steps_to_path_end(node))
        .reduce(lcm)
}

/* == Implementations == */

impl Map<'_> {
    fn parse_str(input: &str) -> Map {
        let mut it = input.lines();
        let directions = it.next().unwrap();
        let nodes = Self::parse_lines(it);
        Map { directions, nodes }
    }

    fn parse_lines<'a>(lines: impl Iterator<Item = &'a str>) -> BTreeMap<NodeId, Directions> {
        lines
            .flat_map(|line| {
                let (id_str, rest) = line.split_once(" = ")?;
                let (a, b) = rest[1..rest.len() - 1].split_once(", ")?;

                Some((
                    NodeId::parse_str(id_str),
                    Directions {
                        left: NodeId::parse_str(a),
                        right: NodeId::parse_str(b),
                    },
                ))
            })
            .collect()
    }

    fn commands(&self) -> impl ExactSizeIterator<Item = Cmd> + Clone + '_ {
        self.directions.bytes().map(Cmd::parse)
    }

    fn steps_to_path_end(&self, node: NodeId) -> u64 {
        let commands = self.commands().cycle().enumerate();
        let mut current_node = node;

        for (i, command) in commands {
            current_node = self.nodes.get(&current_node).unwrap().apply(command);

            if current_node.is_end() {
                return i as u64 + 1;
            }
        }

        unreachable!();
    }
}

impl NodeId {
    fn parse_str(id: &str) -> NodeId {
        let bytes: [u8; 3] = id.as_bytes().try_into().unwrap();
        NodeId(bytes.iter().fold(0, |acc, byte| (acc << 8) + *byte as u32))
    }

    fn is_start(&self) -> bool {
        (self.0 & 0xFF) as u8 == b'A'
    }

    fn is_end(&self) -> bool {
        (self.0 & 0xFF) as u8 == b'Z'
    }
}

impl Directions {
    fn apply(&self, cmd: Cmd) -> NodeId {
        match cmd {
            Cmd::Left => self.left,
            Cmd::Right => self.right,
        }
    }
}

impl Cmd {
    fn parse(cmd: u8) -> Cmd {
        match cmd {
            b'L' => Cmd::Left,
            b'R' => Cmd::Right,
            _ => panic!(),
        }
    }
}

/* == Functions == */

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;

    if min > max {
        mem::swap(&mut min, &mut max);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

/* == Tests == */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_part(DAY, 1));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_part(DAY, 2));
        assert_eq!(result, Some(6));
    }
}
