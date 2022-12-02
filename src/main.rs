#[macro_use]
extern crate lazy_static;
use clap::{Parser, Subcommand};

mod parser;

use crate::parser::{parse_problems, Problem, ProblemID};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Run a solution
    Run {
        /// The solution to run (or all if none is specified)
        solution: Option<ProblemID>,
        /// Whether to time the solution
        #[arg(short, long)]
        time: bool,
    },
    /// Create a solution template
    Create {
        /// The Problem to create a template solution for
        solution: ProblemID,
    },
}

static OFFLINE_PROBLEMS_STR: &str = include_str!("problems.txt");
lazy_static! {
    static ref PROBLEMS: Vec<Problem<'static>> = parse_problems(OFFLINE_PROBLEMS_STR)
        .expect("successful parse")
        .1;
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Command::Run { solution, time } => todo!("run {solution:?} {time}"),
        Command::Create { solution } => {
            if let Some(Problem {
                id,
                description,
                links,
                hash,
            }) = PROBLEMS.iter().find(|p| p.id == solution)
            {
                eprintln!("Found Problem {id}");
                println!("// Project Euler: Problem {id}\n//");

                for line in description {
                    println!("//{line}");
                }

                if !links.is_empty() {
                    println!("//\n// Visible links")
                }

                for line in links {
                    println!("// {line}");
                }

                println!();
                println!(
                    "fn solution() -> impl Display {{
    0
}}"
                );
                println!();

                if let Some(hash) = hash {
                    println!(
                        "fn main() {{
    let result = format!(\"{{}}\", solution());
    let hash = todo!();
    assert_eq!(hash, \"{hash}\");
}}"
                    );
                }
            } else {
                eprintln!("Couldn't find problem {solution}");
            }
        }
    }
}
