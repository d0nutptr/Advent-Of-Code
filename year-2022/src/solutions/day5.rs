use std::collections::VecDeque;
use itertools::Itertools;
use crate::solutions::{AdventSolution, Day};

impl AdventSolution for Day<5> {
    type OutputOne = String;
    type OutputTwo = String;

    fn problem_one(input: &str) -> Self::OutputOne {
        solve_problem::<CrateMover9000>(input)
    }

    fn problem_two(input: &str) -> Self::OutputTwo {
        solve_problem::<CrateMover9001>(input)
    }
}

fn solve_problem<Crane: CrateMoverT>(input: &str) -> String {
    let (mut shipyard, instructions) = parser(input);

    for Instruction { from_idx, to_idx, amount } in instructions {
        shipyard.move_elements::<Crane>(amount, from_idx, to_idx);
    }

    shipyard.top_elements()
        .into_iter()
        .collect::<String>()
}

fn parser(input: &str) -> (Shipyard, Vec<Instruction>) {
    let [shipyard_definition, instructions]: [&str; 2] = input.split("\n\n")
        .collect::<Vec<&str>>()
        .try_into()
        .expect("Unable to parse input file");

    let shipyard =  Shipyard::from(shipyard_definition);
    let instructions = instructions
        .lines()
        .map(Instruction::from)
        .collect::<Vec<Instruction>>();

    (shipyard, instructions)
}

impl From<&str> for Shipyard {
    fn from(input: &str) -> Self {
        let mut input = input.lines().rev();

        // get the number of stacks from the stack line
        let num_stacks = input.next()
            .expect("Missing stack count line")
            .split(" ")
            .last()
            .expect("Unable to parse stack count line")
            .parse::<usize>()
            .expect("Unable to parse stack count line");

        // create a shipyard with the number of required stacks
        let mut shipyard = Shipyard::new(num_stacks);

        /// parses a shipyard line which is a "maybe not complete" line of cargo
        fn parse_shipyard_line(line: &str) -> Vec<char> {
            line
                .chars()
                .skip(1)
                .step_by(4)
                .collect()
        }

        // for each shipyard line, add the cargo to the shipyard
        input
            .map(parse_shipyard_line)
            .for_each(|cargo_line: Vec<char>| {
                let important_elements = cargo_line
                    .into_iter()
                    .enumerate()
                    .filter(|(_, c)| !c.is_whitespace());

                for (idx, element) in important_elements {
                    shipyard.push_element(element, idx);
                }
            });

        shipyard
    }
}

#[derive(Debug)]
struct Shipyard {
    stacks: Vec<VecDeque<char>>
}

impl Shipyard {
    fn new(size: usize) -> Self {
        Self {
            stacks: vec![VecDeque::default(); size]
        }
    }

    fn top_elements(&self) -> Vec<char> {
        self.stacks
            .iter()
            .filter_map(|stack| stack.front())
            .map(|element| *element)
            .collect()
    }

    fn move_elements<Crane: CrateMoverT>(
        &mut self,
        num_elements: usize,
        from_idx: usize,
        to_idx: usize
    ) {
        let cargo_to_move: Vec<char> = self.stacks
            .get_mut(from_idx)
            .expect("Referenced non-existent stack")
            .drain(0 .. num_elements)
            .collect();

        Crane::move_crate(self, cargo_to_move, to_idx)
    }

    fn push_element(&mut self, element: char, to_idx: usize) {
        self.stacks
            .get_mut(to_idx)
            .expect("Referenced non-existent stack")
            .push_front(element);
    }
}

#[derive(Debug)]
struct Instruction {
    from_idx: usize,
    to_idx: usize,
    amount: usize
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let (amount, from_idx, to_idx) = value.split(" ")
            .skip(1)
            .step_by(2)
            .map(|element| element.parse::<usize>().expect("Failed to parse instruction line element as int"))
            .collect_tuple::<(usize, usize, usize)>()
            .expect("Failed to parse instruction line");

        Self {
            from_idx: from_idx - 1,
            to_idx: to_idx - 1,
            amount
        }
    }
}

struct CrateMover9000;
struct CrateMover9001;

trait CrateMoverT {
    fn move_crate(shipyard: &mut Shipyard, cargo: Vec<char>, to_idx: usize);
}

impl CrateMoverT for CrateMover9000 {
    fn move_crate(shipyard: &mut Shipyard, cargo: Vec<char>, to_idx: usize) {
        for element in cargo {
            shipyard.push_element(element, to_idx);
        }
    }
}

impl CrateMoverT for CrateMover9001 {
    fn move_crate(shipyard: &mut Shipyard, cargo: Vec<char>, to_idx: usize) {
        for element in cargo.into_iter().rev() {
            shipyard.push_element(element, to_idx);
        }
    }
}