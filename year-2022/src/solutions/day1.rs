use crate::solutions::{AdventSolution, Day};


impl AdventSolution for Day<1> {
    type OutputOne = u64;
    type OutputTwo = u64;

    fn problem_one(input: &str) -> Self::OutputOne {
        elf_calorie_iterator(&input)
            .max()
            .unwrap_or(0)
    }

    fn problem_two(input: &str) -> Self::OutputTwo {
        use std::collections::BinaryHeap;

        elf_calorie_iterator(&input)
            .collect::<BinaryHeap<u64>>()
            .into_iter()
            .take(3)
            .sum::<u64>()
    }
}


fn elf_calorie_iterator(input: &str) -> impl Iterator<Item = u64> + '_ {
    input
        .split("\n\n")
        .map(|elf_inventory: &str| {
            elf_inventory
                .lines()
                .filter_map(|food| food.parse::<u64>().ok())
                .sum()
        })
}