


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



# Call tree leading to `replay_blockstore_into_bank`



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
