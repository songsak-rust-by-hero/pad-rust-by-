use crate::error::BankError; 

pub struct Account {
    account_id: String,
    balance: i64,
    is_frozen: bool,
    daily_withdrawn: i64,
    daily_limit: i64,
}
impl Account {
    pub fn new(account_id: String, initial_balance: i64)->Result<Self,BankError>{
         if initial_balance < 0 {
              return Err(BankError:: InvalidAmount(initial_balance));
         }
         Ok(Self{
              account_id,
              balance: initial_balance,
              is_frozen: false,
              daily_withdrawn: 0,
              daily_limit: 10_000_000,
         })
    }
    pub fn balance(&self) -> i64 {
         self.balance
    }
    pub fn is_frozen(&self) -> bool {
         self.is_frozen
    }
    pub fn account_id(&self) -> &str {
        &self.account_id
    }
    pub fn remaining_daily_limit(&self) -> i64 {
         self.daily_limit - self.daily_withdrawn
    }
    pub fn set_frozen(&mut self,status:bool){
         self.is_frozen = status;
    }
    pub fn deposit(&mut self,amount: i64)-> Result <(),BankError>{
      if self.is_frozen {
         return Err(BankError::AccountFrozen(self.account_id.clone()));
      }
      if  amount <= 0 {
          return Err(BankError::InvalidAmount(amount));
      }
          let new_balance = self.balance
          .checked_add(amount)
          .ok_or_else(|| BankError::TransactionFailed
              ("ยอดเงินเกินขีดจำกัดที่ระบบรองรับ".to_string()))?;
     self.balance = new_balance; 
         Ok(())
    }      
}
