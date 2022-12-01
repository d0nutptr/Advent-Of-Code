fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Problem one: {}", problem_one(&input));
    println!("Problem two: {}", problem_two(&input));
}

fn problem_one(input: &str) -> u64 {
    elf_calorie_iterator(&input)
        .max()
        .unwrap_or(0)
}

fn problem_two(input: &str) -> u64 {
    use std::collections::BinaryHeap;

    let elf_calories = elf_calorie_iterator(&input).collect::<BinaryHeap<u64>>();

    elf_calories
        .iter()
        .take(3)
        .sum::<u64>()
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