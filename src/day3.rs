use anyhow::{anyhow, Error, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn part_two(data: &Vec<&str>) -> isize {
    let decimal_data = data
        .iter()
        .map(|line| usize::from_str_radix(line, 2).unwrap());

    let n_col = data[0].len();

    let o2 = (0..n_col)
        .rev()
        .scan((0, decimal_data.clone()), |(o2, d), i| {
            let ones = d.filter(|d| has_bit(d, &i)).count();
            let half: usize = d.len() / 2;
            let maj_ones = ones > half;

            let o2_data = d
                .filter(|x| has_bit(x, &i) == maj_ones)
                .collect::<Vec<usize>>();

            o2_data.clone().first().map(|x| (x, o2_data.clone()))
        })
        .last()
        .unwrap();

    let (o2, co2, _) = (0..n_col)
        .rev()
        .scan((0, 0, decimal_data.clone()), |(o2, co2, d), i| {
            let ones = d.filter(|d| has_bit(d, &i)).count();
            let half: usize = d.len() / 2;
            let maj_ones = ones > half;

            let o2_data = d
                .filter(|x| has_bit(x, &i) == maj_ones)
                .collect::<Vec<usize>>()
                .last()
                .unwrap();

            let co2_data = d
                .filter(|x| has_bit(x, &i) != maj_ones)
                .collect::<Vec<usize>>()
                .last()
                .unwrap();

            Some((0, 0, d.filter(|x| has_bit(x, &i) == maj_ones)))
        })
        .last()
        .unwrap();

    o2 * co2
}

fn has_bit(d: &usize, i: &usize) -> bool {
    d & 1 << i > 0
}

fn part_one(data: &Vec<&str>) -> isize {
    let decimal_data = data
        .iter()
        .map(|line| usize::from_str_radix(line, 2).unwrap());

    let n_col: usize = data[0].len();

    let (gamma, epsilon) = (0..n_col).rev().fold((0, 0), |(gamma, epsilon), i| {
        let ones = decimal_data.clone().filter(|d| has_bit(d, &i)).count();
        let half = decimal_data.len() / 2;

        if ones > half {
            (gamma | 1 << i, epsilon)
        } else {
            (gamma, epsilon | 1 << i)
        }
    });

    gamma * epsilon
}

pub fn main(path: &Path) -> Result<(isize, Option<isize>)> {
    let input = fs::read_to_string(path).expect("Failed to read");
    let data = input.lines().collect::<Vec<&str>>();

    Ok((part_one(&data), Some(0)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        Ok(())
    }
}
