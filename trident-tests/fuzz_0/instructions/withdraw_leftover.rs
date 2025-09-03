use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN")]
#[discriminator([20u8, 198u8, 202u8, 237u8, 235u8, 243u8, 183u8, 66u8])]
pub struct WithdrawLeftoverInstruction {
    pub accounts: WithdrawLeftoverInstructionAccounts,
    pub data: WithdrawLeftoverInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(WithdrawLeftoverInstructionData)]
#[storage(FuzzAccounts)]
pub struct WithdrawLeftoverInstructionAccounts {
    #[account(address = "FhVo3mqL8PW5pH5U2CN4XE33DokiyZnUwuGpH2hmHLuM")]
    pub pool_authority: TridentAccount,

    pub config: TridentAccount,

    #[account(mut)]
    pub virtual_pool: TridentAccount,

    #[account(mut)]
    pub token_base_account: TridentAccount,

    #[account(mut)]
    pub base_vault: TridentAccount,

    pub base_mint: TridentAccount,

    pub leftover_receiver: TridentAccount,

    pub token_base_program: TridentAccount,

    pub event_authority: TridentAccount,

    pub program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct WithdrawLeftoverInstructionData {}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for WithdrawLeftoverInstruction {
    type IxAccounts = FuzzAccounts;
}
