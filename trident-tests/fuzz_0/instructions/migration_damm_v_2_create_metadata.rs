use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN")]
#[discriminator([109u8, 189u8, 19u8, 36u8, 195u8, 183u8, 222u8, 82u8])]
pub struct MigrationDammV2CreateMetadataInstruction {
    pub accounts: MigrationDammV2CreateMetadataInstructionAccounts,
    pub data: MigrationDammV2CreateMetadataInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(MigrationDammV2CreateMetadataInstructionData)]
#[storage(FuzzAccounts)]
pub struct MigrationDammV2CreateMetadataInstructionAccounts {
    pub virtual_pool: TridentAccount,

    pub config: TridentAccount,

    #[account(mut)]
    pub migration_metadata: TridentAccount,

    #[account(mut, signer)]
    pub payer: TridentAccount,

    #[account(address = "11111111111111111111111111111111")]
    pub system_program: TridentAccount,

    pub event_authority: TridentAccount,

    pub program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct MigrationDammV2CreateMetadataInstructionData {}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for MigrationDammV2CreateMetadataInstruction {
    type IxAccounts = FuzzAccounts;
}
