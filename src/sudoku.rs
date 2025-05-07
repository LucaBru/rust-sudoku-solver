use std::{
    collections::HashSet,
    fmt::{Display, Write},
    usize,
};

use crate::utils::Matrix;

#[derive(Debug, Default, Clone)]
pub struct Digits(HashSet<usize>);

impl Digits {
    fn is_digit(&self, digit: usize) -> bool {
        self.0.len() == 1 && *self.0.iter().next().unwrap() == digit
    }

    pub fn set(&mut self, digit: usize) {
        if digit == 0 {
            self.0 = HashSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
            return;
        }
        self.0 = HashSet::from([digit])
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &usize> {
        self.0.iter()
    }
}

impl ToString for Puzzle {
    fn to_string(&self) -> String {
        let mut output = String::new();

        for i in 0..9 {
            for j in 0..9 {
                let _ = output.write_str(
                    format!("{} ", self.board.get(i, j).iter().next().unwrap()).as_str(),
                );
            }
            output.push('\n');
        }
        output
    }
}

/* impl Display for Digits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, "[ ");
        self.0.iter().for_each(|digit| {
            let _ = write!(f, "{} ", digit);
        });
        write!(f, "]")
    }
}
 */
#[derive(Clone)]
pub struct Puzzle {
    board: Matrix<Digits, 9>,
}

impl Puzzle {
    pub fn is_valid(&self, row: usize, clm: usize, num: usize) -> bool {
        self.board
            .row_neighbors_iter(row)
            .all(|cell| !cell.is_digit(num))
            && self
                .board
                .clm_neighbors_iter(clm)
                .all(|cell| !cell.is_digit(num))
            && self
                .board
                .box_neighbors_iter(row, clm)
                .all(|(_, cell)| !cell.is_digit(num))
    }

    pub fn get(&self, row: usize, clm: usize) -> Vec<usize> {
        assert!(row < 9);
        assert!(clm < 9);
        self.board.get(row, clm).iter().cloned().collect()
    }

    pub fn set_at(&mut self, row: usize, clm: usize, num: usize) {
        assert!(row < 9);
        assert!(clm < 9);
        self.board.get_mut(row, clm).set(num);
    }

    pub fn new(sudoku: &[[usize; 9]; 9]) -> Puzzle {
        let mut puzzle = Puzzle {
            board: Matrix::new(),
        };
        for r in 0..9 {
            for c in 0..9 {
                puzzle.board.get_mut(r, c).set(sudoku[r][c]);
            }
        }
        puzzle
    }
}
