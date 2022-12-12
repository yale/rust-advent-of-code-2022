use day_12::shortest_dist;

fn main() {
    let input = include_str!("input.txt").trim();
    println!("{}", shortest_dist(input, false));
    println!("{}", shortest_dist(input, true));
}
