use std::error::Error;

use crate::advent::Solution;

struct Solution002;

impl Solution for Solution002 {
    fn answer(&self, data: &str) -> Result<(String, String), Box<dyn Error>> {
        Ok(("".into(), "".into()))
    }
}

pub fn solution() -> Box<dyn Solution> {
    Box::new(Solution002)
}