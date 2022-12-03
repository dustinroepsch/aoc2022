use anyhow::anyhow;
use std::str::FromStr;

use super::Day;

pub const DAY_TWO: Day = Day {
    part_one: part_one,
    part_two: part_two,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

impl FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "A" => Ok(Shape::Rock),
            "B" => Ok(Shape::Paper),
            "C" => Ok(Shape::Scissors),
            "X" => Ok(Shape::Rock),
            "Y" => Ok(Shape::Paper),
            "Z" => Ok(Shape::Scissors),
            _ => Err(anyhow!("Invalid shape: {}", s)),
        }
    }
}

struct Game {
    their_move: Shape,
    my_move: Shape,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let their_move = parts.next().unwrap().parse()?;
        let my_move = parts.next().unwrap().parse()?;
        if parts.next().is_some() {
            return Err(anyhow!("Invalid game: {}", s));
        }
        Ok(Game {
            their_move,
            my_move,
        })
    }
}

impl Game {
    fn ties(&self) -> bool {
        self.their_move == self.my_move
    }

    fn i_win(&self) -> bool {
        match self.their_move {
            Shape::Rock => self.my_move == Shape::Paper,
            Shape::Paper => self.my_move == Shape::Scissors,
            Shape::Scissors => self.my_move == Shape::Rock,
        }
    }

    fn outcome_score(&self) -> i32 {
        if self.ties() {
            3
        } else if self.i_win() {
            6
        } else {
            0
        }
    }

    fn score(&self) -> i32 {
        self.my_move.score() + self.outcome_score()
    }
}

enum DesiredOutcome {
    Win,
    Tie,
    Lose,
}

impl FromStr for DesiredOutcome {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "Z" => Ok(DesiredOutcome::Win),
            "Y" => Ok(DesiredOutcome::Tie),
            "X" => Ok(DesiredOutcome::Lose),
            _ => Err(anyhow!("Invalid desired outcome: {}", s)),
        }
    }
}

pub fn part_one(input: &str) -> String {
    input
        .lines()
        .map(Game::from_str)
        .map(Result::unwrap)
        .map(|game| game.score())
        .sum::<i32>()
        .to_string()
}

pub fn part_two(input: &str) -> String {
    todo!();
}
