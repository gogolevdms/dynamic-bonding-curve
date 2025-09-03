use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN")]
#[discriminator([140u8, 85u8, 215u8, 176u8, 102u8, 54u8, 104u8, 79u8])]
pub struct InitializeVirtualPoolWithSplTokenInstruction {
    pub accounts: InitializeVirtualPoolWithSplTokenInstructionAccounts,
    pub data: InitializeVirtualPoolWithSplTokenInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(InitializeVirtualPoolWithSplTokenInstructionData)]
#[storage(FuzzAccounts)]
pub struct InitializeVirtualPoolWithSplTokenInstructionAccounts {
    pub config: TridentAccount,

    #[account(address = "FhVo3mqL8PW5pH5U2CN4XE33DokiyZnUwuGpH2hmHLuM")]
    pub pool_authority: TridentAccount,

    #[account(signer)]
    pub creator: TridentAccount,

    #[account(mut, signer)]
    pub base_mint: TridentAccount,

    pub quote_mint: TridentAccount,

    #[account(mut)]
    pub pool: TridentAccount,

    #[account(mut)]
    pub base_vault: TridentAccount,

    #[account(mut)]
    pub quote_vault: TridentAccount,

    #[account(mut)]
    pub mint_metadata: TridentAccount,

    #[account(address = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")]
    pub metadata_program: TridentAccount,

    #[account(mut, signer)]
    pub payer: TridentAccount,

    pub token_quote_program: TridentAccount,

    #[account(address = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")]
    pub token_program: TridentAccount,

    #[account(address = "11111111111111111111111111111111")]
    pub system_program: TridentAccount,

    pub event_authority: TridentAccount,

    pub program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct InitializeVirtualPoolWithSplTokenInstructionData {
    pub params: InitializePoolParameters,
}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for InitializeVirtualPoolWithSplTokenInstruction {
    type IxAccounts = FuzzAccounts;
}
