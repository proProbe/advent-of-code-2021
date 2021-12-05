use anyhow::{anyhow, Error, Result};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
struct Board {
    size: usize,
    grid: Vec<Vec<(usize, bool)>>,
}

impl ToString for Board {
    fn to_string(&self) -> String {
        let mut repr = String::from("");
        for row in 0..self.size {
            for col in 0..self.size {
                let (v, marked) = &self.grid[row][col];
                repr.push_str(&format!("({}, {})", v, marked));
            }
            repr.push_str("|");
        }
        repr
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Column,
    Row,
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

    fn iterate_over(
        &self,
        direction: Direction,
    ) -> impl Iterator<Item = (usize, bool, (usize, usize))> + '_ {
        (0..self.size).flat_map(move |row| {
            (0..self.size).map(move |col| {
                match direction {
                    Direction::Row => {
                        let (v, b) = self.grid[row][col];
                        return (v, b, (row, col));
                    }
                    Direction::Column => {
                        let (v, b) = self.grid[col][row];
                        return (v, b, (row, col));
                    }
                };
            })
        })
    }

    fn mark(&mut self, row: usize, col: usize) {
        let (v, _) = self.grid[row][col];
        self.grid[row][col] = (v, true);
    }

    fn pos(&self, num: &usize) -> Option<(usize, usize)> {
        for (v, _, (row, col)) in self.iterate_over(Direction::Row) {
            if v == *num {
                return Some((row, col));
            }
        }

        None
    }

    fn has_bingo(&self) -> bool {
        for row in 0..self.size {
            let mut possible_bingo = true;
            for col in 0..self.size {
                let (_, marked) = &self.grid[row][col];
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
            for col in 0..self.size {
                let (_, marked) = &self.grid[col][row];
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
        self.iterate_over(Direction::Row).fold(
            0,
            |acc, (v, marked, (_, _))| {
                if marked == false {
                    acc + v
                } else {
                    acc
                }
            },
        )
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

fn part_one(boards: &Vec<Board>, inputs: &Vec<usize>) -> Result<usize, Error> {
    let mut boards = boards.clone();
    let mut bingo_board: Option<(&Board, &usize)> = None;

    for number in inputs {
        if bingo_board.is_some() {
            break;
        }

        for board in boards.iter_mut() {
            let pos = board.pos(number);
            match pos {
                Some((row, col)) => board.mark(row, col),
                None => (),
            }
        }

        bingo_board = find_bingo_board(&boards).map(|b| (b, number));
    }

    match bingo_board {
        Some((b, num)) => Ok(b.sum_unmarked() * num),
        None => Err(anyhow!("No answer for part_one")),
    }
}

fn part_two(boards: &Vec<Board>, inputs: &Vec<usize>) -> usize {
    let mut copy_boards = boards.clone();
    for number in inputs {
        for board in copy_boards.iter_mut() {
            let pos = board.pos(number);
            match pos {
                Some((row, col)) => board.mark(row, col),
                None => (),
            }
        }

        if copy_boards.len() > 1 {
            copy_boards.retain(|b| b.has_bingo() == false);
        }

        if copy_boards.len() == 1 {
            if copy_boards[0].has_bingo() {
                return copy_boards[0].sum_unmarked() * number;
            }
        }
    }
    0
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let read_data = fs::read_to_string(path).expect("Failed to read");
    let mut data = read_data.split("\n\n");

    let inputs = data
        .next()
        .map(|x| {
            x.split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .unwrap();

    let boards = data
        .map(|str| -> Board { Board::new(&str.lines().collect()) })
        .collect::<Vec<Board>>();

    Ok((
        part_one(&boards, &inputs)?,
        Some(part_two(&boards, &inputs)),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\n\
                        22 13 17 11  0\n\
                        8  2 23  4 24\n\
                        21  9 14 16  7\n\
                        6 10  3 18  5\n\
                        1 12 20 15 19\n\n\
                        3 15  0  2 22\n\
                        9 18 13 17  5\n\
                        19  8  7 25 23\n\
                        20 11 10 24  4\n\
                        14 21 16 12  6\n\n\
                        14 21 17 24  4\n\
                        10 16 15  9 19\n\
                        18  8 23 26 20\n\
                        22 11 13  6  5\n\
                        2  0 12  3  7";
    #[test]
    fn test_part_one() -> Result<()> {
        let mut data = DATA.split("\n\n");

        let inputs = data
            .next()
            .map(|x| {
                x.split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            })
            .unwrap();

        let boards = data
            .map(|str| -> Board { Board::new(&str.lines().collect()) })
            .collect::<Vec<Board>>();

        let v = part_one(&boards, &inputs)?;

        assert_eq!(v, 4512);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let mut data = DATA.split("\n\n");

        let inputs = data
            .next()
            .map(|x| {
                x.split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            })
            .unwrap();

        let boards = data
            .map(|str| -> Board { Board::new(&str.lines().collect()) })
            .collect::<Vec<Board>>();

        let v = part_two(&boards, &inputs);

        assert_eq!(v, 1924);
        Ok(())
    }

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
        assert_eq!(board.to_string(), "(1, false)(2, false)(3, false)|(4, false)(5, false)(6, false)|(10, false)(11, false)(12, false)|");
        Ok(())
    }

    #[test]
    fn test_board_mark() -> Result<()> {
        let board_rep = vec!["1 2 3", "4 5 6", "10 11 12"];
        let mut board = Board::new(&board_rep);
        board.mark(0, 0);
        board.mark(0, 1);

        assert_eq!(board.to_string(), "(1, true)(2, true)(3, false)|(4, false)(5, false)(6, false)|(10, false)(11, false)(12, false)|");
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

        assert_eq!(board.pos(&11), Some((2, 1)));
        assert_eq!(board.pos(&4), Some((1, 0)));
        Ok(())
    }
}
