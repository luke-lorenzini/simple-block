use std::collections::HashMap;

#[derive(Debug)]
pub enum Trans {
    CreateAccount {
        id: u32,
        start_balance: f64,
    },
    Transfer {
        from_id: u32,
        to_id: u32,
        amount: f64,
    },
}

#[derive(Debug)]
struct AccountDetails {
    balance: f64,
}

#[derive(Default)]
pub struct Transactor {
    accounts: HashMap<u32, AccountDetails>,
    transactions: Vec<(u32, Trans)>,
    next_transaction_number: u32,
}

impl Transactor {
    pub fn new() -> Self {
        let accounts = HashMap::default();
        let transactions = Vec::default();

        Transactor {
            accounts,
            transactions,
            next_transaction_number: 0,
        }
    }

    pub fn transact(&mut self, t: Trans) -> Result<(), Box<dyn std::error::Error>> {
        match t {
            Trans::CreateAccount { id, start_balance } => {
                self.accounts.insert(
                    id,
                    AccountDetails {
                        balance: start_balance,
                    },
                );
                self.transactions
                    .push((self.next_transaction_number + 1, t));
            }
            Trans::Transfer {
                from_id,
                to_id,
                amount,
            } => {
                if self.accounts.contains_key(&from_id) && self.accounts.contains_key(&to_id) {
                    let source_account = self.accounts.get(&from_id).expect("Already checked");
                    let destination_account = self.accounts.get(&to_id).expect("Already checked");

                    if source_account.balance >= amount {
                        let b_source = source_account.balance - amount;
                        let b_dest = destination_account.balance + amount;
                        self.accounts
                            .insert(from_id, AccountDetails { balance: b_source });
                        self.accounts
                            .insert(to_id, AccountDetails { balance: b_dest });
                        self.transactions
                            .push((self.next_transaction_number + 1, t));
                    } else {
                        return Err("NSF".into());
                    }
                } else {
                    return Err("One of the accounts does not exist".into());
                }
            }
        }

        Ok(())
    }

    pub fn display_accounts(&self) {
        println!("{:?}", self.accounts);
    }

    pub fn display_transactions(&self) {
        println!("{:?}", self.transactions);
    }
}
