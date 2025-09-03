use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN")]
#[discriminator([82u8, 220u8, 250u8, 189u8, 3u8, 85u8, 107u8, 45u8])]
pub struct ClaimCreatorTradingFeeInstruction {
    pub accounts: ClaimCreatorTradingFeeInstructionAccounts,
    pub data: ClaimCreatorTradingFeeInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(ClaimCreatorTradingFeeInstructionData)]
#[storage(FuzzAccounts)]
pub struct ClaimCreatorTradingFeeInstructionAccounts {
    #[account(address = "FhVo3mqL8PW5pH5U2CN4XE33DokiyZnUwuGpH2hmHLuM")]
    pub pool_authority: TridentAccount,

    #[account(mut)]
    pub pool: TridentAccount,

    #[account(mut)]
    pub token_a_account: TridentAccount,

    #[account(mut)]
    pub token_b_account: TridentAccount,

    #[account(mut)]
    pub base_vault: TridentAccount,

    #[account(mut)]
    pub quote_vault: TridentAccount,

    pub base_mint: TridentAccount,

    pub quote_mint: TridentAccount,

    #[account(signer)]
    pub creator: TridentAccount,

    pub token_base_program: TridentAccount,

    pub token_quote_program: TridentAccount,

    pub event_authority: TridentAccount,

    pub program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct ClaimCreatorTradingFeeInstructionData {
    pub max_base_amount: u64,

    pub max_quote_amount: u64,
}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for ClaimCreatorTradingFeeInstruction {
    type IxAccounts = FuzzAccounts;
}
