use crate::advent::Solution;

mod s001;
mod s002;
mod s003;

pub fn solutions() -> Vec<Box<dyn Solution>> {
    vec![s001::solution(), s002::solution(), s003::solution()]
}
