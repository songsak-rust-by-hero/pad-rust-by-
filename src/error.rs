use std::fmt;

#[derive(Debug)]
enum BankError {
    InsufficientFunds { balance: i64, amount: i64 },
    AccountNotFound(String),
    AccountFrozen(String),
    DailyLimitExceeded { limit: i64, attempted: i64 },
    InvalidAmount(i64),
    TransactionFailed(String),
    DuplicateTransaction(String),
}

impl fmt::Display for BankError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BankError::InsufficientFunds { balance, amount } =>
                write!(f, "ยอดเงินไม่เพียงพอ: มี {} แต่ต้องการโอน {}",
                    satang_to_baht(*balance), satang_to_baht(*amount)),
            BankError::AccountNotFound(id) =>
                write!(f, "ไม่พบบัญชี: {}", id),
            BankError::AccountFrozen(id) =>
                write!(f, "บัญชีถูกระงับ: {}", id),
            BankError::DailyLimitExceeded { limit, attempted } =>
                write!(f, "เกินวงเงินต่อวัน: วงเงิน {} แต่พยายามทำ {}",
                    satang_to_baht(*limit), satang_to_baht(*attempted)),
            BankError::InvalidAmount(amount) =>
                write!(f, "จำนวนเงินไม่ถูกต้อง: {}",satang_to_baht(*amount)),
            BankError::TransactionFailed(reason) =>
                write!(f, "ธุรกรรมล้มเหลว: {}", reason),
            BankError::DuplicateTransaction(tx_id) =>
                write!(f, "รายการซ้ำซ้อน: {}", tx_id),
        }
    }
}

impl std::error::Error for BankError {}