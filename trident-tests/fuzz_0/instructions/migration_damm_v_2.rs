use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN")]
#[discriminator([156u8, 169u8, 230u8, 103u8, 53u8, 228u8, 80u8, 64u8])]
pub struct MigrationDammV2Instruction {
    pub accounts: MigrationDammV2InstructionAccounts,
    pub data: MigrationDammV2InstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(MigrationDammV2InstructionData)]
#[storage(FuzzAccounts)]
pub struct MigrationDammV2InstructionAccounts {
    #[account(mut)]
    pub virtual_pool: TridentAccount,

    pub migration_metadata: TridentAccount,

    pub config: TridentAccount,

    #[account(mut, address = "FhVo3mqL8PW5pH5U2CN4XE33DokiyZnUwuGpH2hmHLuM")]
    pub pool_authority: TridentAccount,

    #[account(mut)]
    pub pool: TridentAccount,

    #[account(mut)]
    pub first_position_nft_mint: TridentAccount,

    #[account(mut)]
    pub first_position_nft_account: TridentAccount,

    #[account(mut)]
    pub first_position: TridentAccount,

    #[account(mut)]
    pub second_position_nft_mint: TridentAccount,

    #[account(mut)]
    pub second_position_nft_account: TridentAccount,

    #[account(mut)]
    pub second_position: TridentAccount,

    pub damm_pool_authority: TridentAccount,

    #[account(address = "cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG")]
    pub amm_program: TridentAccount,

    #[account(mut)]
    pub base_mint: TridentAccount,

    #[account(mut)]
    pub quote_mint: TridentAccount,

    #[account(mut)]
    pub token_a_vault: TridentAccount,

    #[account(mut)]
    pub token_b_vault: TridentAccount,

    #[account(mut)]
    pub base_vault: TridentAccount,

    #[account(mut)]
    pub quote_vault: TridentAccount,

    #[account(mut, signer)]
    pub payer: TridentAccount,

    pub token_base_program: TridentAccount,

    pub token_quote_program: TridentAccount,

    pub token_2022_program: TridentAccount,

    pub damm_event_authority: TridentAccount,

    #[account(address = "11111111111111111111111111111111")]
    pub system_program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct MigrationDammV2InstructionData {}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for MigrationDammV2Instruction {
    type IxAccounts = FuzzAccounts;
}
