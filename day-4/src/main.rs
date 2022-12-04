#[derive(Debug)]
struct ElfRange(u8, u8);
struct ElfRanges(ElfRange, ElfRange);

impl ElfRange {
    pub fn parse(s: &str) -> Self {
        let mut bounds = s.split('-').map(str::parse::<u8>).map(Result::unwrap);
        Self(bounds.next().unwrap(), bounds.last().unwrap())
    }

    pub fn contains(self: &Self, other: &Self) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    pub fn overlaps(self: &Self, other: &Self) -> bool {
        (self.1 >= other.0 && self.1 <= other.1) || (other.1 >= self.0 && other.1 <= self.1)
    }
}

impl ElfRanges {
    pub fn parse(line: &str) -> Self {
        let mut ranges = line.split(',').map(ElfRange::parse);
        Self(
            ranges.next().expect("Could not parse first range"),
            ranges.last().expect("Could not parse first range"),
        )
    }

    pub fn either_contains(self: &Self) -> bool {
        self.0.contains(&self.1) || self.1.contains(&self.0)
    }

    pub fn any_overlap(self: &Self) -> bool {
        self.0.overlaps(&self.1)
    }
}

fn main() {
    let contents = include_str!("input.txt").trim();

    let part_1 = contents
        .lines()
        .map(ElfRanges::parse)
        .filter(|ranges| ranges.either_contains())
        .count();

    let part_2 = contents
        .lines()
        .map(ElfRanges::parse)
        .filter(|ranges| ranges.any_overlap())
        .count();

    println!("{}", part_1);
    println!("{}", part_2);
}
