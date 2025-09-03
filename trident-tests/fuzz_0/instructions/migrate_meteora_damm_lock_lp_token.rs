use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN")]
#[discriminator([177u8, 55u8, 238u8, 157u8, 251u8, 88u8, 165u8, 42u8])]
pub struct MigrateMeteoraDammLockLpTokenInstruction {
    pub accounts: MigrateMeteoraDammLockLpTokenInstructionAccounts,
    pub data: MigrateMeteoraDammLockLpTokenInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(MigrateMeteoraDammLockLpTokenInstructionData)]
#[storage(FuzzAccounts)]
pub struct MigrateMeteoraDammLockLpTokenInstructionAccounts {
    pub virtual_pool: TridentAccount,

    #[account(mut)]
    pub migration_metadata: TridentAccount,

    #[account(mut, address = "FhVo3mqL8PW5pH5U2CN4XE33DokiyZnUwuGpH2hmHLuM")]
    pub pool_authority: TridentAccount,

    #[account(mut)]
    pub pool: TridentAccount,

    pub lp_mint: TridentAccount,

    #[account(mut)]
    pub lock_escrow: TridentAccount,

    pub owner: TridentAccount,

    #[account(mut)]
    pub source_tokens: TridentAccount,

    #[account(mut)]
    pub escrow_vault: TridentAccount,

    #[account(address = "Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB")]
    pub amm_program: TridentAccount,

    pub a_vault: TridentAccount,

    pub b_vault: TridentAccount,

    pub a_vault_lp: TridentAccount,

    pub b_vault_lp: TridentAccount,

    pub a_vault_lp_mint: TridentAccount,

    pub b_vault_lp_mint: TridentAccount,

    #[account(address = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")]
    pub token_program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct MigrateMeteoraDammLockLpTokenInstructionData {}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for MigrateMeteoraDammLockLpTokenInstruction {
    type IxAccounts = FuzzAccounts;
}
