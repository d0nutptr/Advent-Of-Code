use crate::solutions::{AdventSolution, Day};

impl AdventSolution for Day<6> {
    type OutputOne = usize;
    type OutputTwo = usize;

    fn problem_one(input: &str) -> Self::OutputOne {
        solve_problem_without_bitset(input, 4)
    }

    fn problem_two(input: &str) -> Self::OutputTwo {
        solve_problem_without_bitset(input, 14)
    }
}

// Original solution to the problem
fn _solve_problem(input: &str, window: usize) -> usize {
    use bit_set::BitSet;

    input
        .as_bytes()
        .windows(window)
        .map(|elements| {
            elements
                .into_iter()
                .map(|elem| *elem as usize)
                .collect::<BitSet>()
        })
        .position(|set| set.len() == window)
        .unwrap() + window
}

fn solve_problem_without_bitset(input: &str, window: usize) -> usize {
    input
        .as_bytes()
        .windows(window)
        .filter_map(|element| {
            element
                .iter()
                .map(|elem| 1 << (*elem - b'a'))
                .reduce(|left, right| left | right)
        })
        .position(|num: u64| num.count_ones() == window as _)
        .unwrap() + window
}