use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn apply_rules(pair: &str, rules: &Vec<(&str, char)>) -> Option<(String, String)> {
    let (_, c) = rules.iter().find(|(rule, _)| *rule == pair)?;
    let mut left: String = String::from(&pair[0..1]);
    left.push(*c);
    let mut right: String = c.to_string();
    right.push_str(&pair[1..2]);

    Some((left, right))
}

fn setup(data: &str) -> HashMap<String, usize> {
    data.lines()
        .take(1)
        .flat_map(|x| x.chars())
        .collect::<Vec<char>>()
        .windows(2)
        .fold(HashMap::new(), |mut acc, p| {
            *acc.entry(p.iter().collect::<String>()).or_insert(0) += 1;
            acc
        })
}

fn rules(data: &str) -> Vec<(&str, char)> {
    data.trim()
        .lines()
        .skip(2)
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(pair, c)| (pair, c.chars().last().unwrap()))
        .collect::<Vec<(&str, char)>>()
}

fn step(pairs: &HashMap<String, usize>, rules: &Vec<(&str, char)>) -> HashMap<String, usize> {
    let mut new_map: HashMap<String, usize> = HashMap::new();

    for (k, v) in pairs.iter() {
        let (left, right) = apply_rules(k, &rules).unwrap();
        *new_map.entry(left).or_insert(0) += *v;
        *new_map.entry(right).or_insert(0) += *v;
    }
    new_map
}

fn iter(data: &str, n: usize) -> usize {
    let mut setup = setup(data);
    let rules = rules(data);

    for _ in 0..n {
        setup = step(&setup, &rules);
    }

    let mut counts_map =
        setup
            .iter()
            .fold(HashMap::new(), |mut acc: HashMap<char, usize>, (s, v)| {
                let first = s.chars().nth(0).unwrap();
                *acc.entry(first).or_default() += v;
                acc
            });

    *counts_map.entry('F').or_default() += 1;

    let max = counts_map.values().max().unwrap();
    let min = counts_map.values().min().unwrap();

    max - min
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let content = fs::read_to_string(path)?;

    Ok((iter(&content, 10), Some(iter(&content, 40))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() -> Result<()> {
        Ok(())
    }
}
