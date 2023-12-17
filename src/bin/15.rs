advent_of_code::solution!(15);

use std::collections::HashMap;

use itertools::Itertools;

/* == Definitions == */

const HASH_FACTOR: u8 = 17;
const NUMBER_BOXES: usize = 256;

#[derive(Clone)]
struct LensBox {
    lenses: Vec<Lens>,
}

struct Instruction<'a> {
    label: &'a str,
    operation: Operation,
}

#[derive(Clone)]
struct Lens {
    atom: Atom,
    focal_length: u8,
}

enum Operation {
    Add(u8),
    Subtract,
}

/// Lookup table for atoms associated with string labels.
struct AtomTable<'a> {
    counter: u32,
    labels: HashMap<&'a str, Atom>,
}

/// Represents a string label as a unique integer instead, with the
/// mapping stored in `AtomTable`. Allows for fast comparison without
/// heap-allocated strings.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Atom(u32, u8);

/* == Solutions == */

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .flat_map(|line| line.split(','))
        .map(|line| hash(line) as u32)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut atoms = AtomTable::new();
    let mut boxes = vec![LensBox::new(); NUMBER_BOXES];

    let instructions = input
        .lines()
        .flat_map(|line| line.split(','))
        .map(parse_instruction);

    for instruction in instructions {
        let box_index = hash(instruction.label) as usize;
        let bx = &mut boxes[box_index];
        let atom = atoms.get_label(instruction.label);

        match instruction.operation {
            Operation::Add(focal_length) => bx.add_lens(Lens { atom, focal_length }),
            Operation::Subtract => bx.remove_lens(&atom),
        }
    }

    let result = boxes
        .iter()
        .enumerate()
        .map(|(i, b)| b.focal_power(i))
        .sum();

    Some(result)
}

/* == Input parsing == */

fn parse_instruction(instruction: &str) -> Instruction {
    let index = instruction.find(|x| x == '=' || x == '-').unwrap();
    let (label, op_str) = instruction.split_at(index);
    let mut op_iterator = op_str.bytes();

    match op_iterator.next().unwrap() {
        b'=' => Instruction {
            label,
            operation: Operation::Add(op_iterator.next().unwrap() - b'0'),
        },
        b'-' => Instruction {
            label,
            operation: Operation::Subtract,
        },
        _ => unreachable!(),
    }
}

/* == Implementations == */

impl LensBox {
    fn new() -> LensBox {
        LensBox { lenses: Vec::new() }
    }

    /// Adds a lens to the back of the box, or if a lens with the same label (atom)
    /// is already present, swap it out with the new focal length.
    fn add_lens(&mut self, lens: Lens) {
        match self.lenses.iter_mut().find(|l| l.atom == lens.atom) {
            Some(l) => l.focal_length = lens.focal_length,
            None => self.lenses.push(lens),
        }
    }

    /// Removes a lens from a table.
    fn remove_lens(&mut self, atom: &Atom) {
        if let Some((i, _)) = self.lenses.iter().find_position(|l| l.atom == *atom) {
            self.lenses.remove(i);
        }
    }

    // Computes the focal power of the box.
    fn focal_power(&self, box_index: usize) -> u32 {
        self.lenses
            .iter()
            .enumerate()
            .map(|(i, lens)| (box_index as u32 + 1) * (i as u32 + 1) * lens.focal_length as u32)
            .sum()
    }
}

impl<'a> AtomTable<'a> {
    fn new() -> AtomTable<'a> {
        AtomTable {
            counter: 0,
            labels: HashMap::new(),
        }
    }

    /// Returns the atom associated with a label, or creates a new one if
    /// the label is not yet present.
    fn get_label<'b: 'a>(&mut self, label: &'b str) -> Atom {
        match self.labels.get(label) {
            Some(label) => *label,
            None => {
                self.counter += 1;
                let atom = Atom(self.counter, hash(label));
                self.labels.insert(label, atom);
                atom
            }
        }
    }
}

/* == Utility == */

fn hash(input: &str) -> u8 {
    input.bytes().fold(0, |acc, byte| {
        acc.wrapping_add(byte).wrapping_mul(HASH_FACTOR)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::template::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_example(DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example(DAY));
        assert_eq!(result, Some(145));
    }

    #[test]
    fn test_hash() {
        let result = hash("HASH");
        assert_eq!(result, 52);
    }
}
