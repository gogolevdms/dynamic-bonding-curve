use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN")]
#[discriminator([192u8, 168u8, 234u8, 191u8, 188u8, 226u8, 227u8, 255u8])]
pub struct CreatePartnerMetadataInstruction {
    pub accounts: CreatePartnerMetadataInstructionAccounts,
    pub data: CreatePartnerMetadataInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(CreatePartnerMetadataInstructionData)]
#[storage(FuzzAccounts)]
pub struct CreatePartnerMetadataInstructionAccounts {
    #[account(mut)]
    pub partner_metadata: TridentAccount,

    #[account(mut, signer)]
    pub payer: TridentAccount,

    #[account(signer)]
    pub fee_claimer: TridentAccount,

    #[account(address = "11111111111111111111111111111111")]
    pub system_program: TridentAccount,

    pub event_authority: TridentAccount,

    pub program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct CreatePartnerMetadataInstructionData {
    pub metadata: CreatePartnerMetadataParameters,
}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for CreatePartnerMetadataInstruction {
    type IxAccounts = FuzzAccounts;
}
