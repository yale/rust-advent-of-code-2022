use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");

    let elves = contents.trim().split("\n\n");

    let mut calories_per_elf: Vec<i32> = elves.map(|calories_group| {
        calories_group.split("\n").map(|calories| {
            match calories.parse::<i32>() {
                Ok(n) => n,
                Err(_) => panic!("Invalid input!")
            }
        }).sum()
    }).collect();

    calories_per_elf.sort();
    calories_per_elf.reverse();

    let answer_1: i32 = calories_per_elf[0];
    let answer_2: i32 = calories_per_elf[0..3].iter().sum();

    println!("{}", answer_1);
    println!("{}", answer_2);
}
