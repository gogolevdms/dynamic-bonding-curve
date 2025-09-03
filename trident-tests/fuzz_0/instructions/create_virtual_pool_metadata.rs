use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN")]
#[discriminator([45u8, 97u8, 187u8, 103u8, 254u8, 109u8, 124u8, 134u8])]
pub struct CreateVirtualPoolMetadataInstruction {
    pub accounts: CreateVirtualPoolMetadataInstructionAccounts,
    pub data: CreateVirtualPoolMetadataInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(CreateVirtualPoolMetadataInstructionData)]
#[storage(FuzzAccounts)]
pub struct CreateVirtualPoolMetadataInstructionAccounts {
    #[account(mut)]
    pub virtual_pool: TridentAccount,

    #[account(mut)]
    pub virtual_pool_metadata: TridentAccount,

    #[account(signer)]
    pub creator: TridentAccount,

    #[account(mut, signer)]
    pub payer: TridentAccount,

    #[account(address = "11111111111111111111111111111111")]
    pub system_program: TridentAccount,

    pub event_authority: TridentAccount,

    pub program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct CreateVirtualPoolMetadataInstructionData {
    pub metadata: CreateVirtualPoolMetadataParameters,
}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for CreateVirtualPoolMetadataInstruction {
    type IxAccounts = FuzzAccounts;
}
