use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{HashMap, VecDeque},
    ops::Div,
};

lazy_static! {
    static ref MONKEY_REGEX: Regex = Regex::new(
        r"^Monkey (?<id>\d+):[\r\n].*(?<items>[\d, ]+)[\r\n].*(?<operation>[*+].+)[\r\n].*(?<divisible_by>\d+)[\r\n].*(?<if_true>\d+)[\r\n].*(?<if_false>\d+)"
    )
    .unwrap();
}

type WorryLevel = u32;

enum OpType {
    Add(WorryLevel),
    Multiply(WorryLevel),
    Square,
}

struct ThrowDecision {
    divisible_by: u32,
    if_true: usize,
    if_false: usize,
}

impl ThrowDecision {
    fn check(&self, item: WorryLevel) -> usize {
        if item % self.divisible_by == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

struct Monkey {
    id: usize,
    starting_items: VecDeque<WorryLevel>,
    operation: OpType,
    decision: ThrowDecision,
}

impl Monkey {
    fn parse(input: &str) -> Self {
        let captures = MONKEY_REGEX
            .captures(input)
            .expect("did not match monkey regex");

        let id = captures
            .name("id")
            .expect("expected id")
            .as_str()
            .parse::<usize>()
            .expect("expected numerical id");

        let starting_items: VecDeque<WorryLevel> = captures
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
                .parse::<u32>()
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
            .parse::<u32>()
            .expect("expected numerical divisible_by");

        let if_true = captures
            .name("if_true")
            .expect("expected if_true")
            .as_str()
            .parse::<usize>()
            .expect("expected numerical if_true");

        let if_false = captures
            .name("if_false")
            .expect("expected if_false")
            .as_str()
            .parse::<usize>()
            .expect("expected numerical if_false");

        Self {
            id,
            starting_items,
            operation,
            decision: ThrowDecision {
                divisible_by,
                if_true,
                if_false,
            },
        }
    }

    fn alter(&self, item: WorryLevel) -> WorryLevel {
        match self.operation {
            OpType::Square => item * item,
            OpType::Add(n) => item + n,
            OpType::Multiply(n) => item * n,
        }
    }

    fn take(&mut self, item: WorryLevel) {
        self.starting_items.push_back(item);
    }
}

struct MonkeyInTheMiddle {
    round: u32,
    turn: usize,
    current_monkey: Monkey,
    monkeys: VecDeque<Monkey>,
}

impl MonkeyInTheMiddle {
    fn init(script: String) -> Self {
        let mut monkeys: VecDeque<Monkey> = script.split("\n\n").map(Monkey::parse).collect();
        let current_monkey = monkeys.pop_front().unwrap();

        Self {
            round: 0,
            turn: 0,
            monkeys,
            current_monkey,
        }
    }

    fn turn(&mut self) {
        while let Some(item) = self.current_monkey.starting_items.pop_front() {
            let new_item = self.current_monkey.alter(item).div(3);
            let new_owner = self.current_monkey.decision.check(new_item);
            self.monkeys[new_owner].take(new_item);
        }

        self.monkeys.push_back(self.current_monkey);
        self.current_monkey = self.monkeys.pop_front().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let _input = include_str!("test-input-1.txt").trim();
        assert_eq!(4, 4);
    }
}
