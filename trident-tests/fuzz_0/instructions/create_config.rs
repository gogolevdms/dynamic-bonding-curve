use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN")]
#[discriminator([201u8, 207u8, 243u8, 114u8, 75u8, 111u8, 47u8, 189u8])]
pub struct CreateConfigInstruction {
    pub accounts: CreateConfigInstructionAccounts,
    pub data: CreateConfigInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(CreateConfigInstructionData)]
#[storage(FuzzAccounts)]
pub struct CreateConfigInstructionAccounts {
    #[account(mut, signer)]
    pub config: TridentAccount,

    pub fee_claimer: TridentAccount,

    pub leftover_receiver: TridentAccount,

    pub quote_mint: TridentAccount,

    #[account(mut, signer)]
    pub payer: TridentAccount,

    #[account(address = "11111111111111111111111111111111")]
    pub system_program: TridentAccount,

    pub event_authority: TridentAccount,

    pub program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct CreateConfigInstructionData {
    pub config_parameters: ConfigParameters,
}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for CreateConfigInstruction {
    type IxAccounts = FuzzAccounts;
}
