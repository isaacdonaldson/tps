use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::{Transaction, TransactionId};

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionTree {
    transactions: BTreeMap<TransactionId, Transaction>,
}

impl Default for TransactionTree {
    fn default() -> Self {
        Self::new()
    }
}

impl TransactionTree {
    pub fn new() -> Self {
        Self {
            transactions: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, transaction: Transaction) {
        self.transactions.insert(transaction.tx_id, transaction);
    }

    pub fn contains(&self, tx_id: &TransactionId) -> bool {
        self.transactions.contains_key(tx_id)
    }

    pub fn get(&self, tx_id: &TransactionId) -> Option<&Transaction> {
        self.transactions.get(tx_id)
    }

    pub fn get_mut(&mut self, client_id: &TransactionId) -> Option<&mut Transaction> {
        self.transactions.get_mut(client_id)
    }
}
