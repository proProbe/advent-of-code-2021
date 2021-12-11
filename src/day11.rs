use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::Path;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    x: isize,
    y: isize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Octopus {
    energy: u32,
}

impl Octopus {
    fn charge(&mut self) {
        self.energy += 1;
        if self.energy > 9 {
            self.energy = 0;
        }
    }
}

fn neighbors(Coordinate { x, y }: &Coordinate) -> Vec<Coordinate> {
    [
        Coordinate { x: *x - 1, y: *y },
        Coordinate {
            x: *x - 1,
            y: *y - 1,
        },
        Coordinate {
            x: *x - 1,
            y: *y + 1,
        },
        Coordinate { x: *x + 1, y: *y },
        Coordinate {
            x: *x + 1,
            y: *y - 1,
        },
        Coordinate {
            x: *x + 1,
            y: *y + 1,
        },
        Coordinate { x: *x, y: *y - 1 },
        Coordinate { x: *x, y: *y + 1 },
    ]
    .into_iter()
    .filter(|coord| coord.x > -1 && coord.y > -1 && coord.x < 10 && coord.y < 10)
    .collect::<Vec<Coordinate>>()
}

fn octopuses(data: &str) -> Result<HashMap<Coordinate, Octopus>> {
    let mut octo = HashMap::new();

    for (y, line) in data.trim().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let energy = c.to_string().parse()?;
            octo.insert(
                Coordinate {
                    x: x.try_into()?,
                    y: y.try_into()?,
                },
                Octopus { energy },
            );
        }
    }

    Ok(octo)
}

fn step(state: &HashMap<Coordinate, Octopus>) -> (HashMap<Coordinate, Octopus>, usize) {
    let mut flashed = HashSet::new();
    let mut queue: VecDeque<Coordinate> = VecDeque::new();
    let mut new_state = state.clone();

    for (coord, octo) in new_state.iter_mut() {
        octo.charge();
        if octo.energy == 0 {
            flashed.insert(*coord);
            queue.push_back(*coord);
        }
    }

    while queue.is_empty() == false {
        let coord = queue.pop_back().unwrap();
        for n_coord in neighbors(&coord) {
            if flashed.contains(&n_coord) {
                continue;
            }

            let octo = new_state.entry(n_coord).or_insert(Octopus { energy: 0 });
            octo.charge();
            if octo.energy == 0 {
                flashed.insert(n_coord);
                queue.push_back(n_coord);
            }
        }
    }

    (new_state, flashed.len())
}

fn part_one(data: &str) -> Result<usize> {
    let (mut os, mut v) = step(&octopuses(data)?);
    for _ in 1..100 {
        let (os2, v2) = step(&os);
        os = os2;
        v += v2;
    }

    Ok(v)
}

fn part_two(data: &str) -> Result<usize> {
    let (mut os, mut v) = step(&octopuses(data)?);
    let mut steps = 1;
    while v != os.len() {
        steps += 1;
        let (os2, v2) = step(&os);
        os = os2;
        v = v2;
    }

    Ok(steps)
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let content = fs::read_to_string(path)?;
    Ok((part_one(&content)?, Some(part_two(&content)?)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\n\
5483143223\n\
2745854711\n\
5264556173\n\
6141336146\n\
6357385478\n\
4167524645\n\
2176841721\n\
6882881134\n\
4846848554\n\
5283751526\n\
";

    #[test]
    fn test_part_one() -> Result<()> {
        let (mut os, mut v) = step(&octopuses(DATA)?);
        for _ in 1..100 {
            let (os2, v2) = step(&os);
            os = os2;
            v += v2;
        }
        assert_eq!(v, 1656);
        Ok(())
    }
}
