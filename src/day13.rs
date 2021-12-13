use anyhow::{anyhow, Error, Result};
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Hash, PartialEq, Copy, Eq, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

impl FromStr for Coord {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split_once(",")
            .map(Ok)
            .unwrap_or(Err(anyhow!("Bad format")))?;

        Ok(Coord {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

fn split(
    paper: HashSet<Coord>,
    (direction, line): (char, usize),
) -> (HashSet<Coord>, HashSet<Coord>) {
    let left = paper
        .clone()
        .into_iter()
        .filter(|coord| match direction {
            'x' => coord.x < line,
            'y' => coord.y < line,
            _ => true,
        })
        .collect::<HashSet<Coord>>();

    let right = paper
        .clone()
        .into_iter()
        .filter(|coord| match direction {
            'x' => coord.x > line,
            'y' => coord.y > line,
            _ => true,
        })
        .map(|v| match direction {
            'x' => Coord {
                x: 2 * line - v.x,
                y: v.y,
            },
            'y' => Coord {
                x: v.x,
                y: 2 * line - v.y,
            },
            _ => v,
        })
        .collect::<HashSet<Coord>>();

    (left, right)
}

fn fold<'a>((left, right): (&'a HashSet<Coord>, &'a HashSet<Coord>)) -> HashSet<Coord> {
    left.union(&right)
        .clone()
        .map(|v| *v)
        .collect::<HashSet<Coord>>()
}

fn paper(data: &str) -> Result<HashSet<Coord>> {
    data.trim()
        .lines()
        .map(Coord::from_str)
        .collect::<Result<HashSet<Coord>>>()
}

fn print_paper(paper: &HashSet<Coord>) -> Option<String> {
    let coords = paper
        .iter()
        .map(|coord| (coord.x, coord.y))
        .collect::<Vec<(usize, usize)>>();
    let big_x = coords.iter().map(|(x, _)| x).max()?;
    let big_y = coords.iter().map(|(_, y)| y).max()?;

    let mut code = String::new();

    for y in 0..=*big_y {
        for x in 0..=*big_x {
            let v = paper.get(&Coord { x, y });
            match v {
                Some(_) => {
                    // print!("#");
                    code.push_str("#");
                }
                None => {
                    // print!("_");
                    code.push_str("_");
                }
            }
        }
        code.push_str("\n");
        // println!("");
    }

    Some(code)
}

fn part_one(data: &str) -> Result<usize> {
    let paper = paper(data)?;
    let (left, right) = split(paper, ('x', 655));
    let v = fold((&left, &right));
    Ok(v.len())
}

fn part_two(data: &str) -> Option<String> {
    let mut paper = paper(data).unwrap();
    let inputs = [
        ('x', 655),
        ('y', 447),
        ('x', 327),
        ('y', 223),
        ('x', 163),
        ('y', 111),
        ('x', 81),
        ('y', 55),
        ('x', 40),
        ('y', 27),
        ('y', 13),
        ('y', 6),
    ];

    for input in inputs {
        let (left, right) = split(paper, input);
        paper = fold((&left, &right));
    }

    print_paper(&paper)
}

pub fn main(path: &Path) -> Result<(usize, Option<String>)> {
    let content = fs::read_to_string(path)?;

    Ok((part_one(&content)?, part_two(&content)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() -> Result<()> {
        Ok(())
    }
}
