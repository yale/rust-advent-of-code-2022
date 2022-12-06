use std::collections::HashSet;

fn index_of_first_n_distinct_chars(n: usize, string: &str) -> Option<usize> {
    string.as_bytes().windows(n).enumerate().find(|(_, window)| {
        let mut set: HashSet<u8> = HashSet::new();
        for item in *window {
            if set.contains(item) {
                return false;
            } else {
                set.insert(*item);
            }
        }
        true
    }).map(|(index, _)| index + n)
}

fn main() {
    let contents = include_str!("input.txt").trim();

    let part_1 = index_of_first_n_distinct_chars(4, contents).expect("no valid answer for part 1");
    let part_2 = index_of_first_n_distinct_chars(14, contents).expect("no valid answer for part 2");

    println!("Part 1: {}. Part 2: {}", part_1, part_2);
}
