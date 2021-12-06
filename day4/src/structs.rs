use crate::Number;
use derive_more::{Deref, DerefMut};
use std::str::FromStr;

type NumberParseError = <Number as FromStr>::Err;

#[derive(Debug, PartialEq)]
pub struct Bingo {
    pub random_numbers: Vec<Number>,
    pub boards: Vec<Board>,
}

impl FromStr for Bingo {
    type Err = String;

    fn from_str(s: &str) -> Result<Bingo, Self::Err> {
        // fix for windows :'(
        let t = s
            .lines()
            .fold(String::with_capacity(s.len()), |mut acc, line| {
                acc.push_str(line);
                acc.push('\n');

                acc
            });
        let mut parts = t.split("\n\n");

        let random_numbers_txt = parts.next().ok_or(String::from(
            "invalid input, expected first line to be the random numbers",
        ))?;

        let mut boards = Vec::new();
        for board in parts {
            boards.push(Board::from_str(board).map_err(|e| e.to_string())?);
        }

        Ok(Bingo {
            random_numbers: random_numbers_txt
                .split(',')
                .try_fold::<_, _, Result<_, String>>(Vec::new(), |mut acc, x| {
                    acc.push(x.parse().map_err(|e: NumberParseError| e.to_string())?);
                    Ok(acc)
                })?,
            boards,
        })
    }
}

impl Bingo {
    pub fn play_winning(&mut self) -> Number {
        for number in self.random_numbers.iter() {
            for board in self.boards.iter_mut() {
                board.find_and_set(number);
            }

            let mut won_boards = Vec::new();
            for (i, board) in self.boards.iter().enumerate() {
                if board.have_won() {
                    won_boards.push(i)
                }
            }

            if !won_boards.is_empty() {
                if won_boards.len() > 1 {
                    panic!("muliple winning boards, please handle that :D")
                }

                let sum = self.boards[won_boards[0]].sum_of_unvisited_cells();

                return number * sum;
            }
        }

        0
    }

    pub fn play_losing(&mut self) -> Number {
        let mut board_indexes: Vec<_> = (0..self.boards.len()).collect();
        let mut last = false;
        for number in self.random_numbers.iter() {
            for board in self.boards.iter_mut() {
                board.find_and_set(number);
            }

            if last {
                let sum = self.boards[board_indexes[0]].sum_of_unvisited_cells();

                return number * sum;
            }

            for (i, board) in self.boards.iter().enumerate() {
                if board.have_won() {
                    board_indexes.retain(|x| x != &i)
                }
            }

            if board_indexes.len() == 1 {
                last = true
            }
        }

        panic!("no losing boards :thinking: ")
    }
}

#[derive(Debug, Default, PartialEq, Deref, DerefMut)]
pub struct Board {
    #[deref]
    #[deref_mut]
    board: [[Cell; 5]; 5],
}

impl Board {
    pub fn new(numbers: [[Number; 5]; 5]) -> Board {
        let mut board = Board::default();
        for i in 0..5 {
            board[i] = numbers[i].map(Cell::new);
        }
        board
    }

    pub fn init() -> Board {
        let mut board = Board::default();
        for i in 0..5 {
            board[i] = [0, 0, 0, 0, 0].map(Cell::new);
        }
        board
    }

    pub fn find_and_set(&mut self, number: &Number) {
        for line in self.board.iter_mut() {
            for cell in line.iter_mut() {
                if &cell.number == number {
                    cell.visited = true
                }
            }
        }
    }

    pub fn have_won(&self) -> bool {
        let in_a_row = (0..5).any(|i| self.board[i].iter().all(|x| x.visited));
        let in_a_column = (0..5).any(|i| self.board.iter().all(|x| x[i].visited));
        in_a_row || in_a_column
    }

    pub fn sum_of_unvisited_cells(&self) -> Number {
        self.iter()
            .flat_map(|line| line.iter())
            .filter_map(|cell| {
                if !cell.visited {
                    Some(cell.number)
                } else {
                    None
                }
            })
            .sum()
    }
}

impl FromStr for Board {
    type Err = NumberParseError;

    fn from_str(s: &str) -> Result<Board, Self::Err> {
        let mut board = Board::init();

        for (i, line) in s.trim().lines().enumerate() {
            for (j, item) in line.split_ascii_whitespace().enumerate() {
                board[i][j] = Cell::new(item.parse()?);
            }
        }

        Ok(board)
    }
}

#[derive(Debug, Default, PartialEq, Deref, DerefMut)]
pub struct Cell {
    #[deref]
    #[deref_mut]
    number: Number,
    visited: bool,
}

impl Cell {
    pub fn new(number: Number) -> Cell {
        Cell {
            number,
            visited: false,
        }
    }
}

#[test]
fn test_parsed_input() {
    let input = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"#;

    let parsed = Bingo::from_str(input).unwrap();

    assert_eq!(
        parsed.random_numbers,
        vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1
        ]
    );

    assert_eq!(parsed.boards.len(), 3);
}

#[test]
fn from_str_test() {
    let input = r#"
22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19
"#;

    let expected = Board::new([
        [22, 13, 17, 11, 0],
        [8, 2, 23, 4, 24],
        [21, 9, 14, 16, 7],
        [6, 10, 3, 18, 5],
        [1, 12, 20, 15, 19],
    ]);

    assert_eq!(expected, Board::from_str(input).unwrap());
}
