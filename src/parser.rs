use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    character::complete::{digit1, line_ending, multispace1, not_line_ending, space0, space1},
    combinator::{map, map_res, opt, verify},
    multi::{count, many0, many1},
    sequence::{delimited, pair, tuple},
    AsChar, IResult,
};
use serde::Serialize;

pub type ProblemID = u32;

#[derive(Debug, Serialize)]
pub struct Problem<'a> {
    pub id: ProblemID,
    pub description: Vec<&'a str>,
    pub links: Vec<&'a str>,
    pub hash: Option<&'a str>,
}

fn parse_problem_id(input: &str) -> IResult<&str, ProblemID> {
    let problem_id = map_res(digit1, str::parse::<ProblemID>);
    delimited(
        pair(tag("Problem"), space1),
        problem_id,
        tuple((line_ending, tag("=========="), line_ending)),
    )(input)
}

fn parse_problem_description(input: &str) -> IResult<&str, Vec<&str>> {
    let non_empty_line = delimited(
        opt(pair(space0, line_ending)),
        verify(not_line_ending, |s: &str| !s.is_empty()),
        line_ending,
    );

    delimited(
        count(pair(space0, line_ending), 2), // 2 empty lines
        many1(non_empty_line),
        count(pair(space0, line_ending), 2), // 2 empty lines
    )(input)
}

fn parse_problem_links(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, _) = delimited(space0, tag("Visible links"), line_ending)(input)?;

    many1(delimited(
        tuple((space0, digit1, tag(". "))),
        not_line_ending,
        line_ending,
    ))(input)
}

fn parse_problem_answer_hash(input: &str) -> IResult<&str, Option<&str>> {
    let hash_parser = map(
        alt((
            tag("?"),
            verify(take_while(char::is_hex_digit), |s: &str| s.len() == 32),
        )),
        |s: &str| if s == "?" { None } else { Some(s) },
    );
    delimited(pair(space0, tag("Answer: ")), hash_parser, multispace1)(input)
}

fn parse_problem(input: &str) -> IResult<&str, Problem> {
    // Problem ${id}
    // =========
    let (input, id) = parse_problem_id(input)?;

    // 2 empty lines
    // description
    // 2 empty lines
    let (input, description) = parse_problem_description(input)?;

    // Visible links ???
    // -> Links ???
    let (input, links) = opt(parse_problem_links)(input)?;
    let links = links.unwrap_or_default();

    // Answer: ${hash}?
    // 2 empty lines
    let (rest, hash) = parse_problem_answer_hash(input)?;
    Ok((
        rest,
        Problem {
            id,
            description,
            links,
            hash,
        },
    ))
}

pub fn parse_problems(input: &str) -> IResult<&str, Vec<Problem>> {
    let (input, _header) = take_until("Problem")(input)?;
    eprintln!("Read header: {_header}");
    many0(parse_problem)(input)
}

#[cfg(test)]
mod tests {
    use nom::{
        combinator::consumed,
        sequence::{pair, preceded, tuple},
    };

    use super::Problem;

    static PROBLEM_18: &str = "Problem 18
==========


   By starting at the top of the triangle below and moving to adjacent
   numbers on the row below, the maximum total from top to bottom is 23.

                                       3
                                      7 4
                                     2 4 6
                                    8 5 9 3

   That is, 3 + 7 + 4 + 9 = 23.

   Find the maximum total from top to bottom of the triangle below:

                                       75
                                     95 64
                                    17 47 82
                                  18 35 87 10
                                 20 04 82 47 65
                               19 01 23 75 03 34
                              88 02 77 73 07 63 67
                            99 65 04 28 06 16 70 92
                           41 41 26 56 83 40 80 70 33
                         41 48 72 33 47 32 37 16 94 29
                        53 71 44 65 25 43 91 52 97 51 14
                      70 11 33 28 77 73 17 78 39 68 17 57
                     91 71 52 38 17 14 91 43 58 50 27 29 48
                   63 66 04 68 89 53 67 30 73 16 69 87 40 31
                  04 62 98 27 23 09 70 98 73 93 38 53 60 04 23

   NOTE: As there are only 16384 routes, it is possible to solve this problem
   by trying every route. However, [1]Problem 67, is the same challenge with
   a triangle containing one-hundred rows; it cannot be solved by brute
   force, and requires a clever method! ;o)


   Visible links
   1. problem=67
   Answer: 708f3cf8100d5e71834b1db77dfa15d6

