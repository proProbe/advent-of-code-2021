use anyhow::Result;
use std::fs;
use std::path::Path;

fn part_two(data: &Vec<&str>) -> usize {
    let decimal_data = data
        .iter()
        .map(|line| usize::from_str_radix(line, 2).unwrap());

    let n_col: usize = data[0].len();

    let mut o2: Vec<usize> = decimal_data.clone().collect();

    for i in (0..n_col).rev() {
        if o2.len() < 2 {
            break;
        }
        let ones = o2.iter().filter(|d| has_bit(d, &i)).count();
        let half = (o2.len() + 1) / 2;
        if ones >= half {
            o2.retain(|v| has_bit(v, &i));
        } else {
            o2.retain(|v| has_bit(v, &i) == false);
        }
    }

    let mut co2: Vec<usize> = decimal_data.clone().collect();

    for i in (0..n_col).rev() {
        if co2.len() < 2 {
            break;
        }
        let zeros = co2.iter().filter(|d| has_bit(d, &i) == false).count();
        let half = (co2.len() + 1) / 2;
        if zeros <= half {
            co2.retain(|v| has_bit(v, &i) == false);
        } else {
            co2.retain(|v| has_bit(v, &i));
        }
    }

    o2[0] * co2[0]
}

fn has_bit(d: &usize, i: &usize) -> bool {
    d & 1 << i > 0
}

fn part_one(data: &Vec<&str>) -> usize {
    let decimal_data = data
        .iter()
        .map(|line| usize::from_str_radix(line, 2).unwrap());

    let half = decimal_data.len() / 2;

    let n_col: usize = data[0].len();

    let (gamma, epsilon) = (0..n_col).rev().fold((0, 0), |(gamma, epsilon), i| {
        let ones = decimal_data.clone().filter(|d| has_bit(d, &i)).count();
        if ones > half {
            (gamma | 1 << i, epsilon)
        } else {
            (gamma, epsilon | 1 << i)
        }
    });

    gamma * epsilon
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let input = fs::read_to_string(path).expect("Failed to read");
    let data = input.lines().collect::<Vec<&str>>();

    Ok((part_one(&data), Some(part_two(&data))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        Ok(())
    }
}
