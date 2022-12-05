use crate::solutions::{AdventSolution, Day};

impl AdventSolution for Day<4> {
    type OutputOne = usize;
    type OutputTwo = usize;

    fn problem_one(input: &str) -> Self::OutputOne {
        parse(input)
            .filter(|(left, right)| left.contains_range(&right) || right.contains_range(&left))
            .count()
    }

    fn problem_two(input: &str) -> Self::OutputTwo {
        parse(input)
            .filter(|(left, right)| left.overlaps_with(&right) || right.overlaps_with(&left))
            .count()
    }
}

fn parse(input: &str) -> impl Iterator<Item = (ClearRange, ClearRange)> + '_ {
    input
        .lines()
        .map(|line: &str| {
            let (left, right): (&str, &str) = line.split_once(",").unwrap();

            (ClearRange::from(left), ClearRange::from(right))
        })
}

struct ClearRange {
    low: u64,
    high: u64
}

impl From<&str> for ClearRange {
    fn from(value: &str) -> Self {
        let (low_str, high_str) = value.split_once("-").unwrap();

        Self {
            low: low_str.parse::<u64>().unwrap(),
            high: high_str.parse::<u64>().unwrap()
        }
    }
}

impl ClearRange {
    fn contains_range(&self, ClearRange { low, high }: &ClearRange) -> bool {
        self.low <= *low && self.high >= *high
    }

    fn overlaps_with(&self, ClearRange { low, high }: &ClearRange) -> bool {
        self.high >= *low && *high >= self.low
    }
}