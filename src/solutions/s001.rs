use crate::advent::Solution;

struct Solution001;

impl Solution for Solution001 {
    fn test(&self, data: &str, output: &str) -> Result<(), String> {
        Ok(())
    }

    fn answer(&self, data: &str) -> String {
        "answer".to_owned()
    }
}

pub fn solution() -> Box<dyn Solution> {
    Box::new(Solution001)
}
