mod advent;
mod solutions;

use colored::Colorize;
use std::{error::Error, fs, path::PathBuf};

use advent::Solution;
use structopt::StructOpt;

use crate::solutions::solutions;

fn input_file(a: bool, day: usize) -> PathBuf {
    let ab = if a { "A" } else { "B" };
    format!("./data/test/input/{:0>3}-{}.txt", day, ab).into()
}

fn output_file(a: bool, day: usize) -> PathBuf {
    let ab = if a { "A" } else { "B" };
    format!("./data/test/output/{:0>3}-{}.txt", day, ab).into()
}

fn prompt_file(day: usize) -> PathBuf {
    format!("./data/prompt/{:0>3}.txt", day).into()
}

fn test(solution: &dyn Solution, day: usize) -> Result<(), Box<dyn Error>> {
    for &a in &[true, false] {
        let ab = if a { "A" } else { "B" };
        print!("Test {:0>3} {}: ", day, ab);
        let input = fs::read_to_string(input_file(a, day))?;
        let output = fs::read_to_string(output_file(a, day))?;
        match solution.answer(&input) {
            Err(e) => {
                println!("{} ({})", "Fail".red(), e);
            }
            Ok(x) => {
                let found = if a { x.0 } else { x.1 };
                if found == output {
                    println!("{}", "Ok".green());
                } else {
                    println!("{} (Expected `{}`, Found `{}`)", "Fail".red(), output, found);
                }
            }
        }
    }
    Ok(())
}

fn answer(solution: &dyn Solution, day: usize) -> Result<(), Box<dyn Error>> {
    println!("Answer {:0>3}:", day);
    let input = fs::read_to_string(prompt_file(day))?;
    let (a, b) = solution.answer(&input)?;
    println!("A: {}", a.blue());
    println!("B: {}", b.blue());
    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(name = "advent2022", about = "Advent of code 2022 solutions.")]
enum Opt {
    /// Run the tests for some day.
    Test {
        /// The day to test.
        day: Option<usize>,
    },
    Answer {
        /// The day to get the solution for.
        day: Option<usize>,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    let solutions = solutions();

    match opt {
        Opt::Test { day: None } => {
            for (i, solution) in solutions.iter().enumerate() {
                test((*solution).as_ref(), i + 1)?;
            }
        }
        Opt::Test { day: Some(day) } => {
            let day = day - 1;
            if day >= solutions.len() {
                println!("Day {} not implemented!", day);
            } else {
                test(&*solutions[day], day)?;
            }
        }
        Opt::Answer { day: None } => {
            for (i, solution) in solutions.iter().enumerate() {
                answer((*solution).as_ref(), i + 1)?;
            }
        }
        Opt::Answer { day: Some(day) } => {
            let day = day - 1;
            if day >= solutions.len() {
                println!("Day {} not implemented!", day);
            } else {
                answer(&*solutions[day], day)?;
            }
        }
    }

    Ok(())
}
