use trident_fuzz::fuzzing::*;

/// FuzzAccounts contains all available accounts
///
/// You can create your own accounts by adding new fields to the struct.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-api-macro/trident-types/fuzz-accounts/
#[derive(Default)]
pub struct FuzzAccounts {
    pub amm_program: AccountsStorage,

    pub damm_event_authority: AccountsStorage,

    pub token_b_account: AccountsStorage,

    pub pool_authority: AccountsStorage,

    pub a_vault: AccountsStorage,

    pub token_b_vault: AccountsStorage,

    pub token_program: AccountsStorage,

    pub virtual_pool_metadata: AccountsStorage,

    pub b_vault: AccountsStorage,

    pub quote_vault: AccountsStorage,

    pub output_token_account: AccountsStorage,

    pub b_vault_lp_mint: AccountsStorage,

    pub second_position_nft_account: AccountsStorage,

    pub rent_receiver: AccountsStorage,

    pub event_authority: AccountsStorage,

    pub associated_token_program: AccountsStorage,

    pub escrow_token: AccountsStorage,

    pub lp_mint: AccountsStorage,

    pub locker_program: AccountsStorage,

    pub operator: AccountsStorage,

    pub sender: AccountsStorage,

    pub token_quote_account: AccountsStorage,

    pub a_vault_lp: AccountsStorage,

    pub leftover_receiver: AccountsStorage,

    pub source_token: AccountsStorage,

    pub a_token_vault: AccountsStorage,

    pub second_position: AccountsStorage,

    pub token_base_program: AccountsStorage,

    pub first_position_nft_account: AccountsStorage,

    pub token_a_vault: AccountsStorage,

    pub pool: AccountsStorage,

    pub locker_event_authority: AccountsStorage,

    pub source_tokens: AccountsStorage,

    pub token_a_account: AccountsStorage,

    pub b_vault_lp: AccountsStorage,

    pub virtual_pool: AccountsStorage,

    pub input_token_account: AccountsStorage,

    pub damm_pool_authority: AccountsStorage,

    pub admin: AccountsStorage,

    pub token_base_account: AccountsStorage,

    pub partner_metadata: AccountsStorage,

    pub virtual_pool_lp: AccountsStorage,

    pub token_b_mint: AccountsStorage,

    pub damm_config: AccountsStorage,

    pub token_2022_program: AccountsStorage,

    pub referral_token_account: AccountsStorage,

    pub config: AccountsStorage,

    pub protocol_token_b_fee: AccountsStorage,

    pub token_quote_program: AccountsStorage,

    pub system_program: AccountsStorage,

    pub vault_program: AccountsStorage,

    pub escrow: AccountsStorage,

    pub token_a_mint: AccountsStorage,

    pub program: AccountsStorage,

    pub base_vault: AccountsStorage,

    pub quote_mint: AccountsStorage,

    pub base: AccountsStorage,

    pub first_position_nft_mint: AccountsStorage,

    pub base_mint: AccountsStorage,

    pub first_position: AccountsStorage,

    pub migration_metadata: AccountsStorage,

    pub fee_claimer: AccountsStorage,

    pub a_vault_lp_mint: AccountsStorage,

    pub owner: AccountsStorage,

    pub mint_metadata: AccountsStorage,

    pub creator: AccountsStorage,

    pub escrow_vault: AccountsStorage,

    pub payer: AccountsStorage,

    pub lock_escrow: AccountsStorage,

    pub destination_token: AccountsStorage,

    pub metadata_program: AccountsStorage,

    pub protocol_token_a_fee: AccountsStorage,

    pub rent: AccountsStorage,

    pub claim_fee_operator: AccountsStorage,

    pub b_token_vault: AccountsStorage,

    pub new_creator: AccountsStorage,

    pub second_position_nft_mint: AccountsStorage,
}
