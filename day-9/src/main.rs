use day_9::compute_tail_locations;

fn main() {
    let contents = include_str!("input.txt").trim();
    println!("{}", compute_tail_locations(contents, 2).len());
    println!("{}", compute_tail_locations(contents, 10).len()); // I get 2509, which is wrong :(
}
