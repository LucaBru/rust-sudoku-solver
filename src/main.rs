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
    let boards = vec![("testSimpTechniques", [
        [0, 0, 0, 1, 0, 4, 0, 0, 0],
        [0, 0, 1, 0, 0, 0, 9, 0, 0],
        [0, 9, 0, 7, 0, 3, 0, 6, 0],
        [8, 0, 7, 0, 0, 0, 1, 0, 6],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [3, 0, 4, 0, 0, 0, 5, 0, 9],
        [0, 5, 0, 4, 0, 2, 0, 3, 0],
        [0, 0, 8, 0, 0, 0, 6, 0, 0],
        [0, 0, 0, 8, 0, 6, 0, 0, 0],
    ])];

    /*
    digit 1 suspects:
        row = [all 9 rows] it self included -> 
        clm = [all 9 clms] it self included -> 0,8 1,8
        box = [all 9 boxes] it self included -> 0,6 0,7 0,8 1,7 1,8
    */


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
        concurrent::solver::solve(concurrent_puzzle);
        msg.solution = concurrent_puzzle.to_string();
        tx.send(msg).expect("Failed to send concurrent timing");
    }

    drop(tx);
    timer_handle.join().expect("Timer thread panicked");
}
