use solana_sdk::{
    keyed_account::KeyedAccount,
    pubkey::Pubkey,
};
use std::{
    fs::File,
    io::Write,
    borrow::BorrowMut,
    sync::atomic::{AtomicBool, Ordering},
};
use solana_sdk::pb::codec::{
    AccountChange,
    BalanceChange,
    Instruction,
};
use hex;
use solana_program::hash::Hash;
use num_traits::ToPrimitive;
use protobuf::RepeatedField;

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

pub struct DMInstruction {
    pub ordinal: u32,
    pub parent_ordinal: u32,
    pub data: Vec<u8>,
    pub program_id: Pubkey,
    pub accounts: Vec<String>,
    pub account_changes: Vec<AccountChange>,
    pub lamport_changes: Vec<BalanceChange>
}

impl DMInstruction {
    pub fn add_account_change(&mut self,  pubkey: Pubkey, pre: &[u8], post: &[u8]) {
        let post_len = post.len();
        let mut account = AccountChange{
            pubkey: format!("{}", pubkey),
            prev_data: Vec::with_capacity(pre.len()),
            new_data: Vec::with_capacity(post_len),
            new_data_length: post_len.to_u64().unwrap_or(0),
            ..Default::default()
        };
        account.prev_data.copy_from_slice(pre);
        account.new_data.copy_from_slice(post);
        self.account_changes.push(account);
    }

    pub fn add_lamport_change(&mut self,  pubkey: Pubkey, pre: u64, post: u64) {
        self.lamport_changes.push(BalanceChange{
            pubkey: format!("{}", pubkey),
            prev_lamports: pre,
            new_lamports: post,
            ..Default::default()
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

    pub instructions: Vec<Instruction>,
    pub logs: Vec<String>,
}

impl DMTransaction {
    pub fn start_instruction(&mut self, program_id: Pubkey, keyed_accounts: &[KeyedAccount], instruction_data: &[u8]) {
        let accounts: RepeatedField<String> = keyed_accounts.into_iter().map(
            |i| format!("{}", i.unsigned_key())
        ).collect();

        let inst_ordinal = self.current_instruction_index.to_u32().unwrap_or(0);
        let mut inst = Instruction{
            program_id: format!("{}", program_id),
            account_keys: accounts,
            data: Vec::with_capacity(instruction_data.len()),
            ordinal: (inst_ordinal + 1),
            parent_ordinal: inst_ordinal,
            depth: 0,
            balance_changes: RepeatedField::default(),
            account_changes: RepeatedField::default(),
            ..Default::default()
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

pub struct DMBatchContext {
    pub batch_number: u64,
    pub trxs: Vec<DMTransaction>,
    pub file: File,
    pub path: String,
}

impl<'a> DMBatchContext {
    pub fn start_trx(&mut self, sigs: String, num_required_signatures: u8,num_readonly_signed_accounts: u8,num_readonly_unsigned_accounts: u8, account_keys: String, recent_blockhash: Hash) {

        // let cnt = format!("TRX_START {} {} {} {} {} {}",
        //          sigs,
        //          num_required_signatures,
        //          num_readonly_signed_accounts,
        //          num_readonly_unsigned_accounts,
        //          account_keys,
        //          recent_blockhash,
        // );

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
        //
        // loop through transations, and instructions, and logs and whateve, and print it all out
        // in a format ConsoleReader appreciated.



        // if let Err(e) = self.file.write_all(cnt.as_bytes()) {
        //     println!("DMLOG FILE_ERROR {}", e);
        //     return;
        // }

        if let Err(e) = self.file.sync_all() {
            println!("DMLOG FILE_ERROR {}", e);
            return;
        }

        drop(&self.file);
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
        if let Some(transaction) = self.trxs.last_mut() {
            transaction.add_log(log);
        }
    }
}
