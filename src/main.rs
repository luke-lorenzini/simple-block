// use std::time::{Duration, Instant};

use std::collections::VecDeque;

use simple_block::{Block, Trans, Transactor};

fn main() {
    // loop {
    // let start = Instant::now();
    let block = Block::default();
    let mut blockchain = vec![block];

    let mut xxx = Transactor::new();
    let mut my_stack: VecDeque<Trans> = VecDeque::default();

    my_stack.push_back(Trans::CreateAccount {
        id: 0,
        start_balance: 500.,
    });
    my_stack.push_back(Trans::CreateAccount {
        id: 1,
        start_balance: 50.,
    });
    my_stack.push_back(Trans::Transfer {
        from_id: 0,
        to_id: 1,
        amount: 10.,
    });
    let _ = xxx.balance(0);
    my_stack.push_back(Trans::Transfer {
        from_id: 0,
        to_id: 1,
        amount: 10.,
    });

    // let mut duration = start.elapsed();
    // while duration <= Duration::from_secs(2) {
    //     duration = start.elapsed();
    // }

    while let Some(v) = my_stack.pop_front() {
        let _ = xxx.transact(v);
    }

    let zzz = xxx.cut_block();
    println!("{:?}", zzz);
    blockchain.push(Block {
        id: blockchain.iter().last().unwrap().id + 1,
        transactions: zzz,
    });

    my_stack.push_back(Trans::Transfer {
        from_id: 0,
        to_id: 1,
        amount: 10.,
    });
    let _ = xxx.balance(0);
    my_stack.push_back(Trans::Transfer {
        from_id: 0,
        to_id: 1,
        amount: 10.,
    });
    while let Some(v) = my_stack.pop_front() {
        let _ = xxx.transact(v);
    }

    let zzz = xxx.cut_block();
    println!("{:?}", zzz);
    blockchain.push(Block {
        id: blockchain.iter().last().unwrap().id + 1,
        transactions: zzz,
    });

    println!("{:?}", blockchain);

    // }
}
