# Instruction Execution
- Top Level instructions get execute by  `execute_instruction` -> `runtime/src/message_processor.rs:888`
- The function `process_instruction` is called to run the top level instruction
- When a top level instruction run a sub-instruction that will call `process_cross_program_instruction` ->`runtime/src/message_processor.rs:687`

* Scenario: Instruction A and sub-instruction B both change the same account (A before and after B's execution) 
-> Transaction A `execute_instruction`
    -> Instruction A `fn process_instruction()`
        -> `ACC_CHANGE INST_A PUB_KEY 0 -> 1`
        ->  Instruction B `process_cross_program_instruction`
            * `verify_and_update`       Will check that the partial execution of Instruction A 
                                        modified the correct accounts & update them. A this point we would print out
                                        the "partial" account change of Instruction A.
            * `fn process_instruction()`     Will execute Instruction B
            -> `ACC_CHANGE INST_B PUB_KEY 1 -> 2`
            * `verify_and_update`       Will check that the full execution of Instruction B 
                                        modified the correct accounts & update them. At this point 
                                        we would print out ACCOUNT_CHANGES for instruction B
        ACC_CHANGE PUB_KEY 2 -> 3
    * `verify_and_update`       Will check that the full final execution of Instruction A
                                modified the correct accounts & update them. At this point
                                we would print ACCOUNT_CHANGES for instruction A's "post B Instruction execution"

In this example we would print out 3 account changes
1) DMLOG ACCT_CH INST_A PUB_KEY 0 1
2) DMLOG ACCT_CH INST_B PUB_KEY 1 2
3) DMLOG ACCT_CH INST_A PUB_KEY 2 3

* Scenario: Instruction A and sub-instruction B both change the same account (A only before B's execution)
  -> Transaction A `execute_instruction`
    -> Instruction A `process_instruction`
        -> `ACC_CHANGE INST_A PUB_KEY 0 -> 1`
        ->  Instruction B `process_cross_program_instruction`
            * `verify_and_update` prints an account change
            * `process_instruction`     Will execute Instruction B
            -> `ACC_CHANGE INST_B PUB_KEY 1 -> 2`
            * `verify_and_update` prints an account change
    * `verify_and_update` does not print an account change

1) DMLOG ACCT_CH INST_A PUB_KEY 0 1
2) DMLOG ACCT_CH INST_B PUB_KEY 1 2

* Scenario: Instruction A and sub-instruction B both change the same account (A only after B's execution)
  -> Transaction A `execute_instruction`
    -> Instruction A `process_instruction`
        ->  Instruction B `process_cross_program_instruction`
            * `verify_and_update` does not print anything
            * `process_instruction`     Will execute Instruction B
            -> `ACC_CHANGE INST_B PUB_KEY 0 -> 1`
            * `verify_and_update` prints an account change
        -> `ACC_CHANGE INST_A PUB_KEY 1 -> 2`
        * `verify_and_update` prints an account change

1) DMLOG ACCT_CH INST_B PUB_KEY 0 1
2) DMLOG ACCT_CH INST_A PUB_KEY 1 2



# Call tree
> CONSTRUCTOR! This is PRETTY top level
core/src/validator.rs:221          impl Validator  // pub fn new(

     calls into `new_banks_from_ledger`

