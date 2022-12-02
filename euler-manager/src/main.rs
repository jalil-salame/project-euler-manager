use std::path::Path;

use clap::Parser;
use hex::FromHex;
use project_euler_data::{Problem, ProblemID, PROBLEMS};

#[derive(Debug, Parser)]
struct Cli {
    /// The Problem to create a template solution for
    solution: ProblemID,
    /// Whether to overwrite files
    #[arg(short, long)]
    force: bool,
}

fn main() {
    let Cli { solution, force } = Cli::parse();

    let problem = PROBLEMS.iter().find(|p| p.id == solution);
    let template = rust_template(problem.cloned(), solution);

    let file = Path::new("src")
        .join("solution")
        .join(&format!("sol_{solution}.rs"));

    if file.exists() && !force {
        panic!("File exists, use --force to overwrite");
    }

    std::fs::write(&file, template).expect("write template");
}

fn rust_template(problem: Option<Problem>, id: ProblemID) -> String {
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
