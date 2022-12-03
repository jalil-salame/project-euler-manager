#[macro_use]
extern crate lazy_static;
use hex::FromHex;

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
    let mut md5_hash = None;
    let mut hash_comment = "is missing, consider contributing it";

    if let Some(Problem {
        id: _,
        description,
        links,
        hash,
    }) = problem
    {
        description_str = description.join("\n//");
        if !links.is_empty() {
            links_str = format!("// Visible Links\n//   {}\n", links.join("\n//   "));
        }

        md5_hash = hash.map(|s| <[u8; 16]>::from_hex(s).expect("valid hex string"));

        if hash.is_some() {
            hash_comment = "";
        }
    }

    format!(
        "// Project Euler: Problem {id}
//
// {description_str}
{links_str}
use std::fmt::Display;

/// The MD5 hash of the answer to Problem {id}{hash_comment}
pub static EXPECTED_ANSWER_HASH: Option<[u8; 16]> = {md5_hash:?};

/// Solution to Problem {id}
///
/// NOTE: Auto-generated, don't change signature
pub fn solution() -> impl Display {{
    0
}}"
    )
}
