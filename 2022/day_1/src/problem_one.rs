mod elves;

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("missing input file");

    let max_calories = elves::get_elves_and_calories(&input)
        .max()
        .expect("No elves found...");

    println!("Max calories: {max_calories}");
}
