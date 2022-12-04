use itertools::Itertools;
use phf::phf_map;
use std::collections::HashSet;

// Generated using `cargo run --bin gen_priorities`
pub static PRIORITIES: phf::Map<char, u32> = phf_map! {
    'a' => 1,
    'b' => 2,
    'c' => 3,
    'd' => 4,
    'e' => 5,
    'f' => 6,
    'g' => 7,
    'h' => 8,
    'i' => 9,
    'j' => 10,
    'k' => 11,
    'l' => 12,
    'm' => 13,
    'n' => 14,
    'o' => 15,
    'p' => 16,
    'q' => 17,
    'r' => 18,
    's' => 19,
    't' => 20,
    'u' => 21,
    'v' => 22,
    'w' => 23,
    'x' => 24,
    'y' => 25,
    'z' => 26,
    'A' => 27,
    'B' => 28,
    'C' => 29,
    'D' => 30,
    'E' => 31,
    'F' => 32,
    'G' => 33,
    'H' => 34,
    'I' => 35,
    'J' => 36,
    'K' => 37,
    'L' => 38,
    'M' => 39,
    'N' => 40,
    'O' => 41,
    'P' => 42,
    'Q' => 43,
    'R' => 44,
    'S' => 45,
    'T' => 46,
    'U' => 47,
    'V' => 48,
    'W' => 49,
    'X' => 50,
    'Y' => 51,
    'Z' => 52,
};

struct Rucksack<'a>(&'a str);

impl<'a> Rucksack<'a> {
    fn new(str: &'a str) -> Self {
        Rucksack(str)
    }

    pub fn find_duplicate(self: &Self) -> Option<&u32> {
        let mut found: HashSet<u32> = HashSet::new();
        let size: u32 = self.0.len().try_into().expect("size out of bounds");
        let half = (size / 2) as usize;

        let priorities = self
            .0
            .chars()
            .map(|c| PRIORITIES.get(&c).expect("could not get priority"));

        let dupe = priorities.enumerate().find(|(idx, &n)| {
            if idx < &half {
                found.insert(n);
                false
            } else {
                found.contains(&&n)
            }
        });

        match dupe {
            Some((_, a)) => Some(a),
            None => None,
        }
    }
}

fn find_group<'a>(mut elves: impl Iterator<Item = &'a str>) -> Option<u32> {
    let first: HashSet<char> = HashSet::from_iter(elves.next().unwrap().chars());
    let second: HashSet<char> = HashSet::from_iter(elves.next().unwrap().chars());
    let third: HashSet<char> = HashSet::from_iter(elves.next().unwrap().chars());

    for c in first.intersection(&second) {
        if third.contains(c) {
            let priority = *PRIORITIES.get(&c).unwrap();
            return Some(priority);
        }
    }

    None
}

fn part_1(contents: &'static str) -> u32 {
    contents
        .lines()
        .map(Rucksack::new)
        .map(|r| *r.find_duplicate().expect("No dupe found"))
        .sum()
}

fn part_2(contents: &'static str) -> u32 {
    contents
        .lines()
        .chunks(3)
        .into_iter()
        .map(|elves| find_group(elves).unwrap())
        .sum()
}

fn main() {
    let contents = include_str!("input.txt").trim();
    println!("{}", part_1(&contents));
    println!("{}", part_2(&contents));
}
