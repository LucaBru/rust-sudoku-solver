use std::{cell, collections::HashMap, sync::mpsc::Sender};

use super::{network::Network, rule::Rule};

#[derive(Clone)]
pub struct Rivals {
    row_cs: HashMap<usize, Option<Sender<Rule>>>,
    clm_cs: HashMap<usize, Option<Sender<Rule>>>,
    box_cs: HashMap<(usize, usize), Option<Sender<Rule>>>,
}

impl Rivals {
    pub fn new(row: usize, clm: usize, net: &Network) -> Rivals {
        let mut suspect = Rivals {
            row_cs: net.get_row_txs(row),
            clm_cs: net.get_clm_txs(clm),
            box_cs: net.get_box_txs(row, clm),
        };
        suspect.row_cs.remove(&clm);
        suspect.clm_cs.remove(&row);
        suspect.box_cs.remove(&(row, clm));
        suspect
    }

    pub fn notify(&self, msg: Rule) {
        self.row_cs
            .values()
            .chain(self.clm_cs.values())
            .chain(self.box_cs.values())
            .filter(|ch| ch.is_some())
            .for_each(|ch| {
                let _ = ch.as_ref().unwrap().send(msg.clone());
            });
    }

    pub fn lose(&self) -> bool {
        self.row_cs.len() == 0 || self.clm_cs.len() == 0 || self.box_cs.len() == 0
    }

    pub fn remove_at(&mut self, row: usize, clm: usize, cell_row: usize, cell_clm: usize) {
        if row == cell_row {
            self.row_cs.remove(&clm);
        }
        if clm == cell_clm {
            self.clm_cs.remove(&row);
        }
        self.box_cs.remove(&(row, clm));
    }
}
