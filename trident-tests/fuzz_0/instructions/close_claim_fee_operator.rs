use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN")]
#[discriminator([38u8, 134u8, 82u8, 216u8, 95u8, 124u8, 17u8, 99u8])]
pub struct CloseClaimFeeOperatorInstruction {
    pub accounts: CloseClaimFeeOperatorInstructionAccounts,
    pub data: CloseClaimFeeOperatorInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(CloseClaimFeeOperatorInstructionData)]
#[storage(FuzzAccounts)]
pub struct CloseClaimFeeOperatorInstructionAccounts {
    #[account(mut)]
    pub claim_fee_operator: TridentAccount,

    #[account(mut)]
    pub rent_receiver: TridentAccount,

    #[account(signer)]
    pub admin: TridentAccount,

    pub event_authority: TridentAccount,

    pub program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct CloseClaimFeeOperatorInstructionData {}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for CloseClaimFeeOperatorInstruction {
    type IxAccounts = FuzzAccounts;
}
