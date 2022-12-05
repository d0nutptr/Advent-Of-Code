mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

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