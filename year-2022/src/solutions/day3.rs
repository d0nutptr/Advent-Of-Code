use std::collections::HashSet;
use crate::solutions::{AdventSolution, Day};


impl AdventSolution for Day<3> {
    type OutputOne = u64;
    type OutputTwo = u64;

    fn problem_one(input: &str) -> Self::OutputOne {
        input
            .lines()
            .filter_map(|line: &str| {
                let (left, right) = line.split_at(line.len() / 2);

                let compartment_one = left.chars().collect::<HashSet<char>>();
                let compartment_two = right.chars().collect::<HashSet<char>>();

                compartment_one
                    .intersection(&compartment_two)
                    .map(|element| *element)
                    .next()
            })
            .map(calculate_priority)
            .sum::<u64>()
    }

    fn problem_two(input: &str) -> Self::OutputTwo {
        input
            .lines()
            .array_chunks::<3>()
            .filter_map(|elf_group| {
                let [first, second, third] = elf_group;

                let first_set = first.chars().collect::<HashSet<char>>();
                let second_set = second.chars().collect::<HashSet<char>>();
                let third_set = third.chars().collect::<HashSet<char>>();

                first_set
                    .intersection(&second_set)
                    .map(|element| *element)
                    .collect::<HashSet<char>>()
                    .intersection(&third_set)
                    .map(|element| *element)
                    .next()
            })
            .map(calculate_priority)
            .sum::<u64>()
    }
}

fn calculate_priority(input: char) -> u64 {
    match input {
        _ if input.is_lowercase() => (input as u64) - 96,
        _ => (input as u64) - 38
    }
}