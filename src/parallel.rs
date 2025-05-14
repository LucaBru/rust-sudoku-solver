use crate::{sequential, sudoku::Puzzle};
use std::{
    sync::mpsc::{self, Sender},
    thread,
};

pub fn solve(puzzle: &mut Puzzle) {
    let mut row = 0;
    let mut clm = 0;
    let mut candidates = vec![];
    'outer: for r in 0..9 {
        for c in 0..9 {
            let local_candidates = puzzle.get(r, c);
            if local_candidates.len() == 1 {
                continue;
            }
            if local_candidates.len() > candidates.len() {
                candidates = local_candidates;
                row = r;
                clm = c;
            }
            if candidates.len() == 9 {
                break 'outer;
            }
        }
    }

    if candidates.len() == 0 {
        return;
    }

    let (tx, rx) = mpsc::channel();
    candidates.iter().for_each(|num| {
        let mut puzzle_clone = puzzle.clone();
        puzzle_clone.set_at(row, clm, *num);
        let tx_clone = tx.clone();
        thread::spawn(move || try_fill(&mut puzzle_clone, tx_clone));
    });

    drop(tx);
    *puzzle = rx.recv().unwrap();
}

fn try_fill(puzzle: &mut Puzzle, tx: Sender<Puzzle>) {
    let result_found = sequential::solve(puzzle, 0, 0);
    if result_found {
        let _ = tx.send((*puzzle).clone());
    }
}
