use std::{collections::HashSet, error::Error};

use crate::advent::Solution;

type Priority = u8;

fn parse_priority(c: char) -> Result<Priority, Box<dyn Error>> {
    if c.is_ascii_lowercase() {
        Ok(c as u8 - b'a' + 1)
    } else if c.is_ascii_uppercase() {
        Ok(c as u8 - b'A' + 27)
    } else {
        Err(format!("unknown priority {}", c).into())
    }
}

#[derive(Debug)]
struct Arrangement {
    left: Vec<Priority>,
    right: Vec<Priority>,
}

impl Arrangement {
    fn parse(data: &str) -> Result<Self, Box<dyn Error>> {
        if data.len() % 2 != 0 {
            return Err("line was not even".into());
        }
        let half_len = data.len() / 2;
        let mut left = Vec::with_capacity(half_len);
        let mut right = Vec::with_capacity(half_len);
        for (i, c) in data.chars().enumerate() {
            let parsed = parse_priority(c)?;
            if i < half_len {
                left.push(parsed);
            } else {
                right.push(parsed);
            }
        }
        Ok(Arrangement { left, right })
    }

    fn element_set(&self) -> HashSet<Priority> {
        self.left.iter().chain(self.right.iter()).copied().collect()
    }
}

#[derive(Debug)]
struct Input(Vec<Arrangement>);

impl Input {
    fn parse(data: &str) -> Result<Self, Box<dyn Error>> {
        let mut out = Vec::new();
        for line in data.lines() {
            out.push(Arrangement::parse(line)?);
        }
        Ok(Input(out))
    }
}

fn solve_a(input: &Input) -> u64 {
    let mut sum = 0;
    for arr in &input.0 {
        let left_set: HashSet<u8> = HashSet::from_iter(arr.left.iter().copied());
        for r in &arr.right {
            if left_set.contains(r) {
                sum += u64::from(*r);
                break;
            }
        }
    }
    sum
}

fn solve_b(input: &Input) -> u64 {
    input
        .0
        .chunks(3)
        .map(|chunk| {
            let mut acc = chunk[0].element_set();
            for next in &chunk[1..] {
                acc = acc.intersection(&next.element_set()).copied().collect();
            }
            u64::from(acc.into_iter().next().unwrap())
        })
        .sum()
}

struct Solution003;

impl Solution for Solution003 {
    fn answer(&self, data: &str) -> Result<(String, String), Box<dyn Error>> {
        let input = Input::parse(data)?;
        let a = solve_a(&input);
        let b = solve_b(&input);
        Ok((format!("{}", a), format!("{}", b)))
    }
}

pub fn solution() -> Box<dyn Solution> {
    Box::new(Solution003)
}
