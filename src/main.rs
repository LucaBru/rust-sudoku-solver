mod concurrent;
mod parallel;
mod sequential;
mod sudoku;
mod utils;

use std::sync::mpsc;
use std::thread;
use std::time::Instant;
use sudoku::Puzzle;
use utils::Log;

fn main() {
    utils::init_logging().expect("Failed to initialize logging");
    let boards = vec![
        ("naiveBoard", [
            [3, 1, 2, 6, 0, 5, 4, 0, 0],
            [6, 0, 4, 2, 1, 0, 0, 8, 3],
            [9, 0, 8, 0, 3, 0, 0, 2, 0],
            [2, 4, 7, 5, 6, 0, 0, 3, 0],
            [8, 6, 0, 1, 0, 0, 0, 0, 0],
            [0, 0, 5, 3, 0, 2, 6, 7, 0],
            [0, 8, 0, 0, 0, 0, 0, 0, 4],
            [0, 3, 0, 0, 0, 0, 7, 6, 2],
            [5, 0, 0, 0, 7, 0, 8, 0, 9],
        ]),
        ("testSimpTechniques", [
            [0, 0, 0, 1, 0, 4, 0, 0, 0],
            [0, 0, 1, 0, 0, 0, 9, 0, 0],
            [0, 9, 0, 7, 0, 3, 0, 6, 0],
            [8, 0, 7, 0, 0, 0, 1, 0, 6],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [3, 0, 4, 0, 0, 0, 5, 0, 9],
            [0, 5, 0, 4, 0, 2, 0, 3, 0],
            [0, 0, 8, 0, 0, 0, 6, 0, 0],
            [0, 0, 0, 8, 0, 6, 0, 0, 0],
        ]),
        ("quiteHardBoard", [
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 3, 0, 8, 5],
            [0, 0, 1, 0, 2, 0, 0, 0, 0],
            [0, 0, 0, 5, 0, 7, 0, 0, 0],
            [0, 0, 4, 0, 0, 0, 1, 0, 0],
            [0, 9, 0, 0, 0, 0, 0, 0, 0],
            [5, 0, 0, 0, 0, 0, 0, 7, 3],
            [0, 0, 2, 0, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 4, 0, 0, 0, 9],
        ]),
    ];

    let (tx, rx) = mpsc::channel();

    let timer_handle = thread::spawn(move || {
        utils::time_track(rx);
    });
    for (key, board) in boards {
        let puzzle = Puzzle::new(&board);

        let sequential_puzzle = &mut puzzle.clone();
        let mut msg = Log {
            start: Instant::now(),
            solver_design: "sequential".to_string(),
            puzzle_complexity: key.to_string(),
            solution: String::new(),
        };
        sequential::solve(sequential_puzzle, 0, 0);
        msg.solution = sequential_puzzle.to_string();
        tx.send(msg).expect("Failed to send sequential timing");

        msg = Log {
            start: Instant::now(),
            solver_design: "parallel".to_string(),
            puzzle_complexity: key.to_string(),
            solution: String::new(),
        };
        let parallel_puzzle = &mut puzzle.clone();
        parallel::solve(parallel_puzzle);
        msg.solution = parallel_puzzle.to_string();
        tx.send(msg).expect("Failed to send parallel timing");

        msg = Log {
            start: Instant::now(),
            solver_design: "concurrent".to_string(),
            puzzle_complexity: key.to_string(),
            solution: String::new(),
        };
        let concurrent_puzzle = &mut puzzle.clone();
        concurrent::concurrent::solve(concurrent_puzzle);
        msg.solution = concurrent_puzzle.to_string();
        tx.send(msg).expect("Failed to send concurrent timing");
    }

    drop(tx);
    timer_handle.join().expect("Timer thread panicked");
}
