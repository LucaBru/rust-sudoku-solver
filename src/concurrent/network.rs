use std::{
    collections::HashMap,
    sync::mpsc::{self, Receiver, Sender},
};

use crate::{sudoku::Puzzle, utils::Matrix};

use super::rule::Rule;

pub struct Network {
    chs: Matrix<Option<(Sender<Rule>, Option<Receiver<Rule>>)>, 9>,
}

impl Network {
    pub fn new(puzzle: &Puzzle) -> Network {
        let mut chs = Matrix::new();
        for r in 0..9 {
            for c in 0..9 {
                if puzzle.get(r, c).len() != 1 {
                    let (tx, rx) = mpsc::channel();
                    *chs.get_mut(r, c) = Some((tx, Some(rx)));
                }
            }
        }
        Network { chs: chs }
    }

    pub fn take_rx_at(&mut self, row: usize, clm: usize) -> Option<Receiver<Rule>> {
        assert!(row < 9);
        assert!(clm < 9);
        self.chs.get_mut(row, clm).as_mut()?.1.take()
    }

    pub fn get_row_txs(&self, row: usize) -> HashMap<usize, Option<Sender<Rule>>> {
        assert!(row < 9);
        self.chs
            .row_neighbors_iter(row)
            .map(|endpoint| endpoint.as_ref().map(|(tx, _)| tx.clone()))
            .enumerate()
            .collect()
    }

    pub fn get_clm_txs(&self, clm: usize) -> HashMap<usize, Option<Sender<Rule>>> {
        assert!(clm < 9);
        self.chs
            .clm_neighbors_iter(clm)
            .map(|endpoint| endpoint.as_ref().map(|(tx, _)| tx.clone()))
            .enumerate()
            .collect()
    }

    pub fn get_box_txs(
        &self,
        row: usize,
        clm: usize,
    ) -> HashMap<(usize, usize), Option<Sender<Rule>>> {
        assert!(row < 9);
        assert!(clm < 9);
        self.chs
            .box_neighbors_iter(row, clm)
            .map(|(pos, endpoint)| (pos, endpoint.as_ref().map(|(tx, _)| tx.clone())))
            .collect()
    }
}
