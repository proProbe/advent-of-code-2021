use anyhow::Result;
use std::fs;
use std::path::Path;

fn parse_line(line: &str) -> usize {
    line.parse::<usize>().expect("Failed to parse")
}

fn count_depth_increase(window: usize, content: &str) -> usize {
    let left = content.lines().map(parse_line);
    let right = content.lines().skip(window).map(parse_line);

    left.zip(right).filter(|(l, r)| r > l).count()
}

fn part_one(content: &str) -> usize {
    count_depth_increase(1, content)
}

fn part_two(content: &str) -> usize {
    count_depth_increase(3, content)
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let content = fs::read_to_string(path).expect("Failed to read");

    Ok((part_one(&content), Some(part_two(&content))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() -> Result<()> {
        Ok(())
    }
}
