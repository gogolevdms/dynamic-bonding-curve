use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN")]
#[discriminator([139u8, 133u8, 2u8, 30u8, 91u8, 145u8, 127u8, 154u8])]
pub struct MigrateMeteoraDammClaimLpTokenInstruction {
    pub accounts: MigrateMeteoraDammClaimLpTokenInstructionAccounts,
    pub data: MigrateMeteoraDammClaimLpTokenInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(MigrateMeteoraDammClaimLpTokenInstructionData)]
#[storage(FuzzAccounts)]
pub struct MigrateMeteoraDammClaimLpTokenInstructionAccounts {
    pub virtual_pool: TridentAccount,

    #[account(mut)]
    pub migration_metadata: TridentAccount,

    #[account(mut, address = "FhVo3mqL8PW5pH5U2CN4XE33DokiyZnUwuGpH2hmHLuM")]
    pub pool_authority: TridentAccount,

    pub lp_mint: TridentAccount,

    #[account(mut)]
    pub source_token: TridentAccount,

    #[account(mut)]
    pub destination_token: TridentAccount,

    pub owner: TridentAccount,

    #[account(signer)]
    pub sender: TridentAccount,

    #[account(address = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")]
    pub token_program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct MigrateMeteoraDammClaimLpTokenInstructionData {}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for MigrateMeteoraDammClaimLpTokenInstruction {
    type IxAccounts = FuzzAccounts;
}
