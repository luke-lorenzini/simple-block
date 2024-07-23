use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Block {
    pub id: u32,
    pub transactions: Vec<(u32, Trans)>,
}

impl Block {
    pub fn new(id: u32, transactions: Vec<(u32, Trans)>) -> Self {
        Block { id, transactions }
    }
}

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

    pub fn cut_block(&mut self) -> Vec<(u32, Trans)> {
        self.transactions.drain(..).collect()
    }

    pub fn balance(&self, id: u32) -> Result<f64, Box<dyn std::error::Error>> {
        if let Some(v) = self.accounts.get(&id) {
            Ok(v.balance)
        } else {
            Err("Account does not exist".into())
        }
    }

    pub fn transact(&mut self, t: Trans) -> Result<(), Box<dyn std::error::Error>> {
        match t {
            Trans::CreateAccount { id, start_balance } => match self.accounts.get(&id) {
                None => {
                    self.accounts.insert(
                        id,
                        AccountDetails {
                            balance: start_balance,
                        },
                    );
                    self.next_transaction_number += 1;
                    self.transactions.push((self.next_transaction_number, t));
                }
                Some(_) => return Err("Account already exists".into()),
            },
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
                        self.next_transaction_number += 1;
                        self.transactions.push((self.next_transaction_number, t));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_account() {
        let mut xxx = Transactor::new();
        let res = xxx.transact(Trans::CreateAccount {
            id: 0,
            start_balance: 500.,
        });

        assert!(res.is_ok());
    }

    #[test]
    fn new_duplicate_account() {
        let mut xxx = Transactor::new();
        let res = xxx.transact(Trans::CreateAccount {
            id: 0,
            start_balance: 500.,
        });

        assert!(res.is_ok());

        let res = xxx.transact(Trans::CreateAccount {
            id: 0,
            start_balance: 500.,
        });

        assert!(res.is_err());
    }

    #[test]
    fn new_account_balance() {
        let mut xxx = Transactor::new();
        let _ = xxx.transact(Trans::CreateAccount {
            id: 0,
            start_balance: 500.,
        });

        assert_eq!(500., xxx.balance(0).unwrap());
    }

    #[test]
    fn valid_transaction() {
        let mut xxx = Transactor::new();
        let _ = xxx.transact(Trans::CreateAccount {
            id: 0,
            start_balance: 500.,
        });
        let _ = xxx.transact(Trans::CreateAccount {
            id: 1,
            start_balance: 500.,
        });
        let _ = xxx.transact(Trans::Transfer {
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
        let _ = xxx.transact(Trans::CreateAccount {
            id: 0,
            start_balance: 5.,
        });
        let _ = xxx.transact(Trans::CreateAccount {
            id: 1,
            start_balance: 500.,
        });
        let res = xxx.transact(Trans::Transfer {
            from_id: 0,
            to_id: 1,
            amount: 50.,
        });

        assert!(res.is_err());
    }
}
