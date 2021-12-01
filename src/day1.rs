use anyhow::Result;
use std::fs;
use std::path::Path;

fn parse_line(line: &str) -> usize {
    line.parse::<usize>().expect("Failed to parse")
}

fn a(content: &str) -> usize {
    let left = content.lines().map(parse_line);
    let right = content.lines().skip(1).map(parse_line);

    left.zip(right).filter(|(l, r)| r > l).count()
}

fn b(content: &str) -> usize {
    let left = content.lines().map(parse_line);
    let right = content.lines().skip(3).map(parse_line);

    left.zip(right).filter(|(l, r)| r > l).count()
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let content = fs::read_to_string(path).expect("Failed to read");

    Ok((a(&content), Some(b(&content))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() -> Result<()> {
        Ok(())
    }
}
