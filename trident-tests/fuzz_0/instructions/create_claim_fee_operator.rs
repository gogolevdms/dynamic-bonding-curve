use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN")]
#[discriminator([169u8, 62u8, 207u8, 107u8, 58u8, 187u8, 162u8, 109u8])]
pub struct CreateClaimFeeOperatorInstruction {
    pub accounts: CreateClaimFeeOperatorInstructionAccounts,
    pub data: CreateClaimFeeOperatorInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(CreateClaimFeeOperatorInstructionData)]
#[storage(FuzzAccounts)]
pub struct CreateClaimFeeOperatorInstructionAccounts {
    #[account(mut)]
    pub claim_fee_operator: TridentAccount,

    pub operator: TridentAccount,

    #[account(mut, signer)]
    pub admin: TridentAccount,

    #[account(address = "11111111111111111111111111111111")]
    pub system_program: TridentAccount,

    pub event_authority: TridentAccount,

    pub program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct CreateClaimFeeOperatorInstructionData {}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for CreateClaimFeeOperatorInstruction {
    type IxAccounts = FuzzAccounts;
}
