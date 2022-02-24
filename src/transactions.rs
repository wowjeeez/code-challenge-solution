#[derive(Debug)]
pub enum TRANSACTION {
    DEPOSIT,
    WITHDRAW,
    DISPUTE,
    RESOLVE,
    CHARGEBACK,
}

impl TRANSACTION {
    pub fn from(str: &str) -> TRANSACTION {
        match str {
            "deposit" => TRANSACTION::DEPOSIT,
            "withdrawal" => TRANSACTION::WITHDRAW,
            "dispute" => TRANSACTION::DISPUTE,
            "resolve" => TRANSACTION::RESOLVE,
            "chargeback" => TRANSACTION::CHARGEBACK,
            _ => panic!("Invalid transaction type: {}", str),
        }
    }
    pub fn create(self, client_id: u16, id: u32, amount: f64) -> Transaction {
        Transaction::create(self, client_id, id, amount)
    }
}

#[derive(Debug)]
pub struct Transaction {
    pub r#type: TRANSACTION,
    pub account: u16,
    pub id: u32,
    pub amount: f64, //on modern PCs similar perf and more precision
    pub is_disp: bool,
}

impl Transaction {
    pub fn create(typ: TRANSACTION, client_id: u16, id: u32, amount: f64) -> Transaction {
        Transaction {
            r#type: typ,
            account: client_id,
            id,
            amount,
            is_disp: false,
        }
    }
}
