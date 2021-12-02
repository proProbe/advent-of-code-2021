use anyhow::Result;
use std::fs;
use std::path::Path;

enum Move<X> {
    Forward(X),
    Down(X),
    Up(X),
}

trait Sub {
    fn dive(&mut self, m: Move<usize>) -> ();
    fn get_position(&self) -> usize;
}

struct SubMarine2D {
    horizontal_position: usize,
    vertical_position: usize,
    aim: usize,
}

impl SubMarine2D {
    fn new(horizontal_position: usize, vertical_position: usize) -> SubMarine2D {
        SubMarine2D {
            horizontal_position,
            vertical_position,
            aim: 0,
        }
    }
}

impl Sub for SubMarine2D {
    fn dive(&mut self, m: Move<usize>) {
        match m {
            Move::Forward(x) => {
                self.horizontal_position += x;
                self.vertical_position += x * self.aim;
            }
            Move::Down(x) => self.aim += x,
            Move::Up(x) => self.aim -= x,
        }
    }

    fn get_position(&self) -> usize {
        if self.horizontal_position == 0 {
            return self.vertical_position;
        } else if self.vertical_position == 0 {
            return self.horizontal_position;
        }
        self.horizontal_position * self.vertical_position
    }
}

struct SubMarine1D {
    horizontal_position: usize,
    vertical_position: usize,
}

impl SubMarine1D {
    pub fn new(horizontal_position: usize, vertical_position: usize) -> SubMarine1D {
        SubMarine1D {
            horizontal_position,
            vertical_position,
        }
    }
}

impl Sub for SubMarine1D {
    fn dive(&mut self, m: Move<usize>) {
        match m {
            Move::Forward(x) => self.horizontal_position += x,
            Move::Down(x) => self.vertical_position += x,
            Move::Up(x) => self.vertical_position -= x,
        }
    }

    fn get_position(&self) -> usize {
        if self.horizontal_position == 0 {
            return self.vertical_position;
        } else if self.vertical_position == 0 {
            return self.horizontal_position;
        }
        self.horizontal_position * self.vertical_position
    }
}

fn dive_and_get_position(input: &str, sub: &mut impl Sub) -> usize {
    let movements = input.lines().map(|line| {
        let (direction, _distance) = line.split_once(" ").unwrap();
        let distance = _distance
            .parse::<usize>()
            .expect("Failed to parse distance");

        match direction {
            "forward" => Move::Forward(distance),
            "down" => Move::Down(distance),
            "up" => Move::Up(distance),
            _ => panic!("Invalid direction"),
        }
    });

    for m in movements {
        sub.dive(m);
    }

    sub.get_position()
}

fn part_two(input: &str) -> usize {
    let mut sub_marine = SubMarine2D::new(0, 0);
    dive_and_get_position(input, &mut sub_marine)
}

fn part_one(input: &str) -> usize {
    let mut sub_marine = SubMarine1D::new(0, 0);
    dive_and_get_position(input, &mut sub_marine)
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let input = fs::read_to_string(path).expect("Failed to read");

    Ok((part_one(&input), Some(part_two(&input))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_position() -> Result<()> {
        let sub_marine = SubMarine1D::new(0, 0);
        assert_eq!(sub_marine.get_position(), 0);
        Ok(())
    }
    #[test]
    fn test_horizontal_position() -> Result<()> {
        let sub_marine = SubMarine1D::new(1, 0);
        assert_eq!(sub_marine.get_position(), 1);
        Ok(())
    }
    #[test]
    fn test_vertical_position() -> Result<()> {
        let sub_marine = SubMarine1D::new(0, 1);
        assert_eq!(sub_marine.get_position(), 1);
        Ok(())
    }
    #[test]
    fn test_position() -> Result<()> {
        let sub_marine = SubMarine1D::new(20, 20);
        assert_eq!(sub_marine.get_position(), 400);
        Ok(())
    }
    #[test]
    fn test_go_to() -> Result<()> {
        let mut sub_marine = SubMarine1D::new(0, 20);

        sub_marine.dive(Move::Forward(20));
        assert_eq!(sub_marine.get_position(), 400);

        sub_marine.dive(Move::Up(20));
        assert_eq!(sub_marine.get_position(), 20);

        Ok(())
    }
}
