pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

use std::fmt::Display;

pub struct Day<const NUM: u64>;

pub trait AdventDay {
    const NUM: u64;
    fn get_day_name() -> String;
}

impl<const NUM: u64> AdventDay for Day<NUM> {
    const NUM: u64 = NUM;

    fn get_day_name() -> String {
        format!("Day {}", NUM)
    }
}

pub trait AdventSolution: AdventDay {
    type OutputOne: Display;
    type OutputTwo: Display;

    fn problem_one(input: &str) -> Self::OutputOne;
    fn problem_two(input: &str) -> Self::OutputTwo;

    fn run() {
        let path = format!("inputs/day{}.txt", Self::NUM);

        let input = std::fs::read_to_string(path).unwrap();

        println!("Solution for {}", Self::get_day_name());
        println!("\tProblem one: {}", Self::problem_one(&input));
        println!("\tProblem two: {}\n", Self::problem_two(&input));
    }
}