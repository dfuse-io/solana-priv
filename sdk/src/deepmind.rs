use solana_sdk::{
    keyed_account::KeyedAccount,
    pubkey::Pubkey,
};
use std::{
    fs::File,
    os::unix::io::{FromRawFd},
    io::Write,
    borrow::BorrowMut,
    sync::atomic::{AtomicBool, Ordering},
};
use hex;
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

pub struct DMLamportChange {
    pub pubkey: Pubkey,
    pub pre: u64,
    pub post:u64,
}
pub struct DMInstruction {
    pub ordinal: u32,
    pub parent_ordinal: u32,
    pub data: Vec<u8>,
    pub program_id: Pubkey,
    pub accounts: Vec<String>,
    pub account_changes: Vec<DMAccountChange>,
    pub lamport_changes: Vec<DMLamportChange>
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

    pub fn add_lamport_change(&mut self,  pubkey: Pubkey, pre: u64, post: u64) {
        self.lamport_changes.push(DMLamportChange{
            pubkey,
            pre,
            post
        });
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

    pub current_instruction_index: usize,

    pub instructions: Vec<DMInstruction>,
    pub logs: Vec<String>,
}

impl DMTransaction {
    pub fn start_instruction(&mut self, program_id: Pubkey, keyed_accounts: &[KeyedAccount], instruction_data: &[u8]) {
        let accounts: Vec<String> = keyed_accounts.into_iter().map(|i| format!("{}:{}{}", i.unsigned_key(), if i.is_signer() { 1 }  else { 0 }, if i.is_writable() { 1 }  else { 0 })).collect();
        let inst_ordinal = self.current_instruction_index.to_u32().unwrap_or(0);
        let mut inst = DMInstruction{
            accounts,
            program_id,
            ordinal: (inst_ordinal + 1),
            parent_ordinal: inst_ordinal,
            data: Vec::with_capacity(instruction_data.len()),
            account_changes: Vec::new(),
            lamport_changes: Vec::new(),
        };
        inst.data.copy_from_slice(instruction_data);
        self.instructions.push(inst);
        self.current_instruction_index = self.instructions.len();
    }

    pub fn end_instruction(&mut self) {
        self.current_instruction_index -= 1;
    }

    pub fn add_log(&mut self, log: String) {
       self.logs.push(log)
    }

    pub fn active_instruction(&mut self) -> &mut DMInstruction {
        return self.instructions[self.current_instruction_index].borrow_mut();
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
            current_instruction_index: 0,
            instructions: Vec::new(),
            logs: Vec::new(),
        })
    }

    pub fn flush(&self) {
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

    pub fn end_instruction(&mut self) {
        if let Some(transaction) = self.trxs.last_mut() {
            transaction.end_instruction()
        }
    }

    pub fn account_change(&mut self, pubkey: Pubkey, pre: &[u8], post: &[u8]) {
        if let Some(transaction) = self.trxs.last_mut() {
            let instruction = transaction.active_instruction();
            instruction.add_account_change(pubkey, pre, post);
        }
    }
    pub fn lamport_change(&mut self,pubkey: Pubkey, pre: u64, post: u64) {
        if let Some(transaction) = self.trxs.last_mut() {
            let instruction = transaction.active_instruction();
            instruction.add_lamport_change(pubkey, pre, post);
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