";
    /// Test parsing of problem id
    #[test]
    fn parse_problem_id() {
        match consumed(super::parse_problem_id)(PROBLEM_18) {
            Err(err) => panic!("Failed to parse: {err}"),
            Ok((_rest, (_consumed, id))) => assert_eq!(id, 18),
        }
    }

    /// Test parsing of problem id
    #[test]
    fn parse_problem_description() {
        match consumed(preceded(
            super::parse_problem_id,
            super::parse_problem_description,
        ))(PROBLEM_18)
        {
            Err(err) => panic!("{err}"),
            Ok((_rest, (_consumed, description))) => assert_eq!(
                description,
                vec![
                    "   By starting at the top of the triangle below and moving to adjacent",
                    "   numbers on the row below, the maximum total from top to bottom is 23.",
                    "                                       3",
                    "                                      7 4",
                    "                                     2 4 6",
                    "                                    8 5 9 3",
                    "   That is, 3 + 7 + 4 + 9 = 23.",
                    "   Find the maximum total from top to bottom of the triangle below:",
                    "                                       75",
                    "                                     95 64",
                    "                                    17 47 82",
                    "                                  18 35 87 10",
                    "                                 20 04 82 47 65",
                    "                               19 01 23 75 03 34",
                    "                              88 02 77 73 07 63 67",
                    "                            99 65 04 28 06 16 70 92",
                    "                           41 41 26 56 83 40 80 70 33",
                    "                         41 48 72 33 47 32 37 16 94 29",
                    "                        53 71 44 65 25 43 91 52 97 51 14",
                    "                      70 11 33 28 77 73 17 78 39 68 17 57",
                    "                     91 71 52 38 17 14 91 43 58 50 27 29 48",
                    "                   63 66 04 68 89 53 67 30 73 16 69 87 40 31",
                    "                  04 62 98 27 23 09 70 98 73 93 38 53 60 04 23",
                    "   NOTE: As there are only 16384 routes, it is possible to solve this problem",
                    "   by trying every route. However, [1]Problem 67, is the same challenge with",
                    "   a triangle containing one-hundred rows; it cannot be solved by brute",
                    "   force, and requires a clever method! ;o)",
                ]
            ),
        }
    }

    /// Test parsing of problem links
    #[test]
    fn parse_problem_links() {
        match consumed(preceded(
            pair(super::parse_problem_id, super::parse_problem_description),
            super::parse_problem_links,
        ))(PROBLEM_18)
        {
            Err(err) => panic!("{err}"),
            Ok((_rest, (_consumed, links))) => assert_eq!(links, vec!["problem=67"]),
        }
    }

    /// Test parsing of problem answer hash
    #[test]
    fn parse_problem_answer_hash() {
        match consumed(preceded(
            tuple((
                super::parse_problem_id,
                super::parse_problem_description,
                super::parse_problem_links,
            )),
            super::parse_problem_answer_hash,
        ))(PROBLEM_18)
        {
            Err(err) => panic!("{err}"),
            Ok((_rest, (_consumed, hash))) => {
                assert_eq!(hash, Some("708f3cf8100d5e71834b1db77dfa15d6"))
            }
        }
    }

    /// Test parsing Problem 18
    #[test]
    fn parse_problem() {
        match super::parse_problem(PROBLEM_18) {
            Err(err) => panic!("{err}"),
            Ok((
                "",
                Problem {
                    id,
                    description,
                    links,
                    hash,
                },
            )) => {
                assert_eq!(id, 18);
                assert_eq!(
                    description,
                    vec![
                    "   By starting at the top of the triangle below and moving to adjacent",
                    "   numbers on the row below, the maximum total from top to bottom is 23.",
                    "                                       3",
                    "                                      7 4",
                    "                                     2 4 6",
                    "                                    8 5 9 3",
                    "   That is, 3 + 7 + 4 + 9 = 23.",
                    "   Find the maximum total from top to bottom of the triangle below:",
                    "                                       75",
                    "                                     95 64",
                    "                                    17 47 82",
                    "                                  18 35 87 10",
                    "                                 20 04 82 47 65",
                    "                               19 01 23 75 03 34",
                    "                              88 02 77 73 07 63 67",
                    "                            99 65 04 28 06 16 70 92",
                    "                           41 41 26 56 83 40 80 70 33",
                    "                         41 48 72 33 47 32 37 16 94 29",
                    "                        53 71 44 65 25 43 91 52 97 51 14",
                    "                      70 11 33 28 77 73 17 78 39 68 17 57",
                    "                     91 71 52 38 17 14 91 43 58 50 27 29 48",
                    "                   63 66 04 68 89 53 67 30 73 16 69 87 40 31",
                    "                  04 62 98 27 23 09 70 98 73 93 38 53 60 04 23",
                    "   NOTE: As there are only 16384 routes, it is possible to solve this problem",
                    "   by trying every route. However, [1]Problem 67, is the same challenge with",
                    "   a triangle containing one-hundred rows; it cannot be solved by brute",
                    "   force, and requires a clever method! ;o)",
                ]
                );
                assert_eq!(links, vec!["problem=67"]);
                assert_eq!(hash, Some("708f3cf8100d5e71834b1db77dfa15d6"));
            }
            Ok(_) => panic!("Failed to consume input"),
        }
    }
}
