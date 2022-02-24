//Core transaction handlers

use crate::account::account::Account;
use crate::transactions::Transaction;

pub struct AccountManager<'t> {
    inner: &'t mut Account,
}
impl AccountManager<'_> {
    pub fn new(acc: &mut Account) -> AccountManager {
        AccountManager { inner: acc }
    }
    pub fn deposit(&mut self, trans: &Transaction) {
        let to_add = trans.amount;
        self.inner.total += to_add;
        self.inner.available += to_add;
    }
    pub fn withdraw(&mut self, trans: &Transaction) {
        let to_rem = trans.amount;
        if self.inner.available >= to_rem {
            self.inner.available -= to_rem;
            self.inner.total -= to_rem;
        }
    }
    pub fn resolve(&mut self, trans: &Transaction) {
        let trans_ref_res = self.inner.get_transaction(trans.id);
        if trans_ref_res.is_some() {
            let trans_ref = trans_ref_res.unwrap();
            if trans_ref.is_disp {
                trans_ref.is_disp = false;
                let to_res = trans_ref.amount;
                self.inner.available += to_res;
                self.inner.held -= to_res;
            }
        }
    }
    pub fn chargeback(&mut self, trans: &Transaction) {
        let trans_ref_res = self.inner.get_transaction(trans.id);
        if trans_ref_res.is_some() {
            let trans_ref = trans_ref_res.unwrap();
            if trans_ref.is_disp {
                let to_rem = trans_ref.amount;
                self.inner.held -= to_rem;
                self.inner.total -= to_rem;
                self.inner.locked = true;
            }
        }
    }
    pub fn dispute(&mut self, trans: &Transaction) {
        let trans_ref_res = self.inner.get_transaction(trans.id);
        if trans_ref_res.is_some() {
            let trans_ref = trans_ref_res.unwrap();
            if !trans_ref.is_disp {
                trans_ref.is_disp = true;
                let to_disp = trans_ref.amount;
                self.inner.available -= to_disp;
                self.inner.held += to_disp;
            }
        }
    }
    pub fn push_trans(&mut self, trans: Transaction) {
        self.inner.transactions.push(trans);
    }
}
