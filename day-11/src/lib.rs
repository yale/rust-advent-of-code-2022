use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;

lazy_static! {
    static ref MONKEY_REGEX: Regex = Regex::new(
        r"^Monkey (?P<id>\d+):[\r\n].*items: (?P<items>[\d, ]+)[\r\n].*old (?P<operation>[*+].+)[\r\n].*divisible by (?P<divisible_by>\d+)[\r\n].*monkey (?P<condtrue>\d+)[\r\n].*monkey (?P<condfalse>\d+)"
    )
    .unwrap();
}

type WorryLevel = usize;

#[derive(Debug)]
enum OpType {
    Add(WorryLevel),
    Multiply(WorryLevel),
    Square,
}

#[derive(Debug)]
struct ThrowDecision {
    divisible_by: usize,
    if_true: usize,
    if_false: usize,
}

impl ThrowDecision {
    fn check(&self, item: WorryLevel) -> usize {
        if item.rem_euclid(self.divisible_by) == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

#[derive(Debug)]
pub struct Monkey {
    num_inspections: usize,
    items: VecDeque<WorryLevel>,
    operation: OpType,
    decision: ThrowDecision,
}

impl Monkey {
    fn parse(input: &str) -> Self {
        let captures = MONKEY_REGEX
            .captures(input)
            .expect("did not match monkey regex");

        let items: VecDeque<WorryLevel> = captures
            .name("items")
            .expect("expected items")
            .as_str()
            .split(", ")
            .map(|item| item.parse().expect("expected numerical worry level"))
            .collect();

        let op_string: &str = captures
            .name("operation")
            .expect("expected operation")
            .as_str();

        let operation = if op_string.starts_with("* old") {
            OpType::Square
        } else {
            let operand = op_string
                .split(" ")
                .last()
                .expect("expected operand")
                .parse::<usize>()
                .expect("expected numeric operand");

            if op_string.starts_with("*") {
                OpType::Multiply(operand)
            } else {
                OpType::Add(operand)
            }
        };

        let divisible_by = captures
            .name("divisible_by")
            .expect("expected divisible_by")
            .as_str()
            .parse::<usize>()
            .expect("expected numerical divisible_by");

        let if_true = captures
            .name("condtrue")
            .expect("expected if_true")
            .as_str()
            .parse::<usize>()
            .expect("expected numerical if_true");

        let if_false = captures
            .name("condfalse")
            .expect("expected if_false")
            .as_str()
            .parse::<usize>()
            .expect("expected numerical if_false");

        Self {
            num_inspections: 0,
            items,
            operation,
            decision: ThrowDecision {
                divisible_by,
                if_true,
                if_false,
            },
        }
    }

    fn throw_queue(&self, worry_modulus: Option<usize>) -> Vec<(WorryLevel, usize)> {
        self.items.iter().map(|item| {
            let new_item = self.alter(*item, worry_modulus);
            let new_monkey = self.decision.check(new_item);
            // dbg!(item, new_item, new_monkey);
            (new_item, new_monkey)
        }).collect()
    }

    fn alter(&self, item: WorryLevel, worry_modulus: Option<usize>) -> WorryLevel {
        let new_level = match self.operation {
            OpType::Square => item * item,
            OpType::Add(n) => item + n,
            OpType::Multiply(n) => item * n,
        };

        if let Some(n) = worry_modulus {
            new_level.rem_euclid(n)
        } else {
            new_level / 3
        }
    }

    fn take(&mut self, item: WorryLevel) {
        self.items.push_back(item);
    }
}

pub struct MonkeyInTheMiddle {
    pub round: usize,
    pub worry_modulus: Option<usize>,
    pub monkeys: Vec<Monkey>,
}

impl MonkeyInTheMiddle {
    pub fn init(script: &str, use_worry_modulus: bool) -> Self {
        let monkeys: Vec<Monkey> = script.split("\n\n").map(Monkey::parse).collect();

        let worry_modulus = if use_worry_modulus {
            Some(monkeys.iter().map(|m| m.decision.divisible_by).fold(1, |x, y| x*y))
        } else {
            None
        };

        Self {
            round: 0,
            worry_modulus,
            monkeys,
        }
    }

    pub fn round(&mut self) {
        for turn in 0..self.monkeys.len() {
            // dbg!(self.round, &self.monkeys[turn]);
            let throw_queue = self.monkeys[turn].throw_queue(self.worry_modulus);

            for (new_item, new_owner) in &throw_queue {
                self.monkeys[*new_owner].take(*new_item);
            };

            self.monkeys[turn].num_inspections += throw_queue.len();
            self.monkeys[turn].items = VecDeque::new();
        }

        self.round += 1;
    }

    pub fn inspections(&self) -> Vec<usize> {
        self.monkeys.iter().map(|m| m.num_inspections).collect()
    }

    pub fn monkey_business(&mut self) -> usize {
        let mut inspections = self.inspections();

        inspections.sort();
        inspections.reverse();

        inspections[0] * inspections[1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let input = include_str!("test-input-1.txt").trim();
        let monkey_strs: Vec<&str> = input.split("\n\n").collect();
        let first = monkey_strs.first().unwrap();
        let monkey = Monkey::parse(first);

        assert_eq!(monkey.items, vec![79, 98]);
    }

    #[test]
    fn part_2() {
        let input = include_str!("test-input-1.txt").trim();
        let mut game = MonkeyInTheMiddle::init(input, true);

        game.round();

        assert_eq!(game.inspections(), vec![2, 4, 3, 6]);

        for _ in 0..19 {
            game.round();
        }

        assert_eq!(game.round, 20);

        assert_eq!(game.inspections(), vec![99, 97, 8, 103]);
    }

    #[test]
    fn round() {
        let input = include_str!("test-input-1.txt").trim();
        let mut game = MonkeyInTheMiddle::init(input, false);

        for _ in 0..1 {
            game.round();
        }

        assert_eq!(game.monkeys[0].items, vec![20, 23, 27, 26]);
        assert_eq!(game.monkeys[1].items, vec![2080, 25, 167, 207, 401, 1046]);
    }

    #[test]
    fn two() {
        let input = include_str!("test-input-1.txt").trim();
        let mut game = MonkeyInTheMiddle::init(input, false);

        for _ in 0..2 {
            game.round();
        }

        assert_eq!(game.monkeys[0].items, vec![695, 10, 71, 135, 350]);
        assert_eq!(game.monkeys[1].items, vec![43, 49, 58, 55, 362]);
        assert_eq!(game.monkeys[2].items, vec![]);
    }

    #[test]
    fn multiple() {
        let input = include_str!("test-input-1.txt").trim();
        let mut game = MonkeyInTheMiddle::init(input, false);

        for _ in 0..20 {
            game.round();
        }

        assert_eq!(game.monkeys[0].items, vec![10, 12, 14, 26, 34]);
        assert_eq!(game.monkeys[1].items, vec![245, 93, 53, 199, 115]);
        assert_eq!(game.monkeys[2].items, vec![]);
    }

    #[test]
    fn monkey_business() {
        let input = include_str!("test-input-1.txt").trim();
        let mut game = MonkeyInTheMiddle::init(input, false);

        for _ in 0..20 {
            game.round();
        }

        assert_eq!(game.monkey_business(), 10605);
    }
}
