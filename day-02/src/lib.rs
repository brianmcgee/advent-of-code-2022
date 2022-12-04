use std::fmt;

enum Action {
    Rock,
    Paper,
    Scissors,
}

impl Action {
    fn parse(char: &char) -> Result<Action, &'static str> {
        match char {
            'A' => Ok(Action::Rock),
            'B' => Ok(Action::Paper),
            'C' => Ok(Action::Scissors),
            'X' => Ok(Action::Rock),
            'Y' => Ok(Action::Paper),
            'Z' => Ok(Action::Scissors),
            // TODO specify the invalid character in the error
            _ => Err("invalid character"),
        }
    }
}

struct Parser {}

impl Parser {
    fn parse(input: &str) {
        input
            .lines()
            .map(|line| line.split(" ").collect())
            .map(|entries: Vec<&str>| {
                if x.len() == 2 {
                    Ok(entries)
                } else {
                    Err(format!(
                        "only 2 entries expected per line. found {}",
                        entries.len()
                    ))
                }
            })
            .map(|r| r.clone));

        input.split(" ").map(|x| {
            if x.len() == 2 {
                (x[0], x[1])
            } else {
                panic!("only 2 entries per line: {}", x)
            }
        });
    }
}
