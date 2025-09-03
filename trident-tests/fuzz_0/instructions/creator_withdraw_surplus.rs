use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN")]
#[discriminator([165u8, 3u8, 137u8, 7u8, 28u8, 134u8, 76u8, 80u8])]
pub struct CreatorWithdrawSurplusInstruction {
    pub accounts: CreatorWithdrawSurplusInstructionAccounts,
    pub data: CreatorWithdrawSurplusInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(CreatorWithdrawSurplusInstructionData)]
#[storage(FuzzAccounts)]
pub struct CreatorWithdrawSurplusInstructionAccounts {
    #[account(address = "FhVo3mqL8PW5pH5U2CN4XE33DokiyZnUwuGpH2hmHLuM")]
    pub pool_authority: TridentAccount,

    pub config: TridentAccount,

    #[account(mut)]
    pub virtual_pool: TridentAccount,

    #[account(mut)]
    pub token_quote_account: TridentAccount,

    #[account(mut)]
    pub quote_vault: TridentAccount,

    pub quote_mint: TridentAccount,

    #[account(signer)]
    pub creator: TridentAccount,

    pub token_quote_program: TridentAccount,

    pub event_authority: TridentAccount,

    pub program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct CreatorWithdrawSurplusInstructionData {}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for CreatorWithdrawSurplusInstruction {
    type IxAccounts = FuzzAccounts;
}
