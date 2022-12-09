use day_8::{calculate_visible_trees, calculate_max_scenic_score};

fn main() {
    let contents = include_str!("input.txt").trim();

    println!("{}", calculate_visible_trees(contents));
    println!("{}", calculate_max_scenic_score(contents));
}
