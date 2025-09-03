use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN")]
#[discriminator([8u8, 236u8, 89u8, 49u8, 152u8, 125u8, 177u8, 81u8])]
pub struct ClaimTradingFeeInstruction {
    pub accounts: ClaimTradingFeeInstructionAccounts,
    pub data: ClaimTradingFeeInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(ClaimTradingFeeInstructionData)]
#[storage(FuzzAccounts)]
pub struct ClaimTradingFeeInstructionAccounts {
    #[account(address = "FhVo3mqL8PW5pH5U2CN4XE33DokiyZnUwuGpH2hmHLuM")]
    pub pool_authority: TridentAccount,

    pub config: TridentAccount,

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
    pub fee_claimer: TridentAccount,

    pub token_base_program: TridentAccount,

    pub token_quote_program: TridentAccount,

    pub event_authority: TridentAccount,

    pub program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct ClaimTradingFeeInstructionData {
    pub max_amount_a: u64,

    pub max_amount_b: u64,
}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for ClaimTradingFeeInstruction {
    type IxAccounts = FuzzAccounts;
}
