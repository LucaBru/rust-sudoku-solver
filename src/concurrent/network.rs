use std::{
    collections::HashMap,
    fmt::Display,
    sync::mpsc::{self, Receiver, Sender},
};

use crate::{sudoku::Puzzle, utils::Matrix};

use super::restriction::Restriction;

struct Endpoints {
    tx: Option<Sender<Restriction>>,
    rx: Option<Receiver<Restriction>>,
}

impl Display for Endpoints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "endpoint")
    }
}

impl Default for Endpoints {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        Endpoints {
            tx: Some(tx),
            rx: Some(rx),
        }
    }
}

pub struct Network {
    chs: Matrix<Endpoints, 9>,
}

impl Network {
    fn new(puzzle: &Puzzle) -> Network {
        let mut chs: Matrix<Endpoints, 9> = Matrix::new();
        for r in 0..9 {
            for c in 0..9 {
                if puzzle.get(r, c).len() == 1 {
                    let endpoint = chs.get_mut(r, c);
                    endpoint.tx = None;
                    endpoint.rx = None;
                }
            }
        }
        Network { chs: chs }
    }

    pub fn take_rx_at(&mut self, row: usize, clm: usize) -> Option<Receiver<Restriction>> {
        assert!(row < 9);
        assert!(clm < 9);
        self.chs.get_mut(row, clm).rx.take()
    }

    pub fn get_row_txs(&self, row: usize) -> HashMap<usize, Option<Sender<Restriction>>> {
        assert!(row < 9);
        self.chs
            .row_neighbors_iter(row)
            .map(|endpoint| endpoint.tx.clone())
            .enumerate()
            .collect()
    }

    pub fn get_clm_txs(&self, clm: usize) -> HashMap<usize, Option<Sender<Restriction>>> {
        assert!(clm < 9);
        self.chs
            .clm_neighbors_iter(clm)
            .map(|endpoint| endpoint.tx.clone())
            .enumerate()
            .collect()
    }

    pub fn get_box_txs(
        &self,
        row: usize,
        clm: usize,
    ) -> HashMap<usize, Option<Sender<Restriction>>> {
        assert!(row < 9);
        assert!(clm < 9);
        self.chs
            .box_neighbors_iter(row, clm)
            .map(|endpoint| endpoint.tx.clone())
            .enumerate()
            .collect()
    }
}
