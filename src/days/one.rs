use super::Day;

pub const DAY_ONE: Day = Day { part_one, part_two };

fn part_one(input: &str) -> String {
    let mut current_elf_score = 0;
    let mut best_elf_score = 0;

    for line in input.lines() {
        if !line.trim().is_empty() {
            current_elf_score += line.parse::<u64>().unwrap();
        } else {
            if current_elf_score > best_elf_score {
                best_elf_score = current_elf_score;
            }
            current_elf_score = 0;
        }
    }

    if current_elf_score > best_elf_score {
        best_elf_score = current_elf_score;
    }
    best_elf_score.to_string()
}

fn worst_best(top_three: &mut [u64; 3]) -> (usize, &u64) {
    top_three
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap()
}

fn part_two(input: &str) -> String {
    let mut top_three_elves = [0, 0, 0];
    let mut current_elf_score = 0;

    for line in input.lines() {
        if !line.trim().is_empty() {
            current_elf_score += line.parse::<u64>().unwrap();
        } else {
            let (worst_idx, worst_score) = worst_best(&mut top_three_elves);
            if current_elf_score > *worst_score {
                top_three_elves[worst_idx] = current_elf_score;
            }
            current_elf_score = 0;
        }
    }

    let (worst_idx, worst_score) = worst_best(&mut top_three_elves);
    if current_elf_score > *worst_score {
        top_three_elves[worst_idx] = current_elf_score;
    }

    top_three_elves.iter().sum::<u64>().to_string()
}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn test_part_one_example() {
        let example = include_str!("../../inputs/1/1/example.txt");
        let answer = part_one(example);
        assert_eq!(answer, "24000");
    }

    #[test]
    fn test_part_one_input() {
        let example = include_str!("../../inputs/1/1/input.txt");
        let answer = part_one(example);
        assert_eq!(answer, "71780");
    }
}
