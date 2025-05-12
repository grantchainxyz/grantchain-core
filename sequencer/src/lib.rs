use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tx {
    pub sender: String,
    pub payload: String,
}

pub fn sort_txs(mut txs: Vec<Tx>) -> Vec<Tx> {
    txs.sort_by(|a, b| a.sender.cmp(&b.sender));
    txs
}
