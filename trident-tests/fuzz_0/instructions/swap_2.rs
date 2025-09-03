use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN")]
#[discriminator([65u8, 75u8, 63u8, 76u8, 235u8, 91u8, 91u8, 136u8])]
pub struct Swap2Instruction {
    pub accounts: Swap2InstructionAccounts,
    pub data: Swap2InstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(Swap2InstructionData)]
#[storage(FuzzAccounts)]
pub struct Swap2InstructionAccounts {
    #[account(address = "FhVo3mqL8PW5pH5U2CN4XE33DokiyZnUwuGpH2hmHLuM")]
    pub pool_authority: TridentAccount,

    pub config: TridentAccount,

    #[account(mut)]
    pub pool: TridentAccount,

    #[account(mut)]
    pub input_token_account: TridentAccount,

    #[account(mut)]
    pub output_token_account: TridentAccount,

    #[account(mut)]
    pub base_vault: TridentAccount,

    #[account(mut)]
    pub quote_vault: TridentAccount,

    pub base_mint: TridentAccount,

    pub quote_mint: TridentAccount,

    #[account(signer)]
    pub payer: TridentAccount,

    pub token_base_program: TridentAccount,

    pub token_quote_program: TridentAccount,

    #[account(mut)]
    pub referral_token_account: TridentAccount,

    pub event_authority: TridentAccount,

    pub program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct Swap2InstructionData {
    pub params: SwapParameters2,
}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for Swap2Instruction {
    type IxAccounts = FuzzAccounts;
}
