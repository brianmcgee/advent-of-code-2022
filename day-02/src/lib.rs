use lazy_static::lazy_static;
use regex::Regex;

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
    fn parse(str: &str) -> Round {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^([A-C]) ([X-Z])$").expect("failed to compile regex");
        }
        RE
            .captures_iter(str)
            .take(1)
            .map(|captures| {
                let opp_action = captures
                    .get(1)
                    .map(|m| m.as_str())
                    .map(|str| Action::parse(str))
                    .expect("failed to parse opponent action")
                    .unwrap();

                let my_action = captures
                    .get(2)
                    .map(|m| m.as_str())
                    .map(|str| Action::parse(str))
                    .expect("failed to parse my action")
                    .unwrap();

                Round {
                    opp_action,
                    my_action,
                }
            })
            .nth(0)
            .expect("no round could be parsed")
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
        assert_eq!(Round::parse("A X"), Round {
            opp_action: Action::Rock,
            my_action: Action::Rock
        });

        assert_eq!(Round::parse("A Y"), Round {
            opp_action: Action::Rock,
            my_action: Action::Paper
        });
    }
}
