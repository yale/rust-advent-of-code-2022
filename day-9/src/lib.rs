use std::collections::HashSet;

enum Direction { Up, Down, Left, Right }

struct MoveInstruction {
    dir: Direction,
    times: u8
}

impl MoveInstruction {
    fn parse(input: &str) -> Self {
        let mut parts = input.split(" ");
        let dir = match parts.next().expect("expect dir") {
            "L" => Direction::Left,
            "R" => Direction::Right,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => panic!("Invalid direction")
        };

        let times: u8 = parts.next().expect("expect times").parse().expect("expect times to be numeric");

        Self { dir, times }
    }
}

type Point = (i32, i32);

#[derive(Debug)]
struct Knot(i32, i32);

impl Knot {
    fn new() -> Self {
        Knot(0, 0)
    }

    fn adjust_to(&mut self, knot: &Knot) {
        // If the knot is two away in any diagonal direction,
        // split the difference
        if (knot.0 - self.0).abs() == 2 && (knot.1 - self.1).abs() == 2 {
            self.0 = (knot.0 + self.0) / 2;
            self.1 = (knot.1 + self.1) / 2;
        } else if knot.1 > self.1 + 1 {
            self.0 = knot.0;
            self.1 = knot.1 - 1;
        } else if knot.1 < self.1 - 1 {
            self.0 = knot.0;
            self.1 = knot.1 + 1;
        } else if knot.0 < self.0 - 1 {
            self.0 = knot.0 + 1;
            self.1 = knot.1;
        } else if knot.0 > self.0 + 1 {
            self.0 = knot.0 - 1;
            self.1 = knot.1;
        }
    }

    fn step(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => {
                self.1 = self.1 + 1;
            },
            Direction::Down => {
                self.1 = self.1 - 1;
            },
            Direction::Left => {
                self.0 = self.0 - 1;
            },
            Direction::Right => {
                self.0 = self.0 + 1;
            }
        }
    }
}

struct Rope {
    knots: Vec<Knot>
}

impl Rope {
    fn new(size: u8) -> Self {
        Self {
            knots: (0..size).map(|_| Knot::new()).collect()
        }
    }

    fn step(&mut self, dir: &Direction) {
        self.knots[0].step(dir);

        for i in 1..self.knots.len() {
            // Needed to get mutable refs to multiple vector elements at once
            let (left, right) = self.knots.split_at_mut(i);
            // dbg!(i, left.len(), right.len());
            right[0].adjust_to(&left[i-1]);
        }
    }
}

pub fn compute_tail_locations(contents: &str, rope_size: u8) -> HashSet<Point> {
    let mut rope = Rope::new(rope_size);
    let mut visited: HashSet<Point> = HashSet::new();

    visited.insert((0,0));

    for instr in contents.split("\n").map(MoveInstruction::parse) {
        for _ in 0..instr.times {
            rope.step(&instr.dir);
            let tail = rope.knots.last().expect("expected last knot");
            visited.insert((tail.0, tail.1));
        }
    }
    visited
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let example = "R 4\n\
                       U 4\n\
                       L 3\n\
                       D 1\n\
                       R 4\n\
                       D 1\n\
                       L 5\n\
                       R 2";
        let result = compute_tail_locations(example, 10);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn example_2() {
        let example = "R 5\n\
                       U 8\n\
                       L 8\n\
                       D 3\n\
                       R 17\n\
                       D 10\n\
                       L 25\n\
                       U 20";

        let result = compute_tail_locations(example, 10);
        // for point in result.iter() {
        //     print!("{}, {}; ", point.0, point.1);
        // }
        assert_eq!(result.len(), 36);
    }
}
