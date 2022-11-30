use clap::{Parser, Subcommand};

mod parser;

use crate::parser::{parse_problem, ProblemID};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Generate Problems Database
    Generate,
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

static OFFLINE_PROBLEMS: &str = include_str!("problems.txt");

fn main() {
    let args = Cli::parse();

    match args.command {
        Command::Generate => {
            let _problem = parse_problem(OFFLINE_PROBLEMS);
            unimplemented!()
        }
        Command::Run { solution, time } => todo!("run {solution:?} {time}"),
        Command::Create { solution } => todo!("create {solution}"),
    }
}
