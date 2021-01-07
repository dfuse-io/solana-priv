use solana_sdk::{
    keyed_account::KeyedAccount,
    pubkey::Pubkey,
    signature::Signature,
};
use hex;
use std::sync::atomic::{AtomicBool, Ordering};

pub static DEEPMIND_ENABLED: AtomicBool = AtomicBool::new(false);
pub fn enable_deepmind() {
    DEEPMIND_ENABLED.store(true, Ordering::Relaxed)
}
pub fn disable_deepmind() {
    DEEPMIND_ENABLED.store(false, Ordering::Relaxed)
}
pub fn deepmind_enabled() -> bool {
    return DEEPMIND_ENABLED.load(Ordering::Relaxed);
}


#[derive(Default,Copy,Clone)]
pub struct DMTrxContext {
    pub signature: Signature,
    pub batch_number: u64,

}

#[derive(Default,Copy,Clone)]
pub struct DMLogContext {
    pub batch_number: u64,
    pub ordinal_number: u32,
    pub parent_ordinal_number: u32,
    pub trx_id: Signature,
}

impl DMLogContext {
    //****************************************************************
    // DMLOG FUNCTION ADDITION
    //****************************************************************
    pub fn inc_ordinal_number(&mut self) {
        self.ordinal_number += 1;
    }
    pub fn set_parent_ordinal_number(&mut self, value: u32) {
        self.parent_ordinal_number = value;
    }
    pub fn get_ordinal_number(&self) -> u32 {
        return self.ordinal_number;
    }
    pub fn get_parent_ordinal_number(&self) -> u32 {
        return self.parent_ordinal_number;
    }

    pub fn print_instruction_start(&self, program_id: Pubkey, keyed_accounts: &[KeyedAccount], instruction_data: &[u8]) {
        let accounts: Vec<String> = keyed_accounts.into_iter().map(|i| format!("{}:{}{}", i.unsigned_key(), if i.is_signer() { 1 }  else { 0 }, if i.is_writable() { 1 }  else { 0 })).collect();
        if deepmind_enabled() {
            println!(
                "DMLOG INST_S {} {} {} {} {} {} {}",
                self.batch_number,
                self.trx_id,
                self.ordinal_number,
                self.parent_ordinal_number,
                program_id,
                hex::encode(instruction_data),
                accounts.join(";"),
            );
        }
    }

    pub fn print_lamport_change(&self, pubkey: Pubkey, pre: u64, post: u64) {
        if deepmind_enabled() {
            println!(
                "DMLOG LAMP_CH {} {} {} {} {} {}",
                self.batch_number,
                self.trx_id,
                self.ordinal_number,
                pubkey,
                pre,
                post
            );
        }
    }

    pub fn print_account_change(&self, pubkey: Pubkey, pre: &[u8], post: &[u8]) {
        if deepmind_enabled() {
            println!(
                "DMLOG ACCT_CH {} {} {} {} {} {}",
                self.batch_number,
                self.trx_id,
                self.ordinal_number,
                pubkey,
                hex::encode(pre),
                hex::encode(post)
            );
        }
    }
}
