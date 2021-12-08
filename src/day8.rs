use anyhow::{anyhow, Error, Result};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::num::ParseIntError;
use std::path::Path;
use std::str::FromStr;

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let content = fs::read_to_string(path)?;
    Ok((part_one(&content)?, Some(part_two(&content)?)))
}

#[derive(Debug, PartialEq, Eq)]
struct Input {
    patterns: [HashSet<char>; 10],
    output: [HashSet<char>; 4],
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (i, o) = s.split_once(" | ").unwrap_or(("", ""));

        let mut patterns: [HashSet<char>; 10] = Default::default();
        let mut output: [HashSet<char>; 4] = Default::default();

        for (i, v) in i.split_whitespace().enumerate() {
            patterns[i] = v.chars().collect::<HashSet<char>>();
        }

        for (i, v) in o.split_whitespace().enumerate() {
            output[i] = v.chars().collect::<HashSet<char>>();
        }

        Ok(Input { patterns, output })
    }
}

impl Input {
    fn get_1(&self) -> &HashSet<char> {
        self.patterns.iter().find(|p| p.len() == 2).unwrap()
    }
    fn get_4(&self) -> &HashSet<char> {
        self.patterns.iter().find(|p| p.len() == 4).unwrap()
    }
    fn get_7(&self) -> &HashSet<char> {
        self.patterns.iter().find(|p| p.len() == 3).unwrap()
    }
    fn get_8(&self) -> &HashSet<char> {
        self.patterns.iter().find(|p| p.len() == 7).unwrap()
    }
}

fn match_digit(segments: [char; 7], set: &HashSet<char>) -> char {
    let [a, b, c, d, e, f, g] = segments;

    let v0 = HashSet::from([a, b, c, e, f, g]);
    let v1 = HashSet::from([c, f]);
    let v2 = HashSet::from([a, c, d, e, g]);
    let v3 = HashSet::from([a, c, d, f, g]);
    let v4 = HashSet::from([b, c, d, f]);
    let v5 = HashSet::from([a, b, d, f, g]);
    let v6 = HashSet::from([a, b, d, e, f, g]);
    let v7 = HashSet::from([a, c, f]);
    let v8 = HashSet::from([a, b, c, d, e, f, g]);
    let v9 = HashSet::from([a, b, c, d, f, g]);

    let is_eq = |a: HashSet<_>, b: &HashSet<_>| -> bool {
        let x = a.symmetric_difference(b).collect::<Vec<_>>();
        x.len() == 0
    };

    if is_eq(v0, set) {
        '0'
    } else if is_eq(v1, set) {
        '1'
    } else if is_eq(v2, set) {
        '2'
    } else if is_eq(v3, set) {
        '3'
    } else if is_eq(v4, set) {
        '4'
    } else if is_eq(v5, set) {
        '5'
    } else if is_eq(v6, set) {
        '6'
    } else if is_eq(v7, set) {
        '7'
    } else if is_eq(v8, set) {
        '8'
    } else if is_eq(v9, set) {
        '9'
    } else {
        '0'
    }
}

fn decipher(input: &Input) -> Result<usize, ParseIntError> {
    let segment_count = input.patterns.iter().fold(HashMap::new(), |mut map, set| {
        for c in set.iter() {
            let seg = map.entry(c).or_insert(0);
            *seg += 1;
        }
        map
    });

    let (b, e, f) = segment_count
        .iter()
        .fold(('_', '_', '_'), |(b, e, f), (k, v)| {
            if *v == 9 {
                (b, e, **k)
            } else if *v == 6 {
                (**k, e, f)
            } else if *v == 4 {
                (b, **k, f)
            } else {
                (b, e, f)
            }
        });

    let find_c_set = HashSet::from([b, e, f]);
    let c = input
        .get_1()
        .difference(&find_c_set)
        .collect::<Vec<&char>>()[0];

    let find_a_set = HashSet::from([*c, f]);
    let a = input
        .get_7()
        .difference(&find_a_set)
        .collect::<Vec<&char>>()[0];

    let find_d_set = HashSet::from([b, *c, f]);
    let d = input
        .get_4()
        .difference(&find_d_set)
        .collect::<Vec<&char>>()[0];

    let find_g_set = HashSet::from([*a, b, *c, *d, e, f]);
    let g = input
        .get_8()
        .difference(&find_g_set)
        .collect::<Vec<&char>>()[0];

    let segments: [char; 7] = [*a, b, *c, *d, e, f, *g];

    input
        .output
        .iter()
        .map(|set| -> char { match_digit(segments, set) })
        .collect::<String>()
        .parse::<usize>()
}

fn part_two(s: &str) -> Result<usize> {
    let inputs = s
        .trim()
        .lines()
        .map(Input::from_str)
        .collect::<Result<Vec<Input>>>()?;

    Ok(inputs
        .iter()
        .map(decipher)
        .collect::<Result<Vec<usize>, ParseIntError>>()?
        .iter()
        .sum())
}

fn part_one(s: &str) -> Result<usize> {
    let inputs = s
        .trim()
        .lines()
        .map(Input::from_str)
        .collect::<Result<Vec<Input>>>()?;

    Ok(inputs
        .iter()
        .flat_map(|input| {
            input.output.iter().filter(|o| {
                let len = o.len();
                len == 2 || len == 4 || len == 3 || len == 7
            })
        })
        .count())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\n\
         be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\n\
         edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\n\
         fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\n\
         fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\n\
         aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\n\
         fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\n\
         dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\n\
         bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\n\
         egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\n\
         gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce\n\
         ";

    #[test]
    fn test_part_two() -> Result<()> {
        let n = part_two(DATA)?;

        assert_eq!(n, 61229);

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let n = part_one(DATA)?;

        assert_eq!(n, 26);

        Ok(())
    }
}
