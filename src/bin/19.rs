advent_of_code::solution!(19);

use std::{
    collections::HashMap,
    ops::{Index, IndexMut, Range},
    slice,
};

use advent_of_code::tools::atom::{Atom, AtomTable};
use itertools::Itertools;

/* == Definitions == */

const FIRST_WORKFLOW: &str = "in";
const NUMBER_CATEGORIES: usize = 4;
const MAX_CATEGORY_RANGE: Range<u16> = 1..4001;

#[derive(Clone)]
struct StatRange(Range<u16>);

#[derive(Copy, Clone)]
enum Category {
    Cool,
    Musical,
    Aerodynamic,
    Shiny,
}

/* == Solutions == */

pub fn part_one(input: &str) -> Option<u64> {
    let (workflows, parts) = parse_input(input);

    let result = parts
        .map(|part| {
            WorkflowIterator::new(&workflows, part)
                .map(|part| part.stat_total())
                .sum::<u64>()
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (workflows, _) = parse_input(input);

    let result = WorkflowIterator::new(&workflows, Part::default())
        .map(|part| part.combinations())
        .sum();

    Some(result)
}

/* == Input parsing == */

/// Collects all workflows into a HashMap for lookup, also returning an iterator
/// for the parts that follow. The iterator is lazy, so if the parts are not needed,
/// they will not be parsed.
fn parse_input(input: &str) -> (Workflows, impl Iterator<Item = Part> + '_) {
    let mut lines = input.lines();

    // Uses an atom table to store workflow tags as a unique usize for fast lookup
    let mut atoms = AtomTable::new();
    let mut flows = HashMap::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            return (Workflows { atoms, flows }, lines.map(Part::parse_str));
        }

        let (k, v) = Workflow::parse_str(line, &mut atoms);
        flows.insert(k, v);
    }

    panic!("Expected part definitions, found EOF");
}

/* == Workflow iterator == */

/// The main workhorse of the algorithm. It takes a reference to the workflows
/// and a part to start with, and returns a lazy iterator of part ranges that
/// are accepted by the workflows (in no particular order). The part is progressively
/// split into smaller ranges as it is processed, stored in a FIFO queue.
struct WorkflowIterator<'a> {
    current: Option<(Part, slice::Iter<'a, Instruction>)>,
    queue: Vec<(Part, Atom)>,
    workflows: &'a Workflows<'a>,
}

impl WorkflowIterator<'_> {
    fn new<'a>(workflows: &'a Workflows, part: Part) -> WorkflowIterator<'a> {
        WorkflowIterator {
            current: Some((part, workflows.first().iter())),
            queue: vec![],
            workflows,
        }
    }
}

impl Iterator for WorkflowIterator<'_> {
    type Item = Part;

    fn next(&mut self) -> Option<Self::Item> {
        // This is the main cursor of the iterator, it contains the actively
        // processed part and workflow and is set to None when the part range
        // has been fully consumed.
        let (part, instructions) = self.current.as_mut()?;

        // Keep looping until a part range is accepted, or the queue is empty
        loop {
            while part.is_empty() {
                match self.queue.pop() {
                    Some((new_part, tag)) => {
                        *part = new_part;
                        *instructions = self.workflows.get(tag).0.iter();
                    }

                    // If the queue is empty, we have exhausted all workflows
                    None => {
                        self.current = None;
                        return None;
                    }
                }
            }

            // Apply the next instruction to the part range, possibly returning
            // a new part range that was created by the instruction. This is either
            // returned as an accepted range, or added to the queue for further processing.
            // The old part is mutated in place, trimming its range as necessary.
            match instructions.next() {
                Some(instruction) => match instruction.apply(part) {
                    // Add the new part range to the queue
                    Some((new_part, Action::Jump(tag))) => {
                        self.queue.push((new_part, tag));
                    }

                    // Yield the new accepted part range
                    Some((new_part, Action::Accept)) => {
                        return Some(new_part);
                    }

                    // Drop rejected parts, or no-op if part was not split
                    Some((_, Action::Reject)) | None => {}
                },

                // Each workflow should be terminated by a condition-less instruction
                // that should consume the part, and leave the old range empty.
                None => {
                    if !part.is_empty() {
                        panic!("Non-zero part range after workflow completion");
                    }
                }
            }
        }
    }
}

/* == Implementations == */

struct Workflows<'a> {
    atoms: AtomTable<&'a str>,
    flows: HashMap<Atom, Workflow>,
}

impl Workflows<'_> {
    fn get(&self, atom: Atom) -> &Workflow {
        self.flows.get(&atom).unwrap()
    }

    fn first(&self) -> &Workflow {
        self.get(self.atoms.get(FIRST_WORKFLOW).unwrap())
    }
}

struct Workflow(Vec<Instruction>);

impl Workflow {
    fn parse_str<'a>(input: &'a str, table: &mut AtomTable<&'a str>) -> (Atom, Workflow) {
        let (tag, rest) = input.split_once('{').unwrap();

        let instructions = rest[..rest.len() - 1]
            .split(',')
            .map(|instruction| Instruction::parse_str(instruction, table))
            .collect();

        (table.create(tag), Workflow(instructions))
    }

    fn iter(&self) -> slice::Iter<'_, Instruction> {
        self.0.iter()
    }
}

