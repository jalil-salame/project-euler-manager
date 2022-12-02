#[macro_use]
extern crate lazy_static;

mod parser;
use crate::parser::parse_problems;
pub use crate::parser::{Problem, ProblemID};

static OFFLINE_PROBLEMS_STR: &str = include_str!("../data/problems.txt");

lazy_static! {
    pub static ref PROBLEMS: Vec<Problem<'static>> = parse_problems(OFFLINE_PROBLEMS_STR)
        .expect("successful parse")
        .1;
}
