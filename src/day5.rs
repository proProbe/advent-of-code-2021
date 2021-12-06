use anyhow::{Error, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = s.split(",");

        let x = numbers.next().unwrap().parse::<isize>()?;
        let y = numbers.next().unwrap().parse::<isize>()?;

        Ok(Point { x, y })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Hash)]
struct Vent {
    start: Point,
    end: Point,
}

impl FromStr for Vent {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points = s
            .split(" -> ")
            .map(|x: &str| Point::from_str(x).unwrap())
            .collect::<Vec<Point>>();

        let start = points[0];
        let end = points[1];

        Ok(Vent { start, end })
    }
}

impl Vent {
    fn is_acute_diagonal(&self) -> bool {
        let x_diff = self.start.x - self.end.x;
        let y_diff = self.start.y - self.end.y;

        x_diff.abs() == y_diff.abs()
    }

    fn direction(&self) -> (isize, isize) {
        let x_diff = self.end.x - self.start.x;
        let y_diff = self.end.y - self.start.y;

        (x_diff / x_diff.abs(), y_diff / y_diff.abs())
    }

    fn covers(&self, diagonal: bool) -> Vec<Point> {
        let is_y = self.start.y == self.end.y;
        if is_y {
            let range = if self.start.x < self.end.x {
                self.start.x..=self.end.x
            } else {
                self.end.x..=self.start.x
            };

            return range
                .map(|x: isize| -> Point { Point { x, y: self.start.y } })
                .collect::<Vec<Point>>();
        } else if self.start.x == self.end.x {
            let range = if self.start.y < self.end.y {
                self.start.y..=self.end.y
            } else {
                self.end.y..=self.start.y
            };

            return range
                .map(|y: isize| -> Point { Point { y, x: self.start.x } })
                .collect::<Vec<Point>>();
        } else if self.is_acute_diagonal() && diagonal {
            let (x_direction, y_direction) = self.direction();

            let x_diff = self.end.x - self.start.x;

            return (0..=x_diff.abs())
                .map(|x: isize| -> Point {
                    Point {
                        x: self.start.x + x_direction * x,
                        y: self.start.y + y_direction * x,
                    }
                })
                .collect::<Vec<Point>>();
        }

        Vec::new()
    }
}

fn calculate_n_overlaps(vents: &Vec<Vent>, diagonal: bool) -> usize {
    let mut overlaps: HashMap<Point, isize> = HashMap::new();

    for vent in vents.iter() {
        for point in vent.covers(diagonal) {
            let value = match overlaps.get(&point) {
                Some(x) => *x,
                None => 0,
            };
            overlaps.insert(point, value + 1);
        }
    }

    overlaps.values().filter(|x| *x > &1).count()
}

fn part_one(input: &str) -> usize {
    let vents: Vec<Vent> = input
        .lines()
        // TODO: How do I do question mark here instead?
        .map(|line: &str| Vent::from_str(line).unwrap())
        .collect();
    calculate_n_overlaps(&vents, false)
}

fn part_two(input: &str) -> usize {
    let vents: Vec<Vent> = input
        .lines()
        // TODO: How do I do question mark here instead?
        .map(|line: &str| Vent::from_str(line).unwrap())
        .collect();
    calculate_n_overlaps(&vents, true)
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let input = fs::read_to_string(path).expect("Failed to read");

    Ok((part_one(&input), Some(part_two(&input))))
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = "0,9 -> 5,9\n\
                        8,0 -> 0,8\n\
                        9,4 -> 3,4\n\
                        2,2 -> 2,1\n\
                        7,0 -> 7,4\n\
                        6,4 -> 2,0\n\
                        0,9 -> 2,9\n\
                        3,4 -> 1,4\n\
                        0,0 -> 8,8\n\
                        5,5 -> 8,2";

    #[test]
    fn test_calc_vent_overlaps() -> Result<()> {
        let vents: Vec<Vent> = DATA
            .lines()
            .map(|line: &str| Vent::from_str(line).unwrap())
            .collect();

        assert_eq!(calculate_n_overlaps(&vents, true), 12);

        Ok(())
    }

    #[test]
    fn test_vent_diagonal_cover() -> Result<()> {
        let s = "1,0 -> 9,8";
        let vent = Vent::from_str(s)?;

        assert_eq!(vent.is_acute_diagonal(), true);

        Ok(())
    }

    #[test]
    fn test_vent_diagonal() -> Result<()> {
        let s = "1,0 -> 4,3";
        let vent = Vent::from_str(s)?;

        assert_eq!(
            vent.covers(true),
            vec![
                Point { x: 1, y: 0 },
                Point { x: 2, y: 1 },
                Point { x: 3, y: 2 },
                Point { x: 4, y: 3 },
            ]
        );

        Ok(())
    }

    #[test]
    fn test_vent_diagonal_neg() -> Result<()> {
        let s = "4,3 -> 1,0";
        let vent = Vent::from_str(s)?;

        assert_eq!(
            vent.covers(true),
            vec![
                Point { x: 4, y: 3 },
                Point { x: 3, y: 2 },
                Point { x: 2, y: 1 },
                Point { x: 1, y: 0 },
            ]
        );

        Ok(())
    }

    #[test]
    fn test_vent_covers_y() -> Result<()> {
        let s = "1,1 -> 1,4";
        let vent = Vent::from_str(s)?;

        assert_eq!(
            vent.covers(true),
            vec![
                Point { x: 1, y: 1 },
                Point { x: 1, y: 2 },
                Point { x: 1, y: 3 },
                Point { x: 1, y: 4 }
            ]
        );

        Ok(())
    }
    #[test]
    fn test_vent_covers_y_neg() -> Result<()> {
        let s = "1,4 -> 1,1";
        let vent = Vent::from_str(s)?;

        assert_eq!(
            vent.covers(true),
            vec![
                Point { x: 1, y: 1 },
                Point { x: 1, y: 2 },
                Point { x: 1, y: 3 },
                Point { x: 1, y: 4 }
            ]
        );

        Ok(())
    }

    #[test]
    fn test_vent_covers_x() -> Result<()> {
        let s = "1,1 -> 4,1";
        let vent = Vent::from_str(s)?;

        assert_eq!(
            vent.covers(true),
            vec![
                Point { x: 1, y: 1 },
                Point { x: 2, y: 1 },
                Point { x: 3, y: 1 },
                Point { x: 4, y: 1 }
            ]
        );

        Ok(())
    }

    #[test]
    fn test_vent_covers_x_neg() -> Result<()> {
        let s = "4,1 -> 1,1";
        let vent = Vent::from_str(s)?;

        assert_eq!(
            vent.covers(true),
            vec![
                Point { x: 1, y: 1 },
                Point { x: 2, y: 1 },
                Point { x: 3, y: 1 },
                Point { x: 4, y: 1 }
            ]
        );

        Ok(())
    }
}
