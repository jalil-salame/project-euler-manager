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

pub fn find_problem(id: ProblemID) -> Option<Problem<'static>> {
    PROBLEMS.iter().find(|problem| problem.id == id).cloned()
}

pub fn rust_template(id: ProblemID) -> String {
    let problem = find_problem(id);

    let mut description_str =
        "Note: this problem is missing from the database, consider contributing to it".to_string();
    let mut links_str = "\n".to_string();

    if let Some(Problem {
        id: _,
        description,
        links,
        hash: _,
    }) = problem
    {
        description_str = description.join("\n//");
        if !links.is_empty() {
            links_str = format!(" Visible Links\n//   {}\n", links.join("\n//   "));
        }
    }

    format!(
        "// Project Euler: Problem {id}
use std::fmt::Display;

/// Solution to Problem {id}
//
// {description_str}
//{links_str}
/// NOTE: Auto-generated, don't change signature
pub fn solution() -> impl Display {{
    0
}}"
    )
}
