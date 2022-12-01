use crate::advent::Solution;

mod s001;

pub fn solutions() -> Vec<Box<dyn Solution>> {
    vec![s001::solution()]
}
