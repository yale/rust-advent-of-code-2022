const ROCK: i32 = 0;
const PAPER: i32 = 1;
const SCISSOR: i32 = 2;

const WIN: i32 = 6;
const DRAW: i32 = 3;
const LOSS: i32 = 0;

#[derive(Debug)]
struct Shape(i32);

#[derive(Debug)]
struct Outcome(i32);

#[derive(Debug)]
struct Round {
    player: Shape,
    opponent: Shape
}

// For part two
#[derive(Debug)]
struct RoundWithPreDeterminedOutcome {
    round: Round
}

impl Shape {
    fn score_value(self: &Self) -> i32 {
        self.0 + 1
    }

    fn beats(self: &Self, other: &Self) -> bool {
        self.0 == (other.0 + 1).rem_euclid(3)
    }

    fn needed_for(self: &Self, outcome: &Outcome) -> Shape {
        if outcome.0 == WIN {
            Shape((self.0 + 1).rem_euclid(3))
        } else if outcome.0 == LOSS {
            Shape((self.0 - 1).rem_euclid(3))
        } else {
            Shape(self.0)
        }
    }
}

impl TryFrom<char> for Shape {
    type Error = &'static str;

    fn try_from(input: char) -> Result<Shape, &'static str> {
        match input {
            'A' => Ok(Shape(ROCK)),
            'B' => Ok(Shape(PAPER)),
            'C' => Ok(Shape(SCISSOR)),
            'X' => Ok(Shape(ROCK)),
            'Y' => Ok(Shape(PAPER)),
            'Z' => Ok(Shape(SCISSOR)),
            _ => Err("Invalid input!")
        }
    }
}

impl Outcome {
    fn score_value(self: &Self) -> i32 {
        self.0
    }

}

// For Part 2
impl TryFrom<char> for Outcome {
    type Error = &'static str;

    fn try_from(input: char) -> Result<Outcome, &'static str> {
        match input {
            'X' => Ok(Outcome(LOSS)),
            'Y' => Ok(Outcome(DRAW)),
            'Z' => Ok(Outcome(WIN)),
            _ => Err("Invalid input!")
        }
    }
}

impl Round {
    fn outcome(self: &Self) -> Outcome {
        if self.player.beats(&self.opponent) {
            Outcome(WIN)
        } else if self.opponent.beats(&self.player) {
            Outcome(LOSS)
        } else {
            Outcome(DRAW)
        }
    }

    fn score(self: &Self) -> i32 {
        let outcome = self.outcome();
        outcome.score_value() + self.player.score_value()
    }
}

impl RoundWithPreDeterminedOutcome {
    fn score(self: &Self) -> i32 {
        self.round.score()
    }
}

impl TryFrom<&str> for Round {
    type Error = &'static str;

    fn try_from(input: &str) -> Result<Round, &'static str> {
        let mut chars = input.chars();
        let opponent: Shape = chars.nth(0).unwrap().try_into()?;
        let player: Shape = chars.nth(1).unwrap().try_into()?;
        Ok(Round { opponent, player })
    }
}

impl TryFrom<&str> for RoundWithPreDeterminedOutcome {
    type Error = &'static str;

    fn try_from(input: &str) -> Result<RoundWithPreDeterminedOutcome, &'static str> {
        let mut chars = input.chars();
        let opponent: Shape = chars.nth(0).unwrap().try_into()?;
        let outcome: Outcome = chars.nth(1).unwrap().try_into()?;
        let player: Shape = opponent.needed_for(&outcome);
        Ok(RoundWithPreDeterminedOutcome { round: Round { opponent, player } })
    }
}

fn score_part_one(contents: &'static str) -> i32 {
    contents
        .lines()
        .map(Round::try_from)
        .map(Result::unwrap)
        .map(|r| r.score())
        .sum()
}

fn score_part_two(contents: &'static str) -> i32 {
    contents
        .lines()
        .map(RoundWithPreDeterminedOutcome::try_from)
        .map(Result::unwrap)
        .map(|r| r.score())
        .sum()
}

fn main() {
    let contents = include_str!("input.txt");
    println!("{}", score_part_one(&contents));
    println!("{}", score_part_two(&contents));
}
