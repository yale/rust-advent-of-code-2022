use std::collections::HashSet;
use std::cmp;
use itertools::Itertools;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Coord(u32, u32);

impl Coord {
    fn to(&self, other: &Self) -> Vec<Self> {
        if self.0 == other.0 {
            let lower = cmp::min(self.1, other.1);
            let upper = cmp::max(self.1, other.1);
            (lower..=upper).map(|y| Self(self.0, y)).collect()
        } else if self.1 == other.1 {
            let lower = cmp::min(self.0, other.0);
            let upper = cmp::max(self.0, other.0);
            (lower..=upper).map(|x| Self(x, self.1)).collect()
        } else {
            panic!("points are not aligned")
        }
    }

    fn parse(input: &str) -> Option<Self> {
        let mut parts = input.split(",").map(str::parse::<u32>).map(Result::unwrap);
        Some(Self(parts.next().unwrap(), parts.next().unwrap()))
    }
}

#[derive(Debug)]
pub struct Cave {
    rocks: HashSet<Coord>,
    sand: HashSet<Coord>,
    use_floor: bool,
    x_min: u32,
    x_max: u32,
    floor: u32
}

impl Cave {
    pub fn parse(input: &str, use_floor: bool) -> Self {
        let mut cave = Self::new(use_floor);
        input.split("\n").for_each(|line| cave.parse_line(line));
        cave
    }

    pub fn pour_sand(&mut self) {
        self.last();
    }

    pub fn num_sand(&self) -> usize {
        self.sand.len()
    }

    fn new(use_floor: bool) -> Self {
        Self {
            rocks: HashSet::new(),
            sand: HashSet::new(),
            use_floor,
            x_min: 500,
            x_max: 500,
            floor: 2
        }
    }

    fn parse_line(&mut self, input: &str) {
        input.split(" -> ")
            .map(Coord::parse)
            .map(Option::unwrap)
            .tuple_windows().for_each(|(first, second)| {
                self.add_rocks(first, second);
            });
    }

    fn add_rocks(&mut self, start: Coord, end: Coord) {
        for coord in start.to(&end) {
            self.add_rock(coord);
        }
    }

    fn add_rock(&mut self, coord: Coord) {
        self.x_min = cmp::min(coord.0, self.x_min);
        self.x_max = cmp::max(coord.0, self.x_max);
        self.floor = cmp::max(coord.1 + 2, self.floor);
        self.rocks.insert(coord);
    }

    fn add_sand(&mut self) -> Option<Coord> {
        let mut x: u32 = 500;

        for y in 0.. {
            if self.blocked(&Coord(500, 0)) {
                return None
            }

            if !self.use_floor && (x < self.x_min || x > self.x_max) {
                return None
            }

            if !self.blocked(&Coord(x, y + 1)) {
                continue;
            }

            if !self.blocked(&Coord(x - 1, y + 1)) {
                x -= 1;
                continue;
            }

            if !self.blocked(&Coord(x + 1, y + 1)) {
                x += 1;
                continue;
            }

            let resting_place = Coord(x, y);
            self.sand.insert(resting_place);
            return Some(resting_place)
        }
        None
    }

    fn blocked(&self, coord: &Coord) -> bool {
        self.rocks.contains(coord) || self.sand.contains(coord) || (self.use_floor && coord.1 >= self.floor)
    }
}

impl Iterator for Cave {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        self.add_sand()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cave() {
        let input = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";
        let cave = Cave::parse(input, false);
        assert_eq!(cave.rocks.contains(&Coord(498, 4)), true);
    }

    #[test]
    fn pour_sand() {
        let input = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";
        let mut cave = Cave::parse(input, false);
        cave.pour_sand();
        assert_eq!(cave.sand.contains(&Coord(500, 8)), true);
        assert_eq!(cave.sand.contains(&Coord(500, 2)), true);
        assert_eq!(cave.num_sand(), 24);
    }

    #[test]
    fn pour_sand_with_floor() {
        let input = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";
        let mut cave = Cave::parse(input, true);
        cave.pour_sand();
        assert_eq!(cave.num_sand(), 93);
    }

    #[test]
    fn add_sand() {
        let input = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";
        let mut cave = Cave::parse(input, false);
        cave.add_sand();
        assert_eq!(cave.sand.contains(&Coord(500, 8)), true);
        assert_eq!(cave.num_sand(), 1);
    }

    #[test]
    fn parse_line() {
        let line = "498,4 -> 498,6 -> 496,6";
        let mut cave = Cave::new(false);
        cave.parse_line(line);

        assert_eq!(cave.rocks.contains(&Coord(498, 4)), true);
        assert_eq!(cave.rocks.contains(&Coord(498, 5)), true);
        assert_eq!(cave.rocks.contains(&Coord(498, 6)), true);
        assert_eq!(cave.rocks.contains(&Coord(497, 6)), true);
        assert_eq!(cave.rocks.contains(&Coord(496, 6)), true);
    }

    #[test]
    fn parse_coord() {
        let coord = Coord::parse("498,4");
        assert_eq!(coord, Some(Coord(498,4)));
    }

    #[test]
    fn add_rocks() {
        let mut cave = Cave::new(false);
        cave.add_rocks(Coord(1, 2), Coord(1, 4));
        assert_eq!(cave.rocks.contains(&Coord(1, 2)), true);
        assert_eq!(cave.rocks.contains(&Coord(1, 3)), true);
        assert_eq!(cave.rocks.contains(&Coord(1, 4)), true);
    }

    #[test]
    fn add_rock() {
        let mut cave = Cave::new(false);
        cave.add_rock(Coord(1, 2));
        assert_eq!(cave.rocks.contains(&Coord(1, 2)), true);
    }
}
