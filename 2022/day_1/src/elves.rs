pub fn get_elves_and_calories(input: &str) -> impl Iterator<Item = u64> + '_ {
    input
        .split("\n\n")
        .map(|elf_inventory: &str| {
            elf_inventory
                .lines()
                .map(|item| item.parse::<u64>().expect("Failed to parse calorie content"))
                .sum::<u64>()
        })
}