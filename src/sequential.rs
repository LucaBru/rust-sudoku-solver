use crate::sudoku::Puzzle;

pub fn solve(puzzle: &mut Puzzle, row: usize, clm: usize) -> bool {
    if row == 9 {
        return true;
    }

    let (next_row, next_col) = if clm == 8 {
        (row + 1, 0)
    } else {
        (row, clm + 1)
    };

    let candidate_digits = &puzzle.get(row, clm).clone();
    if candidate_digits.len() == 1 {
        return solve(puzzle, next_row, next_col);
    }
    for num in candidate_digits.iter() {
        if puzzle.is_valid(row, clm, *num) {
            puzzle.set_at(row, clm, *num);
            if solve(puzzle, next_row, next_col) {
                return true;
            }
        }
    }

    puzzle.set_at(row, clm, 0);
    false
}
