use anyhow::{anyhow, Error, Result};
use std::fs;
use std::path::Path;

fn is_open_delimiter(delimiter: &char) -> bool {
    ['(', '[', '{', '<'].contains(delimiter)
}

fn pair_delimiter(delimiter: char) -> Result<(char, char), Error> {
    match delimiter {
        '(' => Ok(('(', ')')),
        '[' => Ok(('[', ']')),
        '{' => Ok(('{', '}')),
        '<' => Ok(('<', '>')),
        _ => Err(anyhow!("Incorrect input for delimiter")),
    }
}

fn find_faulty_delimiter(delimiters: &str) -> Result<char, Error> {
    let mut checks: Vec<(char, char)> = Vec::new();

    for c in delimiters.chars() {
        if is_open_delimiter(&c) {
            checks.push(pair_delimiter(c)?);
            continue;
        }

        let m_check = checks.pop();

        match m_check {
            Some(check) => {
                if check.1 == c {
                    continue;
                } else {
                    return Ok(c);
                }
            }
            None => return Err(anyhow!("Incorrect format")),
        }
    }

    Err(anyhow!("Nothing found error"))
}

fn missing_delimiter(delimiters: &str) -> Result<Vec<char>, Error> {
    let mut checks: Vec<(char, char)> = Vec::new();

    for c in delimiters.chars() {
        if is_open_delimiter(&c) {
            checks.push(pair_delimiter(c)?);
            continue;
        }

        let m_check = checks.pop();

        match m_check {
            Some(check) => {
                if check.1 == c {
                    continue;
                } else {
                    return Err(anyhow!("Is faulty chunk"));
                }
            }
            None => return Err(anyhow!("Incorrect format")),
        }
    }

    Ok(checks.into_iter().map(|(_, c)| c).collect::<Vec<char>>())
}

fn incorrectness(delimiter: &char) -> usize {
    match delimiter {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn missing(delimiter: &char) -> usize {
    match delimiter {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

fn part_one(data: &str) -> usize {
    let v: usize = data
        .trim()
        .lines()
        .map(|ds| find_faulty_delimiter(ds))
        .map(|fd| match fd {
            Ok(d) => incorrectness(&d),
            Err(_) => 0,
        })
        .sum();
    v
}

fn part_two(data: &str) -> usize {
    let mut v = data
        .trim()
        .lines()
        .map(|ds| missing_delimiter(ds))
        .flat_map(|fd| match fd {
            Ok(d) => vec![d.iter().rev().map(missing).fold(0, |acc, v| {
                let new_acc = acc * 5 + v;
                new_acc
            })],
            Err(_) => vec![],
        })
        .collect::<Vec<usize>>();

    v.sort();

    v[v.len() / 2]
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let content = fs::read_to_string(path)?;

    Ok((part_one(&content), Some(part_two(&content))))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_completed_delimiter() -> Result<()> {
        let DATA: Vec<&str> = vec![
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "{([(<{}[<>[]}>{[]{[(<()>",
            "(((({<>}<{<{<>}{[]{[]{}",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
            "<{([{{}}[<[[[<>{}]]]>[]]",
        ];

        let mut v = DATA
            .iter()
            .map(|ds| missing_delimiter(*ds))
            .flat_map(|fd| match fd {
                Ok(d) => vec![d.iter().rev().map(missing).fold(0, |acc, v| {
                    let new_acc = acc * 5 + v;
                    new_acc
                })],
                Err(_) => vec![],
            })
            .collect::<Vec<usize>>();

        v.sort();

        assert_eq!(v[v.len() / 2], 288957);

        Ok(())
    }

    #[test]
    fn test_find_faulty_delimiter() -> Result<()> {
        let DATA: Vec<&str> = vec![
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "{([(<{}[<>[]}>{[]{[(<()>",
            "(((({<>}<{<{<>}{[]{[]{}",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
            "<{([{{}}[<[[[<>{}]]]>[]]",
        ];

        let v: usize = DATA
            .iter()
            .map(|ds| find_faulty_delimiter(*ds))
            .map(|fd| match fd {
                Ok(d) => incorrectness(&d),
                Err(_) => 0,
            })
            .sum();

        assert_eq!(v, 26397);

        Ok(())
    }
}
