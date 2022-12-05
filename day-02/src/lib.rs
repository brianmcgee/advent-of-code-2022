use std::fmt::format;
use lazy_static::lazy_static;
use regex::Regex;
use crate::Action::Paper;

#[derive(Debug, PartialEq, Eq)]
enum Action {
    Rock,
    Paper,
    Scissors,
}

impl Action {
    fn parse(str: &str) -> Result<Action, String> {
        match str.chars().nth(0) {
            Some('A') => Ok(Action::Rock),
            Some('B') => Ok(Action::Paper),
            Some('C') => Ok(Action::Scissors),
            Some('X') => Ok(Action::Rock),
            Some('Y') => Ok(Action::Paper),
            Some('Z') => Ok(Action::Scissors),
            // TODO specify the invalid character in the error
            _ => Err(format!("invalid str: {}", str)),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Round {
    opp_action: Action,
    my_action: Action,
}

impl Round {
    fn parse(str: &str) -> Result<Round, String> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^([A-C]) ([X-Z])$").expect("failed to compile regex");
        }
        let regex = Regex::new(r"^([A-C]) ([X-Z])$").expect("failed to compile regex");

        regex
            .captures(str)
            .take()
            .ok_or(format!("malformed str: {}", str))
            .and_then(|captures| {

                let actions: Vec<Result<Action, String>> = captures.iter()
                    // first match is always the entire string
                    .skip(1)
                    .map(|match_opt| match_opt
                        .ok_or(format!("opponent action not found: {}", str))
                        .and_then(|m| Action::parse(m.as_str()))
                    )
                    .collect();


                match (actions.get(0), actions.get(1)) {
                    (Some(a), Some(b)) => Ok(Round { opp_action: a, my_action: b }),
                    (None, _) => Err(format!("opponent action not found")),
                    (_, None) => Err(format!("my action not found")),
                }
            })
    }
}

// fn parseLine(line: &str)
//
// fn parse(input: &Vec<&str>) -> Vec<(Action, Action)> {
//     lazy_static! {
//         static ref RE: Regex =
//             Regex::new(r"^([A-C]) ([X-Z])$").expect("failed to compile regex");
//     }
//     input
//         .iter()
//         .flat_map(|line| {
//             RE
//                 .captures_iter(line)
//                 .take(1)
//                 .map(|captures| {
//                     let first = captures
//                         .get(1)
//                         .map(|m| m.as_str())
//                         .map(|str| Action::parse(str))
//                         .expect("failed to parse opponent action")
//                         .unwrap();
//
//                     let second = captures
//                         .get(2)
//                         .map(|m| m.as_str())
//                         .map(|str| Action::parse(str))
//                         .expect("failed to parse my action")
//                         .unwrap();
//
//                     (first, second)
//                 })
//         })
//         .collect()
// }

#[cfg(test)]
mod tests {
    use crate::{Action, Round};

    #[test]
    fn action_parse() {
        for str in ["A", "X"] {
            assert_eq!(Action::parse(str), Ok(Action::Rock))
        }
        for str in ["B", "Y"] {
            assert_eq!(Action::parse(str), Ok(Action::Paper))
        }
        for str in ["C", "Z"] {
            assert_eq!(Action::parse(str), Ok(Action::Scissors))
        }

        for str in ["a", "b", "c", "x", "y", "z"] {
            let result = Action::parse(str);
            assert_eq!(result, Err(format!("invalid str: {}", str)))
        }
    }

    #[test]
    fn round_parse() {
        assert_eq!(
            Round::parse("A X"),
            Round {
                opp_action: Action::Rock,
                my_action: Action::Rock
            }
        );

        assert_eq!(
            Round::parse("A Y"),
            Round {
                opp_action: Action::Rock,
                my_action: Action::Paper
            }
        );
    }
}
