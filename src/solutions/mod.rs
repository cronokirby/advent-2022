use crate::advent::Solution;

mod s001;
mod s002;

pub fn solutions() -> Vec<Box<dyn Solution>> {
    vec![s001::solution(), s002::solution()]
}
