use std::{
    collections::VecDeque,
    io::{self, BufRead},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use sha2::{Digest, Sha256};

use simple_block::{Block, TransactionTypes, Transactor, Xxx};

const BLOCK_CUT_TIME: u64 = 10;

fn main() -> io::Result<()> {
    let transactor = Arc::new(Mutex::new(Transactor::new()));
    let mut transaction_stack = VecDeque::default();

    let _block_cutter = thread::spawn({
        let transactor = transactor.clone();
        move || {
            let block = Block::default();
            let mut blockchain = vec![block];

            loop {
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

                    let block = Block {
                        id: blockchain.iter().last().unwrap().id + 1,
                        transactions: block_transactions,
                        block_hash: new_id.to_vec(),
                    };

                    blockchain.push(block);

                    blockchain.iter().for_each(|f| println!("{:?}", f));
                }

                thread::sleep(Duration::from_secs(BLOCK_CUT_TIME));
            }
        }
    });

    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    loop {
        handle.read_line(&mut buffer)?;

        let mut message_args: VecDeque<_> = buffer.split_ascii_whitespace().collect();

        if !message_args.is_empty() {
            match message_args.pop_front() {
                Some("create-account") => {
                    println!("create-account command");

                    let id = message_args.pop_front();
                    let start_balance = message_args.pop_front();

                    if let (Some(id), Some(start_balance)) = (id, start_balance) {
                        if let (Ok(id), Ok(start_balance)) = (id.parse::<u32>(), start_balance.parse::<f64>()) {
                            let new_account = TransactionTypes::CreateAccount { id, start_balance };

                            transaction_stack.push_back(new_account);
                        } else {
                            todo!("Parse error (create-account command)")
                        }
                    } else {
                        todo!("Parse error (create-account command)")
                    }
                }
                Some("transfer") => {
                    println!("transfer command");

                    let from_id = message_args.pop_front();
                    let to_id = message_args.pop_front();
                    let amount = message_args.pop_front();

                    if let (Some(from_id), Some(to_id), Some(amount)) = (from_id, to_id, amount) {
                        if let (Ok(from_id), Ok(to_id), Ok(amount)) = (from_id.parse::<u32>(), to_id.parse::<u32>(), amount.parse::<f64>()) {
                            let transfer = TransactionTypes::Transfer {
                                from_id,
                                to_id,
                                amount,
                            };

                            transaction_stack.push_back(transfer);
                        }
                    } else {
                        todo!("Parse error (transfer command)")
                    }
                }
                Some("balance") => {
                    println!("balance command");

                    let account = message_args.pop_front();

                    if let Some(account) = account {
                        if let Ok(account) = account.parse::<u32>() {

                            let transactor = transactor.lock().unwrap();
                            let balance = transactor.balance(account);
                            if let Ok(balance) = balance {
                                println!("Balance of account {:?} is {:?}", account, balance);
                            }
                        } else {
                            todo!("Parse error (create-account command)")
                        }
                    } else {
                        todo!("Parse error (balance command)")
                    }
                }
                Some(_) => {
                    println!("Invalid command");
                }
                None => {
                    println!("Enter a command");
                }
            }
        }

        buffer.clear();

        while let Some(v) = transaction_stack.pop_front() {
            let _ = transactor.lock().unwrap().transact(v);
        }
    }

    // block_cutter.join().unwrap();
}