core/src/validator.rs:859          fn new_banks_from_ledger(

     calls into `load()`:

ledger/src/bank_forks_utils.rs:32   pub fn load(

    calls into `process_blockstore_from_root()`


> // seems to be to initialize ONLY the slot 0  IGNORE ME FOR NOW
ledger/src/blockstore_processor.rs:339:  fn process_blockstore(
> This ONLY skip the bank0, and continues
ledger/src/blockstore_processor.rs:371:  pub(crate) fn process_blockstore_from_root(  // thin wrapper around `do_process_blockstore_from_root`

    BOTH call into `do_process_blockstore_from_root`

> CONVERGE INTO THIS:
> Does initializations,
> Sets hard forks numbers in the Bank
> Ensures it starts from a `root` slot, with no parents
> Prints the last slot we have shreds about
> Loads frozen forks, and returns
> This func does NOT Start long running processes
> This func does NOT seem to block for a huge amount of time, because
>     its caller is waiting.
ledger/src/blockstore_processor.rs:387:  fn do_process_blockstore_from_root(

    calls into `load_frozen_forks()`

> We loop in there while there are `!pending_slots.is_empty()`
> Perhaps what's left in the blockstore already, without touching the network?
ledger/src/blockstore_processor.rs:864:  fn load_frozen_forks(

    calls into `process_single_slot`

ledger/src/blockstore_processor.rs:1074:  fn process_single_slot(
ledger/src/blockstore_processor.rs:783:  fn process_bank_0(

    calls into `confirm_full_slot()`:

ledger/src/blockstore_processor.rs:544:    confirm_full_slot(
core/src/replay_stage.rs:947:            fn replay_blockstore_into_bank(
    both call into `blockstore_processor.rs:616 confirm_slot()`

> Seems that all of this call tree above stems from initialization ^^
> Probably used when processing `pending_slots` from block storage when
> it's available on disk, but not for LIVE.
>
> Is the REPLAY blockstore_into_bank the "live" aspect of it?

## Call tree leading to `replay_blockstore_into_bank`

> Schedules a thread that will do:
  > Executes some transactions
  > Then checks the heaviest forks
  > Then votes on those slots according to the voting algos, etc..
core/src/replay_stage.rs:213:        impl ReplayStage::new

    calls into `replay_active_banks()`

core/src/replay_stage.rs:1252:            fn replay_active_banks(

    calls into `replay_blockstore_into_bank()`

core/src/replay_stage.rs:947:            fn replay_blockstore_into_bank(

    both call into: `confirm_slot()`


> THIS IS THE COMMON ENTRY POINT TO BOTH INITIALIZATION-TIME PROCESSING
> AND REAL-TIME PROCESSING
ledger/src/blockstore_processor.rs:616:pub fn confirm_slot(

    calls into `process_entries_with_callback()`

ledger/src/blockstore_processor.rs:175     fn process_entries()   SOLELY USED IN TESTS
ledger/src/blockstore_processor.rs:193     fn process_entries_with_callback()

    calls into `execute_batches` a bunch of times


------------------------

blockstore_processor.rs:143   execute_batches runs `execute_batch()` each in its own thread
blockstore_processor.rs:101   execute_batch  runs `load_and_execute_and_commit_transcations()` with the batch
bank.rs:3706                  `load_execute_and_commit_transactions()` calls `self.load_and_execute_transactions` with the batch

bank.rs:2758                  `load_and_execute_transactions` loops through transactions, and executes them, starting a trx, calling for instructions in `message_processor.process_message()`, printing logs and printing the end of a trx.

message_processor.rs        `process_message` executes the instruction,


ThisInvokeContext
  MessageProcessor
    PreAccount


message_processor.rS:976    fn process_message()
    -> loop les instructions TOP-LEVEL du Message, and call `execute_instruction` on it.

message_processor.rs:886    fn execute_instruction()
    -> creates the PreAccounts
    -> creates ThisInvokeContext
    -> calls `process_instruction()`  GOING DEEPER HERE
    -> then calls `MessageProcessor.verify()` on this instruction, passing along
       the INVOKE CONTEXT's `pre_accounts`.

message_processor.rs:498    fn process_instruction
    -> calls the `process_instruction` from either the BPFLoader, the NativeLoader or some other
       program's entrypoint.

IN BPF LOADER:

bpf_loader/lib.rs:146    fn process_instruction(.. InvokeContext)
    -> calls into `process_loader_upgradable_instruction(.. InvokeContext)`

bpf_loader/lib.rs:233    fn process_loader_upgradable_instruction(.. InvokeContext)
    -> calls into `MessageProcessor::native_invoke()` on line 341

BACK IN MESSAGE PROCESSOR:

message_processor.rs:595    fn native_invoke(InvokeContext, ...)
    <- called by `bpf_loader/lib.rs:233`
    -> calls into `MessageProcessor::process_cross_program_instruction()`

message_processor.rs:684    fn process_cross_program_instruction
    -> calls it InvokeContext::verify_and_update()
    -> calls it InvokeContext::process_instruction() -> GOES INTO RECURSIVE CALLS
    -> calls it InvokeContext::verify_and_update()   AGAIN

message_processor.rs:264    fn ThisInvokeContext::verify_and_update()
    -> calls into MessageProcessor::verify_and_update()

message_processor.rs:836    fn MessageProcessor::verify_and_update()
    -> calls PreAccount::verify()
    -> calls PreAccount::update()

message_processor.rs:781    fn MessageProcessor::verify()
    <- only called by `execute_instruction()`, which is very high-level, for TOP-LEVEL instructions.
    -> calls `PreAccount::verify()`

message_processor.rs:71     fn PreAccount::verify()
