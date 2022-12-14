use anyhow::{bail, Context, Result};
use aoc_2022::days::DAYS;
use clap::Parser;

/// A CLI tool that solves Advent of Code 2022 puzzles
#[derive(Debug, Parser)]
#[command(author, about, long_about = None)]
struct Args {
    /// The day of the puzzle to solve
    #[arg(short, long)]
    day: u8,

    /// The part of the puzzle to solve
    #[arg(short, long)]
    part: u8,

    /// The input to use for the puzzle
    /// If not provided, the inputs directory will be searched for a file matching the day and part
    #[arg(short, long)]
    input: Option<String>,
}

pub fn load_example_input(day: u8, part: u8) -> Result<String> {
    let path = format!("inputs/{}/{}/example.txt", day, part);
    let input = std::fs::read_to_string(path).context(format!(
        "Failed to load example input for day {} part {}",
        day, part
    ))?;
    Ok(input)
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.part > 2 {
        bail!("Invalid part number: {}", args.part);
    }

    let day_idx = args.day - 1;

    if day_idx as usize >= DAYS.len() {
        bail!("Day {} is not implemented", args.day);
    }

    let input = match args.input {
        Some(file_path) => {
            std::fs::read_to_string(file_path).context("Failed to load input from file")?
        }
        None => load_example_input(args.day, args.part)?,
    };

    let day = &DAYS[day_idx as usize];

    if args.part == 1 {
        println!("{}", (day.part_one)(&input));
    } else {
        println!("{}", (day.part_two)(&input));
    }

    Ok(())
}