struct Instruction {
    action: Action,
    condition: Option<Predicate>,
}

impl Instruction {
    fn parse_str<'a>(instruction: &'a str, table: &mut AtomTable<&'a str>) -> Instruction {
        match instruction.split_once(':') {
            Some((condition, then)) => Instruction {
                condition: Some(Predicate::parse_str(condition)),
                action: Action::parse_str(then, table),
            },
            None => Instruction {
                condition: None,
                action: Action::parse_str(instruction, table),
            },
        }
    }

    /// Applies the instruction to the part, mutating it in place and optionally
    /// returning the new split part range as a result of the instruction with
    /// the associated action that should be taken.
    fn apply(&self, part: &mut Part) -> Option<(Part, Action)> {
        let mut new_part = part.clone();

        match &self.condition {
            // Verify that the condition is met before trying to split the range
            Some(condition) => {
                let stat = part.category(condition.operand);
                let new_stat = new_part.category(condition.operand);

                // If the condition is met, split the ranges as necessary
                match condition.test {
                    PredicateTest::GreaterThan(than) if stat.end > than + 1 => {
                        stat.end = stat.start.max(than + 1);
                        new_stat.start = stat.start.max(than + 1);
                    }

                    PredicateTest::LessThan(than) if stat.start < than => {
                        stat.start = stat.end.min(than);
                        new_stat.end = stat.end.min(than);
                    }

                    // Condition not met, the instruction is a no-op
                    _ => return None,
                }
            }

            // No condition, consume the original part completely.
            // The returned new part contains the entire range.
            None => {
                part.empty();
            }
        }

        Some((new_part, self.action.clone()))
    }
}

struct Predicate {
    operand: Category,
    test: PredicateTest,
}

enum PredicateTest {
    GreaterThan(u16),
    LessThan(u16),
}

impl Predicate {
    fn parse_str(condition: &str) -> Predicate {
        let mut chars = condition.chars();
        let stat_char = chars.next().unwrap();
        let op = chars.next().unwrap();
        let amount = condition[2..].parse().unwrap();

        Predicate {
            operand: Category::from(stat_char),
            test: match op {
                '<' => PredicateTest::LessThan(amount),
                '>' => PredicateTest::GreaterThan(amount),
                _ => unreachable!(),
            },
        }
    }
}

#[derive(Clone)]
enum Action {
    Accept,
    Reject,
    Jump(Atom),
}

impl Action {
    fn parse_str<'a>(action: &'a str, table: &mut AtomTable<&'a str>) -> Action {
        match action {
            "A" => Action::Accept,
            "R" => Action::Reject,
            tag => Action::Jump(table.create(tag)),
        }
    }
}

#[derive(Clone, Default)]
struct Part {
    stats: [StatRange; NUMBER_CATEGORIES],
}

impl Part {
    fn parse_str(line: &str) -> Part {
        let (x, m, a, s) = line[1..line.len() - 1]
            .split(',')
            .map(|x| x[2..].parse::<u16>().unwrap().into())
            .collect_tuple()
            .unwrap();

        Part {
            stats: [x, m, a, s],
        }
    }

    /// Returns a mutable reference to the underlying Range<u16> for the given category.
    fn category(&mut self, category: Category) -> &mut Range<u16> {
        &mut self.stats[category as usize].0
    }

    /// Empties the part, setting the first range to 0..0 for speed (it's impossible to
    /// represent any valid range with the other non-zero categories).
    fn empty(&mut self) {
        self.stats[0] = StatRange(0..0);
    }

    fn is_empty(&self) -> bool {
        self.stats.iter().any(|x| x.0.is_empty())
    }

    /// Returns the sum of all ranges in the part.
    fn stat_total(&self) -> u64 {
        self.stats.iter().map(|x| x.sum()).sum()
    }

    /// Returns the number of possible part combinations for the ranges.
    fn combinations(&self) -> u64 {
        self.stats.iter().map(|x| x.0.len() as u64).product()
    }
}

impl StatRange {
    /// Returns the the sum of all numbers included within the range, using
    /// the formula for the sum of an arithmetic sequence.
    fn sum(&self) -> u64 {
        let start = self.0.start as u64;
        let end = self.0.end as u64;
        (end * (end - 1) - start * (start - 1)) / 2
    }
}

/* == Trait implementations == */

impl From<u16> for StatRange {
    fn from(value: u16) -> Self {
        StatRange(value..value + 1)
    }
}

impl From<char> for Category {
    fn from(value: char) -> Self {
        match value {
            'x' => Category::Cool,
            'm' => Category::Musical,
            'a' => Category::Aerodynamic,
            's' => Category::Shiny,
            _ => unreachable!(),
        }
    }
}

// Allows the Category enum to index a slice with the correct number of elements
impl<T> Index<Category> for [T; NUMBER_CATEGORIES] {
    type Output = T;

    fn index(&self, stat: Category) -> &Self::Output {
        &self[stat as usize]
    }
}

impl<T> IndexMut<Category> for [T; NUMBER_CATEGORIES] {
    fn index_mut(&mut self, index: Category) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl Default for StatRange {
    fn default() -> Self {
        StatRange(MAX_CATEGORY_RANGE.clone())
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
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_example(DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
