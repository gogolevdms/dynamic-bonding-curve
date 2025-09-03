use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN")]
#[discriminator([47u8, 94u8, 126u8, 115u8, 221u8, 226u8, 194u8, 133u8])]
pub struct MigrationMeteoraDammCreateMetadataInstruction {
    pub accounts: MigrationMeteoraDammCreateMetadataInstructionAccounts,
    pub data: MigrationMeteoraDammCreateMetadataInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(MigrationMeteoraDammCreateMetadataInstructionData)]
#[storage(FuzzAccounts)]
pub struct MigrationMeteoraDammCreateMetadataInstructionAccounts {
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
pub struct MigrationMeteoraDammCreateMetadataInstructionData {}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for MigrationMeteoraDammCreateMetadataInstruction {
    type IxAccounts = FuzzAccounts;
}
