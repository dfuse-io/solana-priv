use solana_sdk::{
    keyed_account::KeyedAccount,
    pubkey::Pubkey,
    signature::Signature,
};
use std::fs::File;
use std::os::unix::io::{FromRawFd, IntoRawFd, RawFd};
use std::io::Write;
use hex;
use std::sync::atomic::{AtomicBool, Ordering};
use solana_program::hash::Hash;
use num_traits::ToPrimitive;

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

pub struct DMAccountChange {
    pub pubkey: Pubkey,
    pub pre: Vec<u8>,
    pub post:Vec<u8>,
}

pub struct DMInstruction {
    pub data: Vec<u8>,
    pub program_id: Pubkey,
    pub accounts: Vec<String>,
    pub account_changes: Vec<DMAccountChange>
}

impl DMInstruction {
    pub fn add_account_change(&mut self,  pubkey: Pubkey, pre: &[u8], post: &[u8]) {
        let mut account = DMAccountChange{
            pubkey,
            pre: Vec::with_capacity(pre.len()),
            post: Vec::with_capacity(post.len())
        };
        account.pre.copy_from_slice(pre);
        account.post.copy_from_slice(post);
        self.account_changes.push(account);
    }

}
#[derive(Default)]
pub struct DMTransaction {
    pub sigs: String, // ':'-separated list of signatures
    pub num_required_signatures: u8,
    pub num_readonly_signed_accounts: u8,
    pub num_readonly_unsigned_accounts: u8,
    pub account_keys: String,
    pub recent_blockhash: Hash,

    pub current_ordinal_number: u32,
    pub instructions: Vec<DMInstruction>,
    pub logs: Vec<String>,
}

impl DMTransaction {
    pub fn inc_ordinal_number(&mut self) {
        self.current_ordinal_number += 1;
    }

    pub fn start_instruction(&mut self, program_id: Pubkey, keyed_accounts: &[KeyedAccount], instruction_data: &[u8]) {
        let accounts: Vec<String> = keyed_accounts.into_iter().map(|i| format!("{}:{}{}", i.unsigned_key(), if i.is_signer() { 1 }  else { 0 }, if i.is_writable() { 1 }  else { 0 })).collect();
        let mut inst = DMInstruction{
            accounts,
            program_id,
            data: Vec::with_capacity(instruction_data.len()),
            account_changes: Vec::new(),
        };
        inst.data.copy_from_slice(instruction_data);
        self.instructions.push(inst);
    }

    pub fn add_log(&mut self, log: String) {
       self.logs.push(log)
    }
}

#[derive(Default)]
pub struct DMBatchContext {
    pub batch_number: u64,
    pub trxs: Vec<DMTransaction>,
    pub fd: i32,
    pub path: String,
}

impl<'a> DMBatchContext {
    pub fn start_trx(&mut self, sigs: String, num_required_signatures: u8,num_readonly_signed_accounts: u8,num_readonly_unsigned_accounts: u8,account_keys: String,recent_blockhash: Hash) {
        let mut ordinal_number = 1;
        if let Some(i) = self.trxs.len().to_u32() {
            ordinal_number = i
        }

        let f = unsafe { &mut File::from_raw_fd(self.fd) };
        let cnt = format!("TRX_START {} {} {} {} {} {}",
                 sigs,
                 num_required_signatures,
                 num_readonly_signed_accounts,
                 num_readonly_unsigned_accounts,
                 account_keys,
                 recent_blockhash,
        );
        f.write_all(cnt.as_bytes()); // TODO: any error handling here??
        // TODO: make sure all those `write_all` calls don't close the file, because the next
        // write would fail

        self.trxs.push(DMTransaction{
            sigs,
            num_required_signatures,
            num_readonly_signed_accounts,
            num_readonly_unsigned_accounts,
            account_keys,
            recent_blockhash,
            current_ordinal_number: ordinal_number,
            instructions: Vec::new(),
            logs: Vec::new(),
        })
    }

    pub fn flush(&mut self) {
        let f = unsafe { &mut File::from_raw_fd(self.fd) };
        drop(f); // TODO: call `sync_all()` to handle errors upon closing, otherwise we'll have issues on the other side!
        println!("DMLOG BATCH {}", self.path);
    }

    pub fn start_instruction(&mut self, program_id: Pubkey, keyed_accounts: &[KeyedAccount], instruction_data: &[u8]) {
        if let Some(transaction) = self.trxs.last_mut() {
            transaction.start_instruction(program_id, keyed_accounts, instruction_data)
        }
        // Do we panic here? this should never happen?
    }

    pub fn account_change(&mut self, pubkey: Pubkey, pre: &[u8], post: &[u8]) {
        if let Some(transaction) = self.trxs.last_mut() {
            if let Some(instruction) = transaction.instructions.last_mut() {
                instruction.add_account_change(pubkey, pre, post)
            }
        }
    }

    pub fn add_log(&mut self, log: String) {
        let f = unsafe { &mut File::from_raw_fd(self.fd) };
        //println!("DMLOG TRX_LOG {} {} {}", slot_num, tx.signatures[0], hex::encode(log));
        //
        // We shouldn't need the `tx.signatures[0]` nor the `slot_num`
        // now because we'll be processing batches that are complete
        // by the time they reach us (through the `DMLOG BATCH` file),
        // and processing them linearly.
        f.write_all(format!("TRX_LOG {}", log).as_bytes());

        if let Some(transaction) = self.trxs.last_mut() {
            transaction.add_log(log);
        }
    }

    pub fn trx_end(&mut self) {
        let f = unsafe { &mut File::from_raw_fd(self.fd) };
        //println!("DMLOG TRX_LOG {} {} {}", slot_num, tx.signatures[0], hex::encode(log));
        //
        // We shouldn't need the `tx.signatures[0]` nor the `slot_num`
        // now because we'll be processing batches that are complete
        // by the time they reach us (through the `DMLOG BATCH` file),
        // and processing them linearly.
        f.write_all("TRX_END".as_bytes());
    }
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
