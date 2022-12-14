use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref MOVE_INSTR_REGEX: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d)+$").unwrap();
}

type Stacks<'a> = [&'a mut Vec<char>; 9];

#[derive(Debug)]
struct MoveInstruction {
    num: usize,
    from: usize,
    to: usize,
}

impl MoveInstruction {
    pub fn parse(str: &str) -> Self {
        let captures = MOVE_INSTR_REGEX.captures(str).expect("Expected input to match regex");

        Self {
            num: captures[1].parse::<usize>().expect("Could not parse digit"),
            from: captures[2].parse::<usize>().expect("Could not parse digit") - 1,
            to: captures[3].parse::<usize>().expect("Could not parse digit") - 1
        }
    }

    pub fn exec_part_1(self: &Self, stacks: &mut Stacks) {
        for _ in 0..self.num {
            let from_val = stacks[self.from].pop().expect("Expected a value in from vec");
            stacks[self.to].push(from_val);
        }
    }

    pub fn exec_part_2(self: &Self, stacks: &mut Stacks) {
        let mut buffer: Vec<char> = Vec::new();
        for _ in 0..self.num {
            let from_val = stacks[self.from].pop().expect("Expected a value in from vec");
            buffer.push(from_val);
        }
        while !buffer.is_empty() {
            stacks[self.to].push(buffer.pop().unwrap());
        }
    }
}

fn main() {
    // I could write a function to parse and generate this from the test input,
    // but nah.
    let mut stacks: Stacks = [
        &mut Vec::from(['H', 'B', 'V', 'W', 'N', 'M', 'L', 'P']),
        &mut Vec::from(['M', 'Q', 'H']),
        &mut Vec::from(['N', 'D', 'B', 'G', 'F', 'Q', 'M', 'L']),
        &mut Vec::from(['Z', 'T', 'F', 'Q', 'M', 'W', 'G']),
        &mut Vec::from(['M', 'T', 'H', 'P']),
        &mut Vec::from(['C', 'B', 'M', 'J', 'D', 'H', 'G', 'T']),
        &mut Vec::from(['M', 'N', 'B', 'F', 'V', 'R']),
        &mut Vec::from(['P', 'L', 'H', 'M', 'R', 'G', 'S']),
        &mut Vec::from(['P', 'D', 'B', 'C', 'N']),
    ];

    let contents = include_str!("input.txt").trim();

    contents.
        lines().
        map(MoveInstruction::parse).
        for_each(|instruction| {
            // instruction.exec_part_1(&mut stacks)
            instruction.exec_part_2(&mut stacks)
        });

    for stack in stacks {
        print!("{}", stack.last().unwrap())
    }
}
