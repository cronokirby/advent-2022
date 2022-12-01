/// Represents an object which can solve a given advent instance.
pub trait Solution {
    fn test(&self, data: &str, output: &str) -> Result<(), String>;
    fn answer(&self, data: &str) -> String;
}
