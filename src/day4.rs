use anyhow::{Error, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug)]
struct Board {
    size: usize,
    grid: Vec<Vec<(usize, bool)>>,
}

impl Board {
    fn new(board_rep: &Vec<&str>) -> Board {
        let size = board_rep.len();

        let grid = board_rep
            .iter()
            .map(|x| {
                x.split_whitespace()
                    .map(|y| (y.parse::<usize>().unwrap(), false))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Vec<(usize, bool)>>>();

        Board { size, grid }
    }

    fn mark(&mut self, x: usize, y: usize) {
        let (v, _) = self.grid[y][x];
        self.grid[y][x] = (v, true);
    }

    fn pos(&self, num: usize) -> Option<(usize, usize)> {
        for x in 0..self.size {
            for y in 0..self.size {
                let (v, _) = self.grid[x][y];

                if v == num {
                    return Some((y, x));
                }
            }
        }

        None
    }

    fn get_board_str(&self) -> String {
        let mut repr = String::from("");
        for x in 0..self.size {
            for y in 0..self.size {
                let (v, marked) = &self.grid[x][y];
                repr.push_str("(");
                repr.push_str(v.to_string().as_str());
                repr.push_str(", ");
                repr.push_str(marked.to_string().as_str());
                repr.push_str(")");
            }
            repr.push_str("|");
        }
        repr.clone()
    }

    fn print(&self) {
        let to_str = |(v, b)| (v, if b { 'O' } else { '_' });

        let vec_str = |v| v.iter().map(to_str).collections::<Vec<_>>();
        println!(
            "{:?} \n {:?} \n {:?} \n {:?} \n {:?}",
            self.grid[0], self.grid[1], self.grid[2], self.grid[3], self.grid[4]
        )
    }

    fn has_bingo(&self) -> bool {
        for i in 0..self.size {
            let mut possible_bingo = true;
            for j in 0..self.size {
                let (_, marked) = &self.grid[i][j];
                if possible_bingo {
                    possible_bingo = *marked;
                } else {
                    break;
                }
            }
            if possible_bingo {
                return true;
            }

            possible_bingo = true;
            for j in 0..self.size {
                let (_, marked) = &self.grid[j][i];
                if possible_bingo {
                    possible_bingo = *marked;
                } else {
                    break;
                }
            }
            if possible_bingo {
                return true;
            }
        }
        false
    }

    fn sum_unmarked(&self) -> usize {
        let mut sum = 0;
        for col in self.grid.iter() {
            for (v, marked) in col {
                if *marked == false {
                    sum += v;
                }
            }
        }
        sum
    }
}

fn find_bingo_board(boards: &Vec<Board>) -> Option<&Board> {
    let mut bingo_board = None;
    for board in boards.iter() {
        if bingo_board.is_some() {
            break;
        }

        if board.has_bingo() {
            bingo_board = Some(board);
        }
    }
    bingo_board
}

fn part_one(path: &Path) -> Result<(usize, Option<usize>)> {
    let _data = fs::read_to_string(path).expect("Failed to read");

    let mut data = _data.split("\n\n");

    let inputs = data
        .next()
        .map(|x| {
            x.split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .unwrap();

    let mut bingo_board: Option<(&Board, usize)> = None;

    let mut boards = data
        .map(|str| -> Board { Board::new(&str.lines().collect()) })
        .collect::<Vec<Board>>();

    for number in inputs {
        if bingo_board.is_some() {
            break;
        }

        for board in boards.iter_mut() {
            let pos = board.pos(number);
            match pos {
                Some((x, y)) => board.mark(x, y),
                None => (),
            }
        }

        bingo_board = find_bingo_board(&boards).map(|b| (b, number));
    }

    match bingo_board {
        Some((b, num)) => Ok((b.sum_unmarked() * num, None)),
        None => Ok((0, None)),
    }
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let _data = fs::read_to_string(path).expect("Failed to read");

    let mut data = _data.split("\n\n");

    let inputs = data
        .next()
        .map(|x| {
            x.split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .unwrap();

    let mut finished_boards: Vec<usize> = Vec::new();

    let mut boards = data
        .map(|str| -> Board { Board::new(&str.lines().collect()) })
        .collect::<Vec<Board>>();

    for number in inputs {
        println!("iter: {}", number);
        let non_bingo_boards = boards.iter_mut().filter(|b| b.has_bingo() == false);

        for board in non_bingo_boards {
            let pos = board.pos(number);
            match pos {
                Some((x, y)) => board.mark(x, y),
                None => (),
            }
        }

        let bingo_board = find_bingo_board(&boards).map(|b| (b, number));

        match bingo_board {
            Some((b, num)) => {
                b.print();
                finished_boards.push(b.sum_unmarked() * num)
            }
            None => (),
        }
    }

    let sum = finished_boards.last().unwrap();

    Ok((*sum, None))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_board_find_bingo_board() -> Result<()> {
        let board_rep1 = vec!["1 2 3", "4 5 6", "10 11 12"];
        let mut board1 = Board::new(&board_rep1);

        board1.mark(0, 0);
        board1.mark(1, 0);
        board1.mark(2, 0);

        let mut board2 = Board::new(&board_rep1);
        board2.mark(0, 0);

        assert_eq!(find_bingo_board(&vec![board1, board2]).is_some(), true);
        Ok(())
    }

    #[test]
    fn test_board_creation() -> Result<()> {
        let board_rep = vec!["1 2 3", "4 5 6", "10 11 12"];
        let board = Board::new(&board_rep);
        assert_eq!(board.get_board_str(), "(1, false)(2, false)(3, false)|(4, false)(5, false)(6, false)|(10, false)(11, false)(12, false)|");
        Ok(())
    }

    #[test]
    fn test_board_mark() -> Result<()> {
        let board_rep = vec!["1 2 3", "4 5 6", "10 11 12"];
        let mut board = Board::new(&board_rep);
        board.mark(0, 0);
        board.mark(0, 1);

        assert_eq!(board.get_board_str(), "(1, true)(2, false)(3, false)|(4, true)(5, false)(6, false)|(10, false)(11, false)(12, false)|");
        Ok(())
    }

    #[test]
    fn test_board_sum_unmarked() -> Result<()> {
        let board_rep = vec!["1 2 3", "4 5 6", "10 11 12"];
        let mut board = Board::new(&board_rep);
        board.mark(0, 0);

        assert_eq!(board.sum_unmarked(), 53);
        Ok(())
    }

    #[test]
    fn test_board_has_bingo_row() -> Result<()> {
        let board_rep = vec!["1 2 3", "4 5 6", "10 11 12"];
        let mut board = Board::new(&board_rep);
        board.mark(0, 0);
        board.mark(0, 1);
        board.mark(0, 2);

        assert_eq!(board.has_bingo(), true);
        Ok(())
    }

    #[test]
    fn test_board_has_bingo_col() -> Result<()> {
        let board_rep = vec!["1 2 3", "4 5 6", "10 11 12"];
        let mut board = Board::new(&board_rep);
        board.mark(0, 0);
        board.mark(1, 0);
        board.mark(2, 0);

        assert_eq!(board.has_bingo(), true);
        Ok(())
    }

    #[test]
    fn test_board_pos() -> Result<()> {
        let board_rep = vec!["1 2 3", "4 5 6", "10 11 12"];
        let board = Board::new(&board_rep);

        assert_eq!(board.pos(12), Some((2, 2)));
        assert_eq!(board.pos(4), Some((0, 1)));
        Ok(())
    }
}
