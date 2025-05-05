use std::{
    collections::HashMap,
    sync::mpsc::{Receiver, Sender},
};

use super::{network::Network, restriction::Restriction};

struct Detective {
    row: usize,
    clm: usize,
    sol_tx: Sender<bool>,
}

impl Detective {
    fn get_box_neighbor_idx(&self, msg: &Restriction) -> usize {
        let upper_row = self.row / 3 * 3;
        let leftmost_clm = self.clm / 3 * 3;
        (msg.row - upper_row) * 3 + msg.clm - leftmost_clm
    }
}

enum Party {
    Row,
    Clm,
}

pub fn investigate(row: usize, clm: usize, network: &Network, sol_tx: Sender<bool>) {
    let voters = HashMap::new();
    let row_voters = network.get_row_txs(row);
    let clm_voters = network.get_clm_txs(clm);
    let box_voters = network.get_box_txs(row, clm);

    /*
    to continue
     */

    let mut candidates = HashMap::<usize, Peers>::new();
    for x in 1..=9 {
        candidates.insert(x, peers.clone());
    }

    for res in rx_restriction {
        if let Some(discovery) = res.as_any().downcast_ref::<Discovery>() {
            let discovery_peers = candidates.remove(&discovery.digit);
            if discovery_peers.is_none() {
                continue;
            }
            // peer must be removed before sending, otherwise error on send
            let box_row = (discovery.row - discovery.row / 3 * 3) * 3;
            let box_clm = (discovery.clm - discovery.clm / 3 * 3) % 3;
            candidates.values_mut().for_each(|voters| {
                voters.remove_peer(Peer::Row, discovery.clm);
                voters.remove_peer(Peer::Clm, discovery.row);
                voters.remove_peer(Peer::Box, box_row + box_clm);
            });

            let clue = Box::new(Clue {
                digit: discovery.digit,
                row: row,
                clm: clm,
            });
            discovery_peers.unwrap().notify(clue);

            if candidates.len() == 1 {
                let k = *candidates.keys().next().unwrap();
                candidates.get(&k).unwrap().notify(Box::new(Discovery {
                    digit: k,
                    row: row,
                    clm: clm,
                }));
                sol_tx.send(true).unwrap();
                return;
            }

            for (digit, voters) in &candidates {
                if !voters.has_peer(Peer::Row)
                    || !voters.has_peer(Peer::Clm)
                    || !voters.has_peer(Peer::Box)
                {
                    voters.notify(Box::new(Discovery {
                        digit: *digit,
                        row: row,
                        clm: clm,
                    }));
                    sol_tx.send(true).unwrap();
                    return;
                }
            }
        }
        if let Some(clue) = res.as_any().downcast_ref::<Clue>() {
            if candidates.get(&clue.digit).is_none() {
                continue;
            }
            for (digit, voters) in &candidates {
                if !voters.has_peer(Peer::Row)
                    || !voters.has_peer(Peer::Clm)
                    || !voters.has_peer(Peer::Box)
                {
                    voters.notify(Box::new(Discovery {
                        digit: *digit,
                        row: row,
                        clm: clm,
                    }));
                    return;
                }
            }
        }
    }
    // lead to error isd someone has already found the solution
    sol_tx.send(false).unwrap()
}
