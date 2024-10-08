use std::sync::{Arc, Mutex};

use log::info;

use crate::message::{self, Transaction};

pub struct MemPool {
    transactions: Vec<message::Transaction>
}

impl MemPool {
    pub fn new() -> MemPool {
        let transactions: Vec<message::Transaction> = Vec::new(); 
        MemPool{transactions: transactions }
    }

    pub fn add_transaction(&mut self,transaction:message::Transaction) {
        self.transactions.push(transaction);
    }

    pub fn payload(&mut self, batch_size:usize) -> Vec<message::message::Transaction> {
        let mempool_size = self.transactions.len();
        if self.transactions.len() < batch_size {
            let payload = self.transactions.drain(0..mempool_size).collect();
            payload
        } else {
            let payload = self.transactions.drain(0..batch_size).collect();
            payload
        }
    }
}