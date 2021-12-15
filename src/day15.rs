use anyhow::{anyhow, Error, Result};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::Path;
use std::str::FromStr;

fn subtract(a: usize, b: usize) -> Option<usize> {
    if b > a {
        return None;
    }
    {
        Some(a - b)
    }
}

#[derive(Eq, Debug, Clone, Copy, PartialEq)]
struct Point {
    x: usize,
    y: usize,
    value: usize,
    from: Option<(usize, usize)>,
}

impl Point {
    fn coord(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

#[derive(Eq, Debug, Clone, PartialEq)]
struct Grid {
    x_size: usize,
    y_size: usize,
    grid: HashMap<(usize, usize), Point>,
}

impl Grid {
    fn neighbours(&self, (x, y): &(usize, usize)) -> Vec<Point> {
        [
            //up
            subtract(*y, 1).map(|c| self.grid.get(&(*x, c))).flatten(),
            //right
            self.grid.get(&(x + 1, *y)),
            //down
            self.grid.get(&(*x, y + 1)),
            // left
            subtract(*x, 1).map(|c| self.grid.get(&(c, *y))).flatten(),
        ]
        .iter()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .map(|p| Point {
            from: Some((*x, *y)),
            ..*p
        })
        .collect::<Vec<Point>>()
    }

    fn from_str_big(s: &str) -> Result<Self, Error> {
        let mut grid: HashMap<(usize, usize), Point> = HashMap::new();
        let trimmed = s.trim().lines();
        let x_size = trimmed
            .clone()
            .last()
            .map(|x| x.len())
            .map(Ok)
            .unwrap_or(Err(anyhow!("Bad format")))?;

        let y_size = trimmed.clone().count();

        for y_plus in 0..5 {
            for (y, line) in trimmed.clone().enumerate() {
                for x_plus in 0..5 {
                    for (x, c) in line.chars().enumerate() {
                        match c.to_digit(10) {
                            Some(d) => {
                                let mut new_val = d as usize;
                                for _ in 0..(x_plus + y_plus) {
                                    new_val += 1;
                                    if new_val > 9 {
                                        new_val = 1;
                                    }
                                }
                                let x_coord = x + x_plus * x_size;
                                let y_coord = y + y_plus * y_size;
                                grid.insert(
                                    (x_coord, y_coord),
                                    Point {
                                        x: x_coord,
                                        y: y_coord,
                                        value: new_val,
                                        from: None,
                                    },
                                );
                            }
                            None => (),
                        }
                    }
                }
            }
        }

        Ok(Grid {
            grid,
            x_size: x_size * 5 - 1,
            y_size: y_size * 5 - 1,
        })
    }
}

impl ToString for Grid {
    fn to_string(&self) -> String {
        let mut s = String::new();

        for y in 0..self.y_size {
            for x in 0..self.x_size {
                match self.grid.get(&(x, y)) {
                    Some(v) => s.push_str(&v.value.to_string()),
                    None => (),
                }
            }
            s.push_str("\n");
        }

        s
    }
}

impl FromStr for Grid {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid: HashMap<(usize, usize), Point> = HashMap::new();
        let trimmed = s.trim().lines();
        let x_size = trimmed
            .clone()
            .last()
            .map(|x| x.len())
            .map(Ok)
            .unwrap_or(Err(anyhow!("Bad format")))?
            - 1;

        let y_size = trimmed.clone().count() - 1;

        for (y, line) in trimmed.enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c.to_digit(10) {
                    Some(d) => {
                        grid.insert(
                            (x, y),
                            Point {
                                x,
                                y,
                                value: d as usize,
                                from: None,
                            },
                        );
                    }
                    None => (),
                }
            }
        }

        Ok(Grid {
            grid,
            x_size,
            y_size,
        })
    }
}

fn dijkstra(grid: Grid, from: (usize, usize), to: (usize, usize)) -> Vec<Point> {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let mut priority_queue: Vec<Point> = vec![Point {
        x: from.0,
        y: from.1,
        value: 0,
        from: None,
    }];

    let mut path = Vec::new();

    while visited.contains(&to) == false {
        let point = priority_queue.remove(0);

        if visited.contains(&point.coord()) {
            continue;
        }
        visited.insert(point.coord());
        path.push(point);
        let mut ns = grid
            .neighbours(&point.coord())
            .iter_mut()
            .map(|p| {
                p.value += point.value;
                *p
            })
            .collect::<Vec<Point>>();

        priority_queue.append(&mut ns);

        priority_queue.sort_by_key(|p| p.value);
    }

    path
}

fn part_two(data: &str) -> Result<usize> {
    let grid = Grid::from_str_big(data)?;
    let x = grid.x_size;
    let y = grid.y_size;
    let path = dijkstra(grid, (0, 0), (x, y));

    let cost = path
        .last()
        .map(|p| p.value)
        .map(Ok)
        .unwrap_or(Err(anyhow!("Did not find last path")))?;

    Ok(cost)
}

fn part_one(data: &str) -> Result<usize> {
    let grid = Grid::from_str(data)?;
    let x = grid.x_size;
    let y = grid.y_size;
    let path = dijkstra(grid, (0, 0), (x, y));

    let cost = path
        .last()
        .map(|p| p.value)
        .map(Ok)
        .unwrap_or(Err(anyhow!("Did not find last path")))?;

    Ok(cost)
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let content = fs::read_to_string(path)?;

    Ok((part_one(&content)?, Some(part_two(&content)?)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
"#;

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(40, part_one(DATA)?);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(315, part_two(DATA)?);
        Ok(())
    }
}
