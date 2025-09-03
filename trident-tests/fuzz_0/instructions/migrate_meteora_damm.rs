use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN")]
#[discriminator([27u8, 1u8, 48u8, 22u8, 180u8, 63u8, 118u8, 217u8])]
pub struct MigrateMeteoraDammInstruction {
    pub accounts: MigrateMeteoraDammInstructionAccounts,
    pub data: MigrateMeteoraDammInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(MigrateMeteoraDammInstructionData)]
#[storage(FuzzAccounts)]
pub struct MigrateMeteoraDammInstructionAccounts {
    #[account(mut)]
    pub virtual_pool: TridentAccount,

    #[account(mut)]
    pub migration_metadata: TridentAccount,

    pub config: TridentAccount,

    #[account(mut, address = "FhVo3mqL8PW5pH5U2CN4XE33DokiyZnUwuGpH2hmHLuM")]
    pub pool_authority: TridentAccount,

    #[account(mut)]
    pub pool: TridentAccount,

    pub damm_config: TridentAccount,

    #[account(mut)]
    pub lp_mint: TridentAccount,

    #[account(mut)]
    pub token_a_mint: TridentAccount,

    pub token_b_mint: TridentAccount,

    #[account(mut)]
    pub a_vault: TridentAccount,

    #[account(mut)]
    pub b_vault: TridentAccount,

    #[account(mut)]
    pub a_token_vault: TridentAccount,

    #[account(mut)]
    pub b_token_vault: TridentAccount,

    #[account(mut)]
    pub a_vault_lp_mint: TridentAccount,

    #[account(mut)]
    pub b_vault_lp_mint: TridentAccount,

    #[account(mut)]
    pub a_vault_lp: TridentAccount,

    #[account(mut)]
    pub b_vault_lp: TridentAccount,

    #[account(mut)]
    pub base_vault: TridentAccount,

    #[account(mut)]
    pub quote_vault: TridentAccount,

    #[account(mut)]
    pub virtual_pool_lp: TridentAccount,

    #[account(mut)]
    pub protocol_token_a_fee: TridentAccount,

    #[account(mut)]
    pub protocol_token_b_fee: TridentAccount,

    #[account(mut, signer)]
    pub payer: TridentAccount,

    pub rent: TridentAccount,

    #[account(mut)]
    pub mint_metadata: TridentAccount,

    pub metadata_program: TridentAccount,

    #[account(address = "Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB")]
    pub amm_program: TridentAccount,

    pub vault_program: TridentAccount,

    #[account(address = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")]
    pub token_program: TridentAccount,

    pub associated_token_program: TridentAccount,

    #[account(address = "11111111111111111111111111111111")]
    pub system_program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct MigrateMeteoraDammInstructionData {}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for MigrateMeteoraDammInstruction {
    type IxAccounts = FuzzAccounts;
}
