use nom::IResult;
use serde::Serialize;

pub type ProblemID = u32;

#[derive(Debug, Serialize)]
pub struct Problem<'a> {
    pub id: ProblemID,
    pub description: &'a str,
    pub hash: Option<&'a str>,
}

pub fn parse_problem(_input: &str) -> IResult<&str, Problem> {
    // Problem ${id}
    // =========
    // 2 empty lines
    // description
    // 2 empty lines
    // Visible links ???
    // -> Links ???
    // Answer: ${hash}?
    // 2 empty lines
    todo!()
}
