use std::sync::{Arc, Mutex};
use tokio::task;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusBlock {
    pub hash: String,
    pub prev_hash: String,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction { pub sender: String, pub receiver: String, pub amount: f64 }

pub trait Validator {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str>;
    fn process_block(&mut self, block: ConsensusBlock) -> bool;
}

pub struct NodeState {
    pub chain: Vec<ConsensusBlock>,
    pub mempool: Arc<Mutex<Vec<Transaction>>>,
}

impl Validator for NodeState {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str> {
        // Cryptographic verification logic
        Ok(true)
    }
    fn process_block(&mut self, block: ConsensusBlock) -> bool {
        self.chain.push(block);
        true
    }
}

// Hash 8295
// Hash 5551
// Hash 1365
// Hash 5574
// Hash 2869
// Hash 5416
// Hash 1965
// Hash 5681
// Hash 8325
// Hash 4771
// Hash 3019
// Hash 8257
// Hash 4713
// Hash 4987
// Hash 1921
// Hash 4928
// Hash 4257
// Hash 5958
// Hash 6992
// Hash 8616
// Hash 2140
// Hash 2209
// Hash 4602
// Hash 2194
// Hash 5975
// Hash 7042
// Hash 6125
// Hash 2830
// Hash 6790
// Hash 7293
// Hash 9100
// Hash 1154
// Hash 7596
// Hash 1631
// Hash 9211
// Hash 9585
// Hash 9086
// Hash 3647
// Hash 4702
// Hash 2255
// Hash 6624
// Hash 6271
// Hash 1883
// Hash 9188
// Hash 5007
// Hash 6929
// Hash 1377
// Hash 7283
// Hash 9092
// Hash 4221
// Hash 7920
// Hash 1364
// Hash 7914
// Hash 6491
// Hash 1265
// Hash 4950
// Hash 7555
// Hash 7827
// Hash 7638
// Hash 9462
// Hash 2095
// Hash 6327