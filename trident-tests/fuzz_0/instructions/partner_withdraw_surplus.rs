use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN")]
#[discriminator([168u8, 173u8, 72u8, 100u8, 201u8, 98u8, 38u8, 92u8])]
pub struct PartnerWithdrawSurplusInstruction {
    pub accounts: PartnerWithdrawSurplusInstructionAccounts,
    pub data: PartnerWithdrawSurplusInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(PartnerWithdrawSurplusInstructionData)]
#[storage(FuzzAccounts)]
pub struct PartnerWithdrawSurplusInstructionAccounts {
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
    pub fee_claimer: TridentAccount,

    pub token_quote_program: TridentAccount,

    pub event_authority: TridentAccount,

    pub program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct PartnerWithdrawSurplusInstructionData {}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for PartnerWithdrawSurplusInstruction {
    type IxAccounts = FuzzAccounts;
}
