# Simple Block

This example program is meant to simulate a simple blockchain toy which can do three things;

1) create new accounts with the  `create-account <id> <amount>` command
2) transfer funds from one account to another with the `transfer <from> <to> <amount>` command
3) display the balance of an account with the `balance <id>` command

This example can handle N separate transactions per block in time T. When a new block is cut, a vector of transactions, id number, and sha-256 hash will be created and added to a vector of blocks. To view the entire chain, see the 'notes' section below.

Valid commands are accepted with a 'success' when a new block is cut. Invalid commands are immediately acknowledged with a failure reason.

## Build and Run

This example is intentionally simple to both build and use. It only requires one external crate for sha-256 hashing.

To run, simply execute

```bash
cargo run
```

A sample sequence could be:

```bash
create-account 0 999
balance 0
create-account 1 .99
balance 1
transfer 0 1 10
balance 0
balance 1
```

Once the program is running, commands will be captured via `stdin` in the same terminal which the program was invoked.

## Use

Every T=block_cut_time seconds the block-cutter will print to `stdout` a list of **successful** transactions since the last time a block was cut. If there were no successful transactions, there will be no output.

## Limitations / Notes

To see the entire blockchain as new transactions are added simply uncomment the line

```Rust
// ****** Uncomment me to write the chain to the screen after a new block has been cut. ******
// blockchain.iter().for_each(|f| println!("{:?}", f));
```

There was a requirement for piping output from one terminal to another, however, due to time limitations this was not implemented.
