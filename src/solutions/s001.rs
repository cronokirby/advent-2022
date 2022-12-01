use std::error::Error;

use crate::advent::Solution;

struct Solution001;

impl Solution for Solution001 {
    fn answer(&self, data: &str) -> Result<(String, String), Box<dyn Error>> {
        Ok(("A".to_owned(), "B".to_owned()))
    }
}

pub fn solution() -> Box<dyn Solution> {
    Box::new(Solution001)
}
