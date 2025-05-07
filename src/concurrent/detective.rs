use core::time;
use std::{
    collections::HashMap,
    sync::mpsc::{Receiver, Sender},
    thread,
};

use super::{
    network::Network,
    rule::{Rule, RuleType},
    solver::Clue,
};

pub struct Cell {
    row: usize,
    clm: usize,
    digits: HashMap<usize, Candidates>,
    rx: Receiver<Rule>,
}

impl Cell {
    pub fn new(row: usize, clm: usize, candidates: Candidates, rx: Receiver<Rule>) -> Cell {
        let mut digits = HashMap::new();
        for digit in 1..=9 {
            digits.insert(digit, candidates.clone());
        }
        Cell {
            row: row,
            clm: clm,
            digits,
            rx: rx,
        }
    }

    fn on_discovery(&mut self, digit: usize, row: usize, clm: usize) -> Option<Rule> {
        // println!("[{}, {}] <- {:?}", self.row, self.clm, r);
        let other_cs = self.digits.remove(&digit);
        if other_cs.is_none() {
            return None;
        }
        // peer must be removed before sending, otherwise error on send
        let clue = Rule {
            type_id: RuleType::Clue,
            digit: digit,
            row: self.row,
            clm: self.clm,
        };
        other_cs.unwrap().notify(clue.clone());

        println!(
            "{} {} removing suspect at {} {} for each digit",
            self.row, self.clm, row, clm
        );

        self.digits
            .keys()
            .cloned()
            .collect::<Vec<usize>>()
            .into_iter()
            .for_each(|digit| self.remove_digit_candidate_at(digit, row, clm));

        if self.digits.len() == 1 {
            let (k, other_cs) = self.digits.iter().next().unwrap();
            let discovery = Rule {
                type_id: RuleType::Discovery,
                digit: *k,
                row: self.row,
                clm: self.clm,
            };
            other_cs.notify(discovery.clone());
            return Some(discovery);
        }

        for (digit, s) in &self.digits {
            if s.win(self.row, self.clm) {
                let discovery = Rule {
                    type_id: RuleType::Discovery,
                    digit: *digit,
                    row: self.row,
                    clm: self.clm,
                };
                self.digits[digit].notify(discovery.clone());
                return Some(discovery);
            }
        }
        return None;
    }

    fn on_hint(&mut self, digit: usize, row: usize, clm: usize) -> Option<Rule> {
        println!(
            "[{} {}] received a clue with digit {} from [{} {}]",
            self.row, self.clm, digit, row, clm
        );
        if self.digits.get(&digit).is_none() {
            return None;
        }

        self.remove_digit_candidate_at(digit, row, clm);

        if self.digits.get(&digit).unwrap().win(self.row, self.clm) {
            let discovery = Rule {
                type_id: RuleType::Discovery,
                digit: digit,
                row: self.row,
                clm: self.clm,
            };

            self.digits.get(&digit).unwrap().notify(discovery.clone());
            return Some(discovery);
        }

        return None;
    }

    pub fn investigate(&mut self, clue_tx: Sender<Rule>) {
        thread::sleep(time::Duration::from_millis(200));
        loop {
            match self.rx.recv_timeout(time::Duration::from_millis(150)) {
                Ok(rule) => {
                    println!(
                        "{} {} recvs from {} {} {:?}",
                        self.row, self.clm, rule.row, rule.clm, rule
                    );
                    let discovery = match rule.type_id {
                        RuleType::Discovery => self.on_discovery(rule.digit, rule.row, rule.clm),
                        _ => self.on_hint(rule.digit, rule.row, rule.clm),
                    };
                    if discovery.is_some() {
                        let _ = clue_tx.send(discovery.unwrap());
                        return;
                    }
                }
                Err(_) => {
                    return;
                }
            }
        }
    }

    fn remove_digit_candidate_at(&mut self, digit: usize, row: usize, clm: usize) {
        if self.row == row {
            self.digits
                .get_mut(&digit)
                .unwrap()
                .remove_row_candidate_at(clm);
        }
        if self.clm == clm {
            self.digits
                .get_mut(&digit)
                .unwrap()
                .remove_clm_candidate_at(row);
        }
        if self.row / 3 == row / 3 && self.clm / 3 == clm / 3 {
            self.digits
                .get_mut(&digit)
                .unwrap()
                .remove_box_candidate_at((row, clm));
        }
        println!(
            "{} {} remove candidate for digit {} at {} {},\nrow: {:?},\nclm: {:?},\nbox: {:?}",
            self.row,
            self.clm,
            digit,
            row,
            clm,
            self.digits.get(&digit).unwrap().row_cs,
            self.digits.get(&digit).unwrap().clm_cs,
            self.digits.get(&digit).unwrap().box_cs
        )
    }
}

#[derive(Clone)]
pub struct Candidates {
    row_cs: HashMap<usize, Option<Sender<Rule>>>,
    clm_cs: HashMap<usize, Option<Sender<Rule>>>,
    box_cs: HashMap<(usize, usize), Option<Sender<Rule>>>,
}

impl Candidates {
    pub fn new(row: usize, clm: usize, network: &Network) -> Candidates {
        let mut suspect = Candidates {
            row_cs: network.get_row_txs(row),
            clm_cs: network.get_clm_txs(clm),
            box_cs: network.get_box_txs(row, clm),
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

    fn remove_row_candidate_at(&mut self, i: usize) {
        self.row_cs.remove(&i);
    }

    fn remove_clm_candidate_at(&mut self, i: usize) {
        self.clm_cs.remove(&i);
    }

    fn remove_box_candidate_at(&mut self, i: (usize, usize)) {
        self.box_cs.remove(&i);
    }

    fn win(&self, row: usize, clm: usize) -> bool {
        println!(
            "{} {} win check {} {} {}",
            row,
            clm,
            self.row_cs.len(),
            self.clm_cs.len(),
            self.box_cs.len()
        );
        self.row_cs.len() == 0 || self.clm_cs.len() == 0 || self.box_cs.len() == 0
    }
}
