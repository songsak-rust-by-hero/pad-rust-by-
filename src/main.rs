mod satang;
mod error;
mod account;

use account::Account;
use satang::satang_to_baht;
use error::BankError;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}",e);
        std::process::exit(1);
    }     
}

fn run()-> Result<(),BankError> {
    println!("Banktest");
    
    let mut acc = Account::new("TH-999".to_string(),1000_00)?;
    println!("บันชี: {}, ยอดเงิน: {}",acc.account_id(),satang_to_baht(acc.balance()));

    println!("ทดสอบระบบฝากเงิน 500_90");
     acc.deposit(500_90)?;
     println!("ยอดเงินใหม่: {}",satang_to_baht(acc.balance()));
    
    println!("ทดลองระบบฝากติดลบ");
    acc.deposit(-100_00)?;
    println!("ยอดงินสุดท้าย {} บาท",satang_to_baht(acc.balance()));
    Ok(())    
}

