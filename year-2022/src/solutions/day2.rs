use crate::solutions::{AdventSolution, Day};


impl AdventSolution for Day<2> {
    type OutputOne = u64;
    type OutputTwo = u64;

    fn problem_one(input: &str) -> Self::OutputOne {
        solve_problem::<Hand, Outcome, Hand>(input)
    }

    fn problem_two(input: &str) -> Self::OutputTwo {
        solve_problem::<Hand, Hand, Outcome>(input)
    }
}

fn solve_problem<L, Ratio, Unknown>(input: &str) -> u64
    where
        L: From<String>,
        Ratio: From<String> + PointsT + Clone,
        Unknown: From<(L, Ratio)> + PointsT
{
    rps_iter::<L, Ratio>(input)
        .map(|(left, right)| Unknown::from((left, right.clone())).get_points() + right.get_points())
        .sum::<u64>()
}

fn rps_iter<Left, Right>(input: &str) -> impl Iterator<Item = (Left, Right)> + '_
    where
        Left: From<String>,
        Right: From<String>
{
    input
        .lines()
        .map(|line: &str| {
            let [left, right]: [String; 2] = line
                .split(" ")
                .map(String::from)
                .collect::<Vec<String>>()
                .try_into()
                .expect("Failed to parse line");

            (left.into(), right.into())
        })
}

#[derive(Debug, Clone, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors
}

impl From<(Hand, Outcome)> for Hand {
    fn from((opponent_hand, game_outcome): (Hand, Outcome)) -> Self {
        match (opponent_hand, game_outcome) {
            (opponent_hand, Outcome::Win) => opponent_hand.get_hand_this_loses_against(),
            (opponent_hand, Outcome::Loss) => opponent_hand.get_hand_this_wins_against(),
            (opponent_hand, _) => opponent_hand
        }
    }
}

impl From<String> for Hand {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "a" | "x" => Hand::Rock,
            "b" | "y" => Hand::Paper,
            "c" | "z" => Hand::Scissors,
            _ => unimplemented!("Invalid hand")
        }
    }
}

impl PointsT for Hand {
    fn get_points(&self) -> u64 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3
        }
    }
}

impl Hand {
    /// This returns the hand that this hand will do the best against
    fn get_hand_this_wins_against(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper
        }
    }

    /// This returns the hand that this hand will do the worst against
    fn get_hand_this_loses_against(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Scissors => Hand::Rock,
            Hand::Paper => Hand::Scissors,
        }
    }
}

#[derive(Clone)]
enum Outcome {
    Win,
    Draw,
    Loss
}

impl From<String> for Outcome {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "x" => Outcome::Loss,
            "y" => Outcome::Draw,
            "z" => Outcome::Win,
            _ => unimplemented!("Invalid outcome")
        }
    }
}

impl From<(Hand, Hand)> for Outcome {
    fn from((opponent, us): (Hand, Hand)) -> Self {
        match (opponent, us) {
            (opponent_hand, our_hand) if opponent_hand == our_hand => Outcome::Draw,
            (opponent_hand, our_hand) if our_hand == opponent_hand.get_hand_this_loses_against() => Outcome::Win,
            _ => Outcome::Loss
        }
    }
}

impl PointsT for Outcome {
    fn get_points(&self) -> u64 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0
        }
    }
}

trait PointsT {
    fn get_points(&self) -> u64;
}