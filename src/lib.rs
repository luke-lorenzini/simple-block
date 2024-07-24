use std::collections::HashMap;

pub type Transaction = (u32, TransactionTypes);

#[derive(Debug, Default)]
pub struct Block {
    pub id: u32,
    pub transactions: Vec<Transaction>,
    pub block_hash: Vec<u8>,
}

impl Block {
    pub fn new(id: u32, transactions: Vec<Transaction>, block_hash: Vec<u8>) -> Self {
        Block {
            id,
            transactions,
            block_hash,
        }
    }
}

#[derive(Debug)]
pub enum TransactionTypes {
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
    transactions: Vec<Transaction>,
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

    pub fn cut_block(&mut self) -> Vec<Transaction> {
        self.transactions.drain(..).collect()
    }

    pub fn balance(&self, id: u32) -> Result<f64, Box<dyn std::error::Error>> {
        if let Some(v) = self.accounts.get(&id) {
            Ok(v.balance)
        } else {
            Err("Account does not exist".into())
        }
    }

    pub fn transact(
        &mut self,
        transaction_type: TransactionTypes,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match transaction_type {
            TransactionTypes::CreateAccount { id, start_balance } => match self.accounts.get(&id) {
                None => {
                    self.accounts.insert(
                        id,
                        AccountDetails {
                            balance: start_balance,
                        },
                    );
                    self.next_transaction_number += 1;
                    self.transactions
                        .push((self.next_transaction_number, transaction_type));
                }
                Some(_) => return Err("Account already exists".into()),
            },
            TransactionTypes::Transfer {
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
                        self.next_transaction_number += 1;
                        self.transactions
                            .push((self.next_transaction_number, transaction_type));
                    } else {
                        return Err("Insufficient funds for transfer".into());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_account() {
        let mut xxx = Transactor::new();
        let res = xxx.transact(TransactionTypes::CreateAccount {
            id: 0,
            start_balance: 500.,
        });

        assert!(res.is_ok());
    }

    #[test]
    fn new_duplicate_account() {
        let mut xxx = Transactor::new();
        let res = xxx.transact(TransactionTypes::CreateAccount {
            id: 0,
            start_balance: 500.,
        });

        assert!(res.is_ok());

        let res = xxx.transact(TransactionTypes::CreateAccount {
            id: 0,
            start_balance: 500.,
        });

        assert!(res.is_err());
    }

    #[test]
    fn new_account_balance() {
        let mut xxx = Transactor::new();
        let _ = xxx.transact(TransactionTypes::CreateAccount {
            id: 0,
            start_balance: 500.,
        });

        assert_eq!(500., xxx.balance(0).unwrap());
    }

    #[test]
    fn valid_transaction() {
        let mut xxx = Transactor::new();
        let _ = xxx.transact(TransactionTypes::CreateAccount {
            id: 0,
            start_balance: 500.,
        });
        let _ = xxx.transact(TransactionTypes::CreateAccount {
            id: 1,
            start_balance: 500.,
        });
        let _ = xxx.transact(TransactionTypes::Transfer {
            from_id: 0,
            to_id: 1,
            amount: 50.,
        });

        assert_eq!(450., xxx.balance(0).unwrap());
        assert_eq!(550., xxx.balance(1).unwrap());
    }

    #[test]
    fn nsf() {
        let mut xxx = Transactor::new();
        let _ = xxx.transact(TransactionTypes::CreateAccount {
            id: 0,
            start_balance: 5.,
        });
        let _ = xxx.transact(TransactionTypes::CreateAccount {
            id: 1,
            start_balance: 500.,
        });
        let res = xxx.transact(TransactionTypes::Transfer {
            from_id: 0,
            to_id: 1,
            amount: 50.,
        });

        assert!(res.is_err());
    }
}
