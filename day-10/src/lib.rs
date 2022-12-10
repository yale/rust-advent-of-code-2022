use std::collections::VecDeque;

#[derive(Clone, Copy)]
enum Op {
    AddX(isize),
    Noop,
}

#[derive(Clone, Copy)]
struct Instruction {
    op: Op,
    cycles: usize,
}

impl Instruction {
    fn parse(str: &str) -> Option<Self> {
        if str.starts_with("addx") {
            let n = str
                .split(" ")
                .last()
                .expect("expect last part of add instr")
                .parse::<isize>()
                .expect("expect to be numeric");
            Some(Self {
                op: Op::AddX(n),
                cycles: 2,
            })
        } else if str.starts_with("noop") {
            Some(Self {
                op: Op::Noop,
                cycles: 1,
            })
        } else {
            None
        }
    }
}

pub struct CPU {
    cycle: usize,
    x: isize,
    q: VecDeque<Instruction>,
    curr_instr: Option<Instruction>,
    instr_cycle: usize,
}

#[derive(Debug)]
pub struct CPUState {
    pub x: isize,
    pub cycle: usize,
}

impl CPUState {
    pub fn signal_strength(&self) -> isize {
        self.x * (self.cycle as isize)
    }
}

impl CPU {
    pub fn init() -> Self {
        Self {
            cycle: 0,
            x: 1,
            q: VecDeque::new(),
            curr_instr: None,
            instr_cycle: 0,
        }
    }

    pub fn push_instr(&mut self, input: &str) {
        self.q.extend(
            input
                .split("\n")
                .map(Instruction::parse)
                .map(Option::unwrap),
        );

        if self.curr_instr.is_none() {
            self.curr_instr = self.q.pop_front();
        }
    }

    fn tick(&mut self) {
        if let Some(instr) = self.curr_instr {
            if instr.cycles == self.instr_cycle {
                self.exec(&instr);
                self.curr_instr = self.q.pop_front();
                self.instr_cycle = 0;
            }
        }

        self.instr_cycle += 1;
        self.cycle += 1;
    }

    fn exec(&mut self, instr: &Instruction) {
        match instr.op {
            Op::AddX(n) => self.x += n,
            Op::Noop => (),
        };
    }
}

impl Iterator for CPU {
    type Item = CPUState;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_instr.is_none() {
            return None;
        };

        self.tick();
        let x = self.x;

        Some(CPUState {
            x,
            cycle: self.cycle,
        })
    }
}

pub fn to_crt_output_buffer(cpu: CPU) -> Vec<bool> {
    cpu.map(|state| {
        dbg!(&state);
        dbg!(state.x.abs_diff(((state.cycle - 1) % 40) as isize) < 2)
    })
    .collect()
}

pub fn print_crt(output: &Vec<bool>) {
    output.chunks(40).for_each(|chunk| {
        let line: String = chunk
            .into_iter()
            .map(|on_or_off| if *on_or_off { '#' } else { '.' })
            .collect();

        println!("{}", line);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let instrs = "noop\n\
                      addx 3\n\
                      addx -5";

        let mut cpu = CPU::init();
        cpu.push_instr(instrs);

        let output: Vec<isize> = cpu.map(|s| s.x).collect();

        assert_eq!(output, vec![1, 1, 1, 4, 4, -1]);
    }

    #[test]
    fn signal_strength() {
        let instrs = "addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop";

        let mut cpu = CPU::init();
        cpu.push_instr(instrs);

        let strength: isize = cpu
            .filter(|s| (s.cycle + 20) % 40 == 0)
            .map(|s| s.signal_strength())
            .sum();

        assert_eq!(strength, 13140);
    }

    #[test]
    fn crt() {
        let instrs = "addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop";

        let mut cpu = CPU::init();
        cpu.push_instr(instrs);

        let crt_output = to_crt_output_buffer(cpu);

        print_crt(&crt_output);
    }
}
