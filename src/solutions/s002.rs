use std::error::Error;

use crate::advent::Solution;

#[derive(Debug, Clone, Copy, PartialEq)]
enum ABC {
    A,
    B,
    C,
}

impl ABC {
    fn parse(data: &str) -> Result<Self, Box<dyn Error>> {
        let out = match data {
            "A" => Self::A,
            "B" => Self::B,
            "C" => Self::C,
            _ => return Err(format!("unexpected data {}", data).into()),
        };
        Ok(out)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum XYZ {
    X,
    Y,
    Z,
}

impl XYZ {
    fn parse(data: &str) -> Result<Self, Box<dyn Error>> {
        let out = match data {
            "X" => Self::X,
            "Y" => Self::Y,
            "Z" => Self::Z,
            _ => return Err(format!("unexpected data {}", data).into()),
        };
        Ok(out)
    }
}

#[derive(Debug, Clone)]
struct Input(Vec<(ABC, XYZ)>);

impl Input {
    fn parse(data: &str) -> Result<Self, Box<dyn Error>> {
        let mut out = Vec::new();
        for line in data.lines() {
            if line.len() < 3 {
                return Err("line wasn't at least 3 characters".into());
            }
            let abc = ABC::parse(&line[0..1])?;
            let xyz = XYZ::parse(&line[2..3])?;
            out.push((abc, xyz))
        }
        Ok(Input(out))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Outcome {
    Win,
    Tie,
    Lose,
}

impl Outcome {
    fn score(&self) -> u64 {
        match self {
            Outcome::Win => 6,
            Outcome::Tie => 3,
            Outcome::Lose => 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn score(&self) -> u64 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn outcome(&self, other: &Self) -> Outcome {
        use Move::*;
        use Outcome::*;

        match (*self, *other) {
            (Rock, Rock) => Tie,
            (Rock, Paper) => Lose,
            (Rock, Scissors) => Win,
            (Paper, Rock) => Win,
            (Paper, Paper) => Tie,
            (Paper, Scissors) => Lose,
            (Scissors, Rock) => Lose,
            (Scissors, Paper) => Win,
            (Scissors, Scissors) => Tie,
        }
    }

    fn make_outcome(outcome: &Outcome, against: &Self) -> Self {
        use Move::*;
        use Outcome::*;

        match (*outcome, *against) {
            (Win, Rock) => Paper,
            (Win, Paper) => Scissors,
            (Win, Scissors) => Rock,
            (Tie, Rock) => Rock,
            (Tie, Paper) => Paper,
            (Tie, Scissors) => Scissors,
            (Lose, Rock) => Scissors,
            (Lose, Paper) => Rock,
            (Lose, Scissors) => Paper,
        }
    }
}

fn abc_mapping(abc: &ABC) -> Move {
    match abc {
        ABC::A => Move::Rock,
        ABC::B => Move::Paper,
        ABC::C => Move::Scissors,
    }
}

fn xyz_mapping_move(xyz: &XYZ) -> Move {
    match xyz {
        XYZ::X => Move::Rock,
        XYZ::Y => Move::Paper,
        XYZ::Z => Move::Scissors,
    }
}

fn xyz_mapping_outcome(xyz: &XYZ) -> Outcome {
    match xyz {
        XYZ::X => Outcome::Lose,
        XYZ::Y => Outcome::Tie,
        XYZ::Z => Outcome::Win,
    }
}

fn solve_a(input: &Input) -> u64 {
    let mut sum = 0;
    for (them, me) in &input.0 {
        let their_move = abc_mapping(them);
        let my_move = xyz_mapping_move(me);
        sum += my_move.score() + my_move.outcome(&their_move).score();
    }
    sum
}

fn solve_b(input: &Input) -> u64 {
    let mut sum = 0;
    for (them, me) in &input.0 {
        let their_move = abc_mapping(them);
        let outcome = xyz_mapping_outcome(me);
        let my_move = Move::make_outcome(&outcome, &their_move);
        sum += my_move.score() + outcome.score();
    }
    sum
}

struct Solution002;

impl Solution for Solution002 {
    fn answer(&self, data: &str) -> Result<(String, String), Box<dyn Error>> {
        let input = Input::parse(data)?;
        Ok((format!("{}", solve_a(&input)), format!("{}", solve_b(&input))))
    }
}

pub fn solution() -> Box<dyn Solution> {
    Box::new(Solution002)
}
