use std::collections::BinaryHeap;

mod elves;

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("missing input file");

    let elf_calories = elves::get_elves_and_calories(&input).collect::<BinaryHeap<u64>>();

    let total_calories = elf_calories.iter()
        .take(3)
        .sum::<u64>();

    println!("Total calories of top 3 elves is {total_calories}");
}