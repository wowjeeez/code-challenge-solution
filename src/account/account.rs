use std::io;
use crate::transactions::Transaction;
use crate::account::manager::AccountManager;
use crate::TRANSACTION;

#[derive(Debug)]
pub struct Account {
    pub(crate) client: u16,
    pub(crate) transactions: Vec<Transaction>,
    pub(crate) available: f64,
    pub(crate) held: f64,
    pub(crate) total: f64,
    pub(crate) locked: bool
}
impl Account {
    pub fn get_transaction(&mut self, id: u32) -> Option<&mut Transaction> {
        for trans in self.transactions.iter_mut() {
            if trans.id == id {
                return Some(trans)
            }
        }
        None
    }
}

//Struct to wrap accounts for easier access
pub struct AccountStorage {
    inner: Vec<Account>
}



impl AccountStorage {
    fn get_idx(&self, client: u16) -> Option<usize> {
        for (idx, acc) in self.inner.iter().enumerate() {
            if acc.client == client {
                return Some(idx)
            }
        }
        None
    }
    //Core wrapper method, will create an account if it doesnt exist, then process the transaction
    pub fn handle_transaction(&mut self, client: u16, trans: Transaction) {
        let idx = self.get_or_create(client);
        let mut mgr = self.get_manager(idx);
        match &trans.r#type {
            TRANSACTION::DEPOSIT => mgr.deposit(&trans),
            TRANSACTION::WITHDRAW => mgr.withdraw(&trans),
            TRANSACTION::DISPUTE => mgr.dispute(&trans),
            TRANSACTION::RESOLVE => mgr.resolve(&trans),
            TRANSACTION::CHARGEBACK => mgr.chargeback(&trans)
        };
        mgr.push_trans(trans);

    }
    fn get_or_create(&mut self, client: u16) -> usize {
        let idx = self.get_idx(client);
        return if idx.is_some() {
            idx.unwrap()
        } else {
            self.create(client)
        }
    }
    
    fn create(&mut self, client: u16) -> usize {
        self.inner.push(Account {
            client,
            transactions: vec![],
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false
        });
        self.inner.len() - 1
    }
    
    pub fn new() -> AccountStorage {
        AccountStorage { inner: vec![]}
    }
    //returns a new struct allowing to edit a certain account (assumes that the account exists)
    fn get_manager(&mut self, idx: usize) -> AccountManager {
        AccountManager::new(self.inner.get_mut(idx).unwrap())
    }
    //writes the solution to stdout
    pub fn write(&self) {
        let mut wrt = csv::WriterBuilder::new().from_writer(io::stdout());
        wrt.write_record(&["client", "available", "held", "total", "locked"]).expect("failed to write to stdout");
        for acc in self.inner.iter() {
            wrt.write_record(&[acc.client.to_string(), convert(acc.available), convert(acc.held), convert(acc.total), acc.locked.to_string()]).expect("failed to write to stdout");
        }
    }
}
//convert floats with prec kept
fn convert(val: f64) -> String {
    format!("{:.prec$}", val, prec = 4)
}