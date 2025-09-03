use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN")]
#[discriminator([167u8, 90u8, 137u8, 154u8, 75u8, 47u8, 17u8, 84u8])]
pub struct CreateLockerInstruction {
    pub accounts: CreateLockerInstructionAccounts,
    pub data: CreateLockerInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(CreateLockerInstructionData)]
#[storage(FuzzAccounts)]
pub struct CreateLockerInstructionAccounts {
    #[account(mut)]
    pub virtual_pool: TridentAccount,

    pub config: TridentAccount,

    #[account(mut, address = "FhVo3mqL8PW5pH5U2CN4XE33DokiyZnUwuGpH2hmHLuM")]
    pub pool_authority: TridentAccount,

    #[account(mut)]
    pub base_vault: TridentAccount,

    #[account(mut)]
    pub base_mint: TridentAccount,

    #[account(mut)]
    pub base: TridentAccount,

    pub creator: TridentAccount,

    #[account(mut)]
    pub escrow: TridentAccount,

    #[account(mut)]
    pub escrow_token: TridentAccount,

    #[account(mut, signer)]
    pub payer: TridentAccount,

    pub token_program: TridentAccount,

    #[account(address = "LocpQgucEQHbqNABEYvBvwoxCPsSbG91A1QaQhQQqjn")]
    pub locker_program: TridentAccount,

    pub locker_event_authority: TridentAccount,

    #[account(address = "11111111111111111111111111111111")]
    pub system_program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct CreateLockerInstructionData {}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for CreateLockerInstruction {
    type IxAccounts = FuzzAccounts;
}
