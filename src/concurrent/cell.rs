use core::time;
use std::{
    collections::HashMap,
    sync::mpsc::{Receiver, Sender},
    thread,
};

use super::{
    rivals::Rivals,
    rule::{Rule, RuleType},
};

pub struct Cell {
    row: usize,
    clm: usize,
    digits: HashMap<usize, Rivals>,
    rx: Receiver<Rule>,
}

impl Cell {
    pub fn new(row: usize, clm: usize, rivals: Rivals, rx: Receiver<Rule>) -> Cell {
        let mut digits = HashMap::new();
        for digit in 1..=9 {
            digits.insert(digit, rivals.clone());
        }
        Cell {
            row: row,
            clm: clm,
            digits,
            rx: rx,
        }
    }

    fn on_discovery(&mut self, digit: usize, row: usize, clm: usize) -> Option<usize> {
        let rivals = self.digits.remove(&digit)?;
        if self.digits.len() == 1 {
            let (k, _) = self.digits.iter().next().unwrap();
            return Some(*k);
        }
        // peer must be removed before sending, otherwise error on send
        for (digit, rival) in self.digits.iter_mut() {
            rival.remove_at(row, clm, self.row, self.clm);
            if rival.lose() {
                return Some(*digit);
            }
        }

        let clue = Rule {
            type_id: RuleType::Clue,
            digit: digit,
            row: self.row,
            clm: self.clm,
        };
        rivals.notify(clue.clone());
        return None;
    }

    fn on_hint(&mut self, digit: usize, row: usize, clm: usize) -> Option<usize> {
        let rivals = self.digits.get_mut(&digit)?;
        rivals.remove_at(row, clm, self.row, self.clm);
        if rivals.lose() {
            return Some(digit);
        }
        return None;
    }

    pub fn investigate(&mut self, digit_tx: Sender<Rule>) {
        thread::sleep(time::Duration::from_millis(200));
        loop {
            match self.rx.recv_timeout(time::Duration::from_millis(150)) {
                Ok(rule) => {
                    println!(
                        "{} {} recvs from {} {} {:?}",
                        self.row, self.clm, rule.row, rule.clm, rule
                    );
                    let digit = match rule.type_id {
                        RuleType::Discovery => self.on_discovery(rule.digit, rule.row, rule.clm),
                        _ => self.on_hint(rule.digit, rule.row, rule.clm),
                    };
                    if let Some(digit) = digit {
                        let discovery = Rule {
                            digit: digit,
                            clm: self.clm,
                            row: self.row,
                            type_id: RuleType::Discovery,
                        };

                        self.digits
                            .get(&discovery.digit)
                            .unwrap()
                            .notify(discovery.clone());
                        let _ = digit_tx.send(discovery);
                        return;
                    }
                }
                Err(_) => {
                    return;
                }
            }
        }
    }
}
