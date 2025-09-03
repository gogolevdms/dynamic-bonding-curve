use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN")]
#[discriminator([169u8, 118u8, 51u8, 78u8, 145u8, 110u8, 220u8, 155u8])]
pub struct InitializeVirtualPoolWithToken2022Instruction {
    pub accounts: InitializeVirtualPoolWithToken2022InstructionAccounts,
    pub data: InitializeVirtualPoolWithToken2022InstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(InitializeVirtualPoolWithToken2022InstructionData)]
#[storage(FuzzAccounts)]
pub struct InitializeVirtualPoolWithToken2022InstructionAccounts {
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

    #[account(mut, signer)]
    pub payer: TridentAccount,

    pub token_quote_program: TridentAccount,

    #[account(address = "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb")]
    pub token_program: TridentAccount,

    #[account(address = "11111111111111111111111111111111")]
    pub system_program: TridentAccount,

    pub event_authority: TridentAccount,

    pub program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct InitializeVirtualPoolWithToken2022InstructionData {
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
impl InstructionHooks for InitializeVirtualPoolWithToken2022Instruction {
    type IxAccounts = FuzzAccounts;
}
