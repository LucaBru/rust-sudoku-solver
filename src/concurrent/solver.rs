use crate::concurrent::detective::Candidates;
use crate::concurrent::network::Network;
use crate::parallel;
use crate::sudoku::Puzzle;
use std::sync::{Arc, mpsc};
use std::thread;

use super::detective::Cell;
use super::rule::{Rule, RuleType};

#[derive(Debug)]
pub struct Clue {
    pub row: usize,
    pub clm: usize,
    pub digit: usize,
    pub reason: RuleType,
}

pub fn solve(puzzle: &mut Puzzle) {
    let mut network = Network::new(puzzle);

    let (discovery_tx, discovery_rx) = mpsc::channel();

    for r in 0..9 {
        for c in 0..9 {
            let candidates = Candidates::new(r, c, &network);
            if puzzle.get(r, c).len() == 1 {
                println!("Singleton at {} {}", r, c);
                let rest = Rule {
                    type_id: RuleType::Discovery,
                    row: r,
                    clm: c,
                    digit: puzzle.get(r, c).get(0).unwrap().clone(),
                };
                candidates.notify(rest);
                continue;
            }
            let mut det = Cell::new(r, c, candidates, network.take_rx_at(r, c).unwrap());
            println!("thread spawned");
            let discovery_tx_clone = discovery_tx.clone();
            thread::spawn(move || det.investigate(discovery_tx_clone));
        }
    }
    drop(network);
    drop(discovery_tx);
    for clue in discovery_rx {
        puzzle.set_at(clue.row, clue.clm, clue.digit);
        println!("Found at {} {} {}", clue.row, clue.clm, clue.digit)
    }

    parallel::solve(puzzle);
}
