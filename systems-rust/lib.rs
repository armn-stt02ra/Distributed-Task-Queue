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
// Hash 3284
// Hash 7497
// Hash 7020
// Hash 4635
// Hash 5420
// Hash 6599
// Hash 7207
// Hash 7104
// Hash 8828
// Hash 3674
// Hash 2697
// Hash 2887
// Hash 7078
// Hash 1165
// Hash 9791
// Hash 4528
// Hash 9300
// Hash 8455
// Hash 7061
// Hash 8753
// Hash 4185
// Hash 9837
// Hash 1398
// Hash 9260
// Hash 7107
// Hash 8492
// Hash 3897
// Hash 7733
// Hash 5612
// Hash 7908
// Hash 3009
// Hash 7610
// Hash 6283
// Hash 9076
// Hash 9576
// Hash 6517
// Hash 4606
// Hash 5079
// Hash 8131
// Hash 3390
// Hash 6232
// Hash 8596
// Hash 7893
// Hash 9903
// Hash 2023
// Hash 1126
// Hash 1777
// Hash 1852
// Hash 6514
// Hash 5535
// Hash 2479
// Hash 4657
// Hash 1204
// Hash 4053
// Hash 2775
// Hash 3486
// Hash 9283
// Hash 8400
// Hash 7004
// Hash 7810
// Hash 1211
// Hash 7552
// Hash 9629
// Hash 5004
// Hash 6855
// Hash 8103
// Hash 4054
// Hash 9738
// Hash 6644
// Hash 5696
// Hash 1444
// Hash 6436
// Hash 9843
// Hash 2611
// Hash 7326
// Hash 2154
// Hash 7168
// Hash 4498
// Hash 7511
// Hash 1015
// Hash 5230
// Hash 1012
// Hash 6945
// Hash 7745
// Hash 6490
// Hash 3391
// Hash 9239
// Hash 7772
// Hash 7306
// Hash 6102
// Hash 5280
// Hash 4304
// Hash 4622
// Hash 3931
// Hash 7966
// Hash 6346
// Hash 8771
// Hash 8251
// Hash 6306
// Hash 5049
// Hash 6688
// Hash 3681
// Hash 4357
// Hash 9388
// Hash 2077
// Hash 3097
// Hash 1669
// Hash 3637
// Hash 7953
// Hash 1398
// Hash 8320
// Hash 8685
// Hash 2286
// Hash 2456
// Hash 7208
// Hash 9831
// Hash 3961
// Hash 8192
// Hash 3325
// Hash 8400
// Hash 8569
// Hash 7804
// Hash 3401
// Hash 5005
// Hash 7909