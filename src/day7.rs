use anyhow::Result;
use std::fs;
use std::num::ParseIntError;
use std::path::Path;

fn triangle_number(n: usize) -> usize {
    (1..=n).fold(0, |sum, i| sum + i)
}

// TODO: Have this function return f32 instead?
fn mean(values: &Vec<usize>) -> usize {
    let sum = values.iter().sum::<usize>();
    let n = values.len();

    sum / n
}

fn median(values: &mut Vec<usize>) -> usize {
    values.sort();
    values[values.len() / 2]
}

fn part_one(crabs: &mut Vec<usize>) -> usize {
    let target_y = median(crabs);

    crabs
        .iter()
        .map(|crab_y| -> usize {
            // TODO: how to properly take 'abs' of (usize - usize)?
            if target_y > *crab_y {
                target_y - crab_y
            } else {
                crab_y - target_y
            }
        })
        .sum::<usize>()
}

fn part_two(crabs: &mut Vec<usize>) -> usize {
    let m = mean(crabs);

    let calc = |target: usize| {
        crabs
            .iter()
            .map(|crab_y| -> usize {
                let steps = if target > *crab_y {
                    target - crab_y
                } else {
                    crab_y - target
                };
                triangle_number(steps)
            })
            .sum::<usize>()
    };

    // will have two possible values
    let fst = calc(m);
    let snd = calc(m + 1); // + 1 as mean() rounds down

    // TODO: would rather return a min(fst, snd) here :D
    if fst > snd {
        snd
    } else {
        fst
    }
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let input = fs::read_to_string(path).expect("Failed to read");
    let data = input
        .trim()
        .split(",")
        .map(|s| s.parse::<usize>())
        .collect::<Result<Vec<usize>, ParseIntError>>()?;

    Ok((
        part_one(&mut data.clone()),
        Some(part_two(&mut data.clone())),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: [usize; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(part_one(&mut DATA.to_vec()), 37);
        Ok(())
    }
    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(part_two(&mut DATA.to_vec()), 168);
        Ok(())
    }
}
