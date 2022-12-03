use std::path::Path;

use clap::Parser;
use project_euler_data::{rust_template, ProblemID};

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

    let template = rust_template(solution);

    let file = Path::new("src")
        .join("solution")
        .join(&format!("sol_{solution}.rs"));

    if file.exists() && !force {
        panic!("File exists, use --force to overwrite");
    }

    std::fs::write(&file, template).expect("write template");
}
