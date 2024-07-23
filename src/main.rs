use std::time::{Duration, Instant};

use simple_block::{Trans, Transactor};

fn main() {
    // loop {
    //     let start = Instant::now();

    //     let mut duration = start.elapsed();
    //     while duration <= Duration::from_secs(2) {
    //         duration = start.elapsed();
    //     }

    //     println!("Done");
    // }
    let _ = test();
}

fn test() -> Result<(), Box<dyn std::error::Error>> {
    let mut xxx = Transactor::new();

    xxx.transact(Trans::CreateAccount {
        id: 0,
        start_balance: 500.99,
    })?;
    xxx.transact(Trans::CreateAccount {
        id: 1,
        start_balance: 50.9,
    })?;
    xxx.transact(Trans::Transfer {
        from_id: 0,
        to_id: 1,
        amount: 100.,
    })?;
    xxx.transact(Trans::Transfer {
        from_id: 0,
        to_id: 1,
        amount: 10.,
    })?;
    // xxx.transact(Trans::Transfer { from_id: 1, to_id: 0, amount: 1000. })?;

    xxx.display_accounts();
    xxx.display_transactions();

    Ok(())
}
