use crate::Day;
use std::{env, fs};

pub mod aoc_cli;
pub mod commands;
pub mod readme_benchmarks;
pub mod runner;

pub const ANSI_ITALIC: &str = "\x1b[3m";
pub const ANSI_BOLD: &str = "\x1b[1m";
pub const ANSI_RESET: &str = "\x1b[0m";

#[must_use]
pub fn read_data(folder: &str, name: &str) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd.join("data").join(folder).join(format!("{name}.txt"));
    let f = fs::read_to_string(filepath);
    f.expect("could not open data")
}

#[must_use]
pub fn read_input(day: Day) -> String {
    read_data("inputs", &day.to_string())
}

#[must_use]
pub fn read_example(day: Day) -> String {
    read_data("examples", &day.to_string())
}

#[must_use]
pub fn read_example_part(day: Day, case: u32) -> String {
    read_data("examples", &format!("{day}-{case}"))
}

/// Creates the constant `DAY` and sets up the input and runner for each part.
#[macro_export]
macro_rules! solution {
    ($day:expr) => {
        /// The current day.
        const DAY: advent_of_code::Day = advent_of_code::day!($day);

        fn main() {
            use advent_of_code::template::runner::*;
            let input = advent_of_code::template::read_input(DAY);
            run_part(part_one, &input, DAY, 1);
            run_part(part_two, &input, DAY, 2);
        }
    };
}
