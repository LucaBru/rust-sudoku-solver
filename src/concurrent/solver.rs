use crate::concurrent::network::Network;
use crate::concurrent::restriction::Discovery;
use crate::parallel;
use crate::sudoku::Puzzle;
use std::sync::mpsc;
use std::thread;
/*
pub fn solve(puzzle: &mut impl Puzzle) {
    let mut network = Network::init(puzzle);
    let (sol_tx, sol_rx) = mpsc::channel::<bool>();
    let mut threads = 0;
    for r in 0..9 {
        for c in 0..9 {
            let peers = Peers::new(&network, r, c);
            if let Some(x) = puzzle.digit_at(r, c) {
                peers.notify(Box::new(Discovery {
                    row: r,
                    clm: c,
                    digit: x,
                }));
                continue;
            }
            let rx_res = network.get_rx_at(r, c).unwrap();
            let sol_tx_clone = sol_tx.clone();
            thread::spawn(move || investigate(r, c, peers, sol_tx_clone, rx_res));
            threads += 1;
        }
    }
    drop(network);
    let mut found = false;
    for _ in 0..threads {
        found = found || sol_rx.recv().unwrap();
    }
    if found == false {
        parallel::solve(puzzle);
    }
}
 */
