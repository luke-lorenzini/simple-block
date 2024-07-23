use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use sha2::{Digest, Sha256};

use simple_block::{Block, TransactionTypes, Transactor, Xxx};

const BLOCK_CUT_TIME: u64 = 10;

fn main() {
    // loop {

    let transactor = Arc::new(Mutex::new(Transactor::new()));
    let mut transaction_stack = VecDeque::default();

    let block_cutter = thread::spawn({
        let transactor = transactor.clone();
        move || {
            let block = Block::default();
            let mut blockchain = vec![block];

            for _ in 0..5 {
                let block_transactions: Vec<Xxx>;
                // Minimize the time we hold the mutex
                {
                    block_transactions = transactor.lock().unwrap().cut_block();
                }
                if !block_transactions.is_empty() {
                    let mut hasher = Sha256::new();
                    hasher.update(blockchain.iter().last().unwrap().block_hash.as_slice());
                    // hasher.update(block_transactions.as_bytes());
                    let new_id = hasher.finalize();
                    // let nnn = new_id.as_slice();
                    // println!("{:?}", new_id);

                    let block = Block {
                        id: blockchain.iter().last().unwrap().id + 1,
                        transactions: block_transactions,
                        block_hash: new_id.to_vec(),
                    };

                    blockchain.push(block);

                    // println!("{:?}\n", blockchain);
                    blockchain.iter().for_each(|f| println!("{:?}", f));
                }

                thread::sleep(Duration::from_secs(BLOCK_CUT_TIME));
            }
        }
    });

    transaction_stack.push_back(TransactionTypes::CreateAccount {
        id: 0,
        start_balance: 500.,
    });
    transaction_stack.push_back(TransactionTypes::CreateAccount {
        id: 1,
        start_balance: 50.,
    });
    transaction_stack.push_back(TransactionTypes::Transfer {
        from_id: 0,
        to_id: 1,
        amount: 10.,
    });
    transaction_stack.push_back(TransactionTypes::Transfer {
        from_id: 0,
        to_id: 1,
        amount: 10.,
    });

    dumb_sleep(1);

    while let Some(v) = transaction_stack.pop_front() {
        let _ = transactor.lock().unwrap().transact(v);
    }

    transaction_stack.push_back(TransactionTypes::Transfer {
        from_id: 0,
        to_id: 1,
        amount: 10.,
    });
    transaction_stack.push_back(TransactionTypes::Transfer {
        from_id: 0,
        to_id: 1,
        amount: 10.,
    });

    dumb_sleep(1);

    while let Some(v) = transaction_stack.pop_front() {
        let _ = transactor.lock().unwrap().transact(v);
    }

    transaction_stack.push_back(TransactionTypes::Transfer {
        from_id: 0,
        to_id: 1,
        amount: 10.,
    });
    transaction_stack.push_back(TransactionTypes::Transfer {
        from_id: 0,
        to_id: 1,
        amount: 10.,
    });

    dumb_sleep(1);

    while let Some(v) = transaction_stack.pop_front() {
        let _ = transactor.lock().unwrap().transact(v);
    }

    block_cutter.join().unwrap();
    // }
}

fn dumb_sleep(sleep_time: u64) {
    let start = Instant::now();
    let mut duration = start.elapsed();

    while duration <= Duration::from_secs(sleep_time) {
        duration = start.elapsed();
    }
}
