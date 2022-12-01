use std::error::Error;

use crate::advent::Solution;

#[derive(Debug)]
struct Input {
    calories: Vec<Vec<u64>>,
}

impl Input {
    fn parse(data: &str) -> Self {
        let mut out = vec![Vec::new()];
        for line in data.lines() {
            if let Ok(x) = line.parse::<u64>() {
                out.last_mut().unwrap().push(x);
            } else {
                out.push(Vec::new())
            }
        }
        Input { calories: out }
    }
}

fn solve_A(input: &Input) -> Result<u64, Box<dyn Error>> {
    input.calories.iter().map(|v| v.iter().sum()).max().ok_or_else(|| "empty calories".into())
}

fn solve_B(input: &Input) -> u64 {
    let mut sums: Vec<u64> = input.calories.iter().map(|v| v.iter().sum()).collect();
    sums.sort();
    sums.reverse();
    sums[..3].iter().sum()
}

struct Solution001;

impl Solution for Solution001 {
    fn answer(&self, data: &str) -> Result<(String, String), Box<dyn Error>> {
        let input = Input::parse(data);
        let a = solve_A(&input)?;
        let b = solve_B(&input);
        Ok((format!("{}", a), format!("{}", b)))
    }
}

pub fn solution() -> Box<dyn Solution> {
    Box::new(Solution001)
}
