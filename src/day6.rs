use anyhow::Result;
use std::fs;
use std::num::ParseIntError;
use std::path::Path;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct LanternFish {
    timer: usize,
}

impl FromStr for LanternFish {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<usize>().map(|timer| LanternFish { timer })
    }
}

fn reproduce(mut fish_count: [usize; 9], ticks: usize) -> usize {
    for _ in 0..ticks {
        let births = fish_count[0];
        for i in 1..fish_count.len() {
            fish_count[i - 1] = fish_count[i];
        }
        fish_count[6] += births;
        fish_count[8] = births;
    }
    fish_count.iter().sum()
}

fn get_fish_count(fishes: &Vec<LanternFish>) -> [usize; 9] {
    fishes
        .iter()
        .fold([0, 0, 0, 0, 0, 0, 0, 0, 0], |mut acc, f| {
            acc[f.timer] += 1;
            acc
        })
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let fishes = fs::read_to_string(path)?
        .trim()
        .split(",")
        .map(LanternFish::from_str)
        .collect::<Result<Vec<LanternFish>, ParseIntError>>()?;

    let fish_count = get_fish_count(&fishes);
    Ok((reproduce(fish_count, 80), Some(reproduce(fish_count, 256))))
}

#[cfg(test)]
mod tests {
    use super::*;

    const FISH_DATA: [&str; 5] = ["3", "4", "3", "1", "2"];

    #[test]
    fn test_part_two() -> Result<()> {
        let fishes: Vec<LanternFish> = FISH_DATA
            .iter()
            .map(|f| LanternFish::from_str(f).unwrap())
            .collect();

        let n_fishes = reproduce(get_fish_count(&fishes), 256);

        assert_eq!(n_fishes, 26984457539);
        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let fishes: Vec<LanternFish> = FISH_DATA
            .iter()
            .map(|f| LanternFish::from_str(f).unwrap())
            .collect();

        let n_fishes = reproduce(get_fish_count(&fishes), 80);

        assert_eq!(n_fishes, 5934);
        Ok(())
    }

    #[test]
    fn test_fish() -> Result<()> {
        let fish = LanternFish::from_str("3")?;

        assert_eq!(fish, LanternFish { timer: 3 });
        Ok(())
    }
}
