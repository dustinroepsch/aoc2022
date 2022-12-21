use std::collections::VecDeque;

use itertools::Itertools;

use super::Day;

pub const DAY_SIX: Day = Day { part_one, part_two };

fn part_one(input: &str) -> String {
    let mut buf = input.chars().take(4).collect::<VecDeque<_>>();
    let mut total_taken = 4;
    let mut chars = input.chars().skip(4);
    while !buf.iter().all_unique() {
        buf.pop_front();
        buf.push_back(chars.next().unwrap());
        total_taken += 1;
    }
    total_taken.to_string()
}

fn part_two(input: &str) -> String {
    let mut buf = input.chars().take(14).collect::<VecDeque<_>>();
    let mut total_taken = 14;
    let mut chars = input.chars().skip(14);
    while !buf.iter().all_unique() {
        buf.pop_front();
        buf.push_back(chars.next().unwrap());
        total_taken += 1;
    }
    total_taken.to_string()
}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn test_part_one_example() {
        let example = include_str!("../../inputs/6/1/example.txt");
        let answer = part_one(example);
        assert_eq!(answer, "7");
    }

    #[test]
    fn test_part_one_input() {
        let example = include_str!("../../inputs/6/1/input.txt");
        let answer = part_one(example);
        assert_eq!(answer, "1175");
    }

    #[test]
    fn test_part_two_example() {
        let example = include_str!("../../inputs/6/2/example.txt");
        let answer = part_two(example);
        assert_eq!(answer, "19");
    }

    #[test]
    fn test_part_two_input() {
        let example = include_str!("../../inputs/6/2/input.txt");
        let answer = part_two(example);
        assert_eq!(answer, "3217");
    }
}
