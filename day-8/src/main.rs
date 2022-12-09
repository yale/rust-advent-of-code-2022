use day_8::calculate_visible_trees;

fn main() {
    let contents = include_str!("input.txt").trim();

    println!("{}", calculate_visible_trees(contents));
}
