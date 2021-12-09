use anyhow::{Error, Result};
use std::collections::HashSet;
use std::fs;
use std::num::ParseIntError;
use std::path::Path;
use std::str::FromStr;

struct Grid {
    grid: Vec<Vec<usize>>,
}

impl FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .map(|line| -> Vec<usize> {
                line.chars()
                    .map(|c| c.to_string().parse::<usize>())
                    .collect::<Result<Vec<usize>, ParseIntError>>()
                    .unwrap_or(Vec::new())
            })
            .collect::<Vec<Vec<usize>>>();

        Ok(Grid { grid })
    }
}

fn subtract(a: usize, b: usize) -> Option<usize> {
    if b > a {
        None
    } else {
        Some(a - b)
    }
}

impl Grid {
    fn iterate(&self) -> impl Iterator<Item = (&usize, (usize, usize))> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(ri, r)| r.iter().enumerate().map(move |(ci, c)| (c, (ri, ci))))
    }

    fn check_point(&self, (r, c): (usize, usize)) -> Option<usize> {
        let row = self.grid.get(r);
        let col = row.map(|r| r.get(c)).flatten().map(|x| *x);

        col
    }

    fn neighbors(&self, (r, c): (usize, usize)) -> Vec<(usize, (usize, usize))> {
        let up = subtract(r, 1)
            .map(|v| self.check_point((v, c)))
            .flatten()
            .map(|v| (v, (r - 1, c)));
        let right = self.check_point((r, c + 1)).map(|v| (v, (r, c + 1)));
        let down = self.check_point((r + 1, c)).map(|v| (v, (r + 1, c)));

        let left = subtract(c, 1)
            .map(|v| self.check_point((r, v)))
            .flatten()
            .map(|v| (v, (r, c - 1)));

        let neighbors = [up, right, down, left]
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect::<Vec<(usize, (usize, usize))>>();

        neighbors
    }

    fn smaller_neighbors(&self, (r, c): (usize, usize)) -> Vec<(usize, (usize, usize))> {
        let neighbors = self.neighbors((r, c));
        let value = self.check_point((r, c));

        let smallest = value
            .map(move |v| {
                neighbors
                    .into_iter()
                    .filter(|(n, _)| if *n == 9 { false } else { *n >= v })
                    .collect::<Vec<(usize, (usize, usize))>>()
            })
            .unwrap_or(Vec::new());

        smallest
    }

    fn smaller_than_neighbors(&self, (r, c): (usize, usize)) -> bool {
        let neighbors = self.neighbors((r, c));
        let value = self.check_point((r, c));

        let smallest = value
            .map(|v| neighbors.iter().all(|(n, _)| v < *n))
            .unwrap_or(false);

        smallest
    }

    fn find_basin(&self, v: usize, (r, c): (usize, usize)) -> usize {
        let wall = 9;

        let mut queue = Vec::new();

        queue.push((v, (r, c)));

        let mut basin = HashSet::new();

        while queue.len() > 0 {
            let p = queue.pop();

            match p {
                Some((v, (r, c))) => {
                    let contains = basin.contains(&(v, (r, c)));
                    if v < wall && contains == false {
                        basin.insert((v, (r, c)));
                        for n in self.smaller_neighbors((r, c)) {
                            queue.push(n);
                        }
                    } else {
                        ()
                    }
                }
                None => (),
            }
        }

        basin.len()
    }
}

fn part_one(data: &str) -> Result<usize> {
    let grid = Grid::from_str(data)?;
    let n: usize = grid
        .iterate()
        .filter(|(_, (r, c))| grid.smaller_than_neighbors((*r, *c)))
        .map(|(v, _)| v + 1)
        .sum();
    Ok(n)
}

fn part_two(data: &str) -> Result<usize> {
    let grid = Grid::from_str(data)?;
    let mut basins: Vec<usize> = grid
        .iterate()
        .filter(|(_, (r, c))| grid.smaller_than_neighbors((*r, *c)))
        .map(|(v, (r, c))| grid.find_basin(*v, (r, c)))
        .collect::<Vec<usize>>();

    basins.sort_by(|a, b| b.partial_cmp(a).unwrap());

    Ok(basins.iter().take(3).product())
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let content = fs::read_to_string(path)?;

    Ok((part_one(&content)?, Some(part_two(&content)?)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "2199943210\n\
                        3987894921\n\
                        9856789892\n\
                        8767896789\n\
                        9899965678";

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(part_two(DATA)?, 1134);
        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(part_one(DATA)?, 15);
        Ok(())
    }

    #[test]
    fn test_parse() -> Result<()> {
        let grid = Grid::from_str(DATA)?;

        let g: Vec<Vec<usize>> = vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];

        assert_eq!(grid.grid, g);
        Ok(())
    }
}
