use day_14::Cave;
use std::time::Instant;

fn main() {
    let mut now = Instant::now();
    let input = include_str!("input.txt").trim();
    let mut cave = Cave::parse(input, false);
    cave.pour_sand();
    println!("{}, ran in {} ms", cave.num_sand(), now.elapsed().as_millis());

    now = Instant::now();

    let mut cave_with_floor = Cave::parse(input, true);
    cave_with_floor.pour_sand();
    println!("{}, ran in {} ms", cave_with_floor.num_sand(), now.elapsed().as_millis());
}
