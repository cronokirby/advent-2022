use std::error::Error;

/// Represents an object which can solve a given advent instance.
pub trait Solution {
    /// Generate the answers on some data.
    fn answer(&self, data: &str) -> Result<(String, String), Box<dyn Error>>;
}
