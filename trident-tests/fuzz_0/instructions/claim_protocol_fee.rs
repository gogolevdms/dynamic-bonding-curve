use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN")]
#[discriminator([165u8, 228u8, 133u8, 48u8, 99u8, 249u8, 255u8, 33u8])]
pub struct ClaimProtocolFeeInstruction {
    pub accounts: ClaimProtocolFeeInstructionAccounts,
    pub data: ClaimProtocolFeeInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(ClaimProtocolFeeInstructionData)]
#[storage(FuzzAccounts)]
pub struct ClaimProtocolFeeInstructionAccounts {
    #[account(address = "FhVo3mqL8PW5pH5U2CN4XE33DokiyZnUwuGpH2hmHLuM")]
    pub pool_authority: TridentAccount,

    pub config: TridentAccount,

    #[account(mut)]
    pub pool: TridentAccount,

    #[account(mut)]
    pub base_vault: TridentAccount,

    #[account(mut)]
    pub quote_vault: TridentAccount,

    pub base_mint: TridentAccount,

    pub quote_mint: TridentAccount,

    #[account(mut)]
    pub token_base_account: TridentAccount,

    #[account(mut)]
    pub token_quote_account: TridentAccount,

    pub claim_fee_operator: TridentAccount,

    #[account(signer)]
    pub operator: TridentAccount,

    pub token_base_program: TridentAccount,

    pub token_quote_program: TridentAccount,

    pub event_authority: TridentAccount,

    pub program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct ClaimProtocolFeeInstructionData {}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for ClaimProtocolFeeInstruction {
    type IxAccounts = FuzzAccounts;
}
