use std::{
    collections::VecDeque,
    io::{self, BufRead},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use sha2::{Digest, Sha256};

use simple_block::{Block, Transaction, TransactionTypes, Transactor};

const BLOCK_CUT_TIME: u64 = 10;

fn main() -> io::Result<()> {
    let transactor = Arc::new(Mutex::new(Transactor::new()));

    // Start a thread which wakes to cut new blocks and insert them into the chain
    thread::spawn({
        let transactor = transactor.clone();
        move || {
            let block = Block::default();
            let mut blockchain = vec![block];

            loop {
                let block_transactions: Vec<Transaction>;
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

                    println!("Trans #\tType");
                    blockchain
                        .iter()
                        .last()
                        .unwrap()
                        .transactions
                        .iter()
                        .for_each(|f| {
                            println!("{:?}\t{:?}", f.0, f.1);
                        });

                    // ****** Uncomment me to write the chain to the screen after a new block has been cut. ******
                    // blockchain.iter().for_each(|f| println!("{:?}", f));
                }

                thread::sleep(Duration::from_secs(BLOCK_CUT_TIME));
            }
        }
    });

    process_input(transactor.clone())?;

    Ok(())
}

fn process_input(transactor: Arc<Mutex<Transactor>>) -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    loop {
        handle.read_line(&mut buffer)?;

        let mut message_args: VecDeque<_> = buffer.split_ascii_whitespace().collect();

        if !message_args.is_empty() {
            match message_args.pop_front() {
                Some("create-account") => {
                    // println!("create-account command");

                    let id = message_args.pop_front().and_then(|s| s.parse::<u32>().ok());
                    let start_balance =
                        message_args.pop_front().and_then(|s| s.parse::<f64>().ok());

                    match (id, start_balance) {
                        (Some(id), Some(start_balance)) => {
                            let new_account = TransactionTypes::CreateAccount { id, start_balance };

                            if let Err(e) = transactor.lock().unwrap().transact(new_account) {
                                println!("{:?}", e);
                            }
                        }
                        (_, _) => {
                            println!(
                                "Parse error (create-account command) - Please enter valid parameters"
                            )
                        }
                    }
                }
                Some("transfer") => {
                    // println!("transfer command");

                    let from_id = message_args.pop_front().and_then(|s| s.parse::<u32>().ok());
                    let to_id = message_args.pop_front().and_then(|s| s.parse::<u32>().ok());
                    let amount = message_args.pop_front().and_then(|s| s.parse::<f64>().ok());

                    match (from_id, to_id, amount) {
                        (Some(from_id), Some(to_id), Some(amount)) => {
                            let transfer = TransactionTypes::Transfer {
                                from_id,
                                to_id,
                                amount,
                            };

                            if let Err(e) = transactor.lock().unwrap().transact(transfer) {
                                println!("{:?}", e);
                            }
                        }
                        (_, _, _) => println!(
                            "Parse error (transfer command) - Please enter valid parameters"
                        ),
                    }
                }
                Some("balance") => {
                    // println!("balance command");

                    let account = message_args.pop_front().and_then(|s| s.parse::<u32>().ok());

                    match account {
                        Some(account) => {
                            let transactor = transactor.lock().unwrap();
                            let balance = transactor.balance(account);
                            if let Ok(balance) = balance {
                                println!("Balance of account {:?} is {:?}", account, balance);
                            }
                        }
                        None => {
                            println!(
                                "Parse error (balance command) - Please enter valid parameters"
                            )
                        }
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
    }
}
