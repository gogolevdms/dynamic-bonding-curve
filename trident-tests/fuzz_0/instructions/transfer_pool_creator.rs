use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN")]
#[discriminator([20u8, 7u8, 169u8, 33u8, 58u8, 147u8, 166u8, 33u8])]
pub struct TransferPoolCreatorInstruction {
    pub accounts: TransferPoolCreatorInstructionAccounts,
    pub data: TransferPoolCreatorInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(TransferPoolCreatorInstructionData)]
#[storage(FuzzAccounts)]
pub struct TransferPoolCreatorInstructionAccounts {
    #[account(mut)]
    pub virtual_pool: TridentAccount,

    pub config: TridentAccount,

    #[account(signer)]
    pub creator: TridentAccount,

    pub new_creator: TridentAccount,

    pub event_authority: TridentAccount,

    pub program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct TransferPoolCreatorInstructionData {}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for TransferPoolCreatorInstruction {
    type IxAccounts = FuzzAccounts;
}
