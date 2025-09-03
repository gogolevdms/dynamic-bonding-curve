use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

/// File containing all custom types which can be used
/// in transactions and instructions or invariant checks.
///
/// You can define your own custom types here.

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct BaseFeeConfig {
    pub cliff_fee_numerator: u64,

    pub second_factor: u64,

    pub third_factor: u64,

    pub first_factor: u16,

    pub base_fee_mode: u8,

    pub padding_0: [u8; 5],
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct BaseFeeParameters {
    pub cliff_fee_numerator: u64,

    pub first_factor: u16,

    pub second_factor: u64,

    pub third_factor: u64,

    pub base_fee_mode: u8,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct ClaimFeeOperator {
    pub operator: TridentPubkey,

    //pub _padding: [u8; 128], - ошибка компиляции не может по дефолту заполнить массив 0, т.к. по умолчанию реализовано только для массивов из 32 элементов
    pub _padding: [u8; 128],
}

impl Default for ClaimFeeOperator {
    fn default() -> Self {
        Self {
            operator: TridentPubkey::default(),
            // _padding: array::from_fn(|_| 0), - сложная реализация с заполнением через лямбду функцию
            _padding: [0; 128],
        }
    }
}


#[derive(Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct Config {
    pub pool_fees: PoolFees,

    pub activation_duration: u64,

    pub vault_config_key: TridentPubkey,

    pub pool_creator_authority: TridentPubkey,

    pub activation_type: u8,

    pub partner_fee_numerator: u64,

    // pub padding: [u8; 219], - ошибка компиляции не может по дефолту заполнить массив 0, т.к. по умолчанию реализовано только для массивов из 32 элементов
    pub _padding: [u8; 219],
}

impl Default for Config {
    fn default() -> Self {
        Self {
            pool_fees: PoolFees::default(),           // Используем Default для PoolFees
            activation_duration: 0,                   // u64 по умолчанию = 0
            vault_config_key: TridentPubkey::default(), // Default для публичного ключа
            pool_creator_authority: TridentPubkey::default(), // Default для публичного ключа
            activation_type: 0,                       // u8 по умолчанию = 0
            partner_fee_numerator: 0,                 // u64 по умолчанию = 0
            _padding: [0; 219],                       // Массив из 219 нулевых байт не может быть заполнен с помощью типовых макросов Rust
        }
    }
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct ConfigParameters {
    pub pool_fees: PoolFeeParameters,

    pub collect_fee_mode: u8,

    pub migration_option: u8,

    pub activation_type: u8,

    pub token_type: u8,

    pub token_decimal: u8,

    pub partner_lp_percentage: u8,

    pub partner_locked_lp_percentage: u8,

    pub creator_lp_percentage: u8,

    pub creator_locked_lp_percentage: u8,

    pub migration_quote_threshold: u64,

    pub sqrt_start_price: u128,

    pub locked_vesting: LockedVestingParams,

    pub migration_fee_option: u8,

    pub token_supply: Option<TokenSupplyParams>,

    pub creator_trading_fee_percentage: u8,

    pub token_update_authority: u8,

    pub migration_fee: MigrationFee,

    pub migrated_pool_fee: MigratedPoolFee,

    pub padding: [u64; 7],

    pub curve: Vec<LiquidityDistributionParameters>,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct CreatePartnerMetadataParameters {
    // pub padding: [u8; 96], - ошибка компиляции не может по дефолту заполнить массив 0, т.к. по умолчанию реализовано только для массивов из 32 элементов
    pub _padding: [u8; 96],

    pub name: String,

    pub website: String,

    pub logo: String,
}

impl Default for CreatePartnerMetadataParameters {
    fn default() -> Self {
        Self {
            _padding: [0; 96],        // Инициализируем массив нулями
            name: String::new(),      // Пустая строка по умолчанию
            website: String::new(),   // Пустая строка по умолчанию
            logo: String::new(),      // Пустая строка по умолчанию
        }
    }
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct CreateVirtualPoolMetadataParameters {
    // pub padding: [u8; 96], - ошибка компиляции не может по дефолту заполнить массив 0, т.к. по умолчанию реализовано только для массивов из 32 элементов
    pub _padding: [u8; 96],

    pub name: String,

    pub website: String,

    pub logo: String,
}

impl Default for CreateVirtualPoolMetadataParameters {
    fn default() -> Self {
        Self {
            _padding: [0; 96],        // Инициализируем массив нулями
            name: String::new(),      // Пустая строка по умолчанию
            website: String::new(),   // Пустая строка по умолчанию
            logo: String::new(),      // Пустая строка по умолчанию
        }
    }
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct DynamicFeeConfig {
    pub initialized: u8,

    pub padding: [u8; 7],

    pub max_volatility_accumulator: u32,

    pub variable_fee_control: u32,

    pub bin_step: u16,

    pub filter_period: u16,

    pub decay_period: u16,

    pub reduction_factor: u16,

    pub padding2: [u8; 8],

    pub bin_step_u128: u128,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct DynamicFeeParameters {
    pub bin_step: u16,

    pub bin_step_u128: u128,

    pub filter_period: u16,

    pub decay_period: u16,

    pub reduction_factor: u16,

    pub max_volatility_accumulator: u32,

    pub variable_fee_control: u32,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct EvtClaimCreatorTradingFee {
    pub pool: TridentPubkey,

    pub token_base_amount: u64,

    pub token_quote_amount: u64,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct EvtClaimProtocolFee {
    pub pool: TridentPubkey,

    pub token_base_amount: u64,

    pub token_quote_amount: u64,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct EvtClaimTradingFee {
    pub pool: TridentPubkey,

    pub token_base_amount: u64,

    pub token_quote_amount: u64,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct EvtCloseClaimFeeOperator {
    pub claim_fee_operator: TridentPubkey,

    pub operator: TridentPubkey,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct EvtCreateClaimFeeOperator {
    pub operator: TridentPubkey,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct EvtCreateConfig {
    pub config: TridentPubkey,

    pub quote_mint: TridentPubkey,

    pub fee_claimer: TridentPubkey,

    pub owner: TridentPubkey,

    pub pool_fees: PoolFeeParameters,

    pub collect_fee_mode: u8,

    pub migration_option: u8,

    pub activation_type: u8,

    pub token_decimal: u8,

    pub token_type: u8,

    pub partner_locked_lp_percentage: u8,

    pub partner_lp_percentage: u8,

    pub creator_locked_lp_percentage: u8,

    pub creator_lp_percentage: u8,

    pub swap_base_amount: u64,

    pub migration_quote_threshold: u64,

    pub migration_base_amount: u64,

    pub sqrt_start_price: u128,

    pub locked_vesting: LockedVestingParams,

    pub migration_fee_option: u8,

    pub fixed_token_supply_flag: u8,

    pub pre_migration_token_supply: u64,

    pub post_migration_token_supply: u64,

    pub curve: Vec<LiquidityDistributionParameters>,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct EvtCreateConfigV2 {
    pub config: TridentPubkey,

    pub quote_mint: TridentPubkey,

    pub fee_claimer: TridentPubkey,

    pub leftover_receiver: TridentPubkey,

    pub config_parameters: ConfigParameters,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct EvtCreateDammV2MigrationMetadata {
    pub virtual_pool: TridentPubkey,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct EvtCreateMeteoraMigrationMetadata {
    pub virtual_pool: TridentPubkey,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct EvtCreatorWithdrawSurplus {
    pub pool: TridentPubkey,

    pub surplus_amount: u64,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct EvtCurveComplete {
    pub pool: TridentPubkey,

    pub config: TridentPubkey,

    pub base_reserve: u64,

    pub quote_reserve: u64,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct EvtInitializePool {
    pub pool: TridentPubkey,

    pub config: TridentPubkey,

    pub creator: TridentPubkey,

    pub base_mint: TridentPubkey,

    pub pool_type: u8,

    pub activation_point: u64,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct EvtPartnerMetadata {
    pub partner_metadata: TridentPubkey,

    pub fee_claimer: TridentPubkey,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct EvtPartnerWithdrawMigrationFee {
    pub pool: TridentPubkey,

    pub fee: u64,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct EvtPartnerWithdrawSurplus {
    pub pool: TridentPubkey,

    pub surplus_amount: u64,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct EvtProtocolWithdrawSurplus {
    pub pool: TridentPubkey,

    pub surplus_amount: u64,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct EvtSwap {
    pub pool: TridentPubkey,

    pub config: TridentPubkey,

    pub trade_direction: u8,

    pub has_referral: bool,

    pub params: SwapParameters,

    pub swap_result: SwapResult,

    pub amount_in: u64,

    pub current_timestamp: u64,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct EvtSwap2 {
    pub pool: TridentPubkey,

    pub config: TridentPubkey,

    pub trade_direction: u8,

    pub has_referral: bool,

    pub swap_parameters: SwapParameters2,

    pub swap_result: SwapResult2,

    pub quote_reserve_amount: u64,

    pub migration_threshold: u64,

    pub current_timestamp: u64,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct EvtUpdatePoolCreator {
    pub pool: TridentPubkey,

    pub creator: TridentPubkey,

    pub new_creator: TridentPubkey,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct EvtVirtualPoolMetadata {
    pub virtual_pool_metadata: TridentPubkey,

    pub virtual_pool: TridentPubkey,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct EvtWithdrawLeftover {
    pub pool: TridentPubkey,

    pub leftover_receiver: TridentPubkey,

    pub leftover_amount: u64,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct EvtWithdrawMigrationFee {
    pub pool: TridentPubkey,

    pub fee: u64,

    pub flag: u8,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct InitializePoolParameters {
    pub name: String,

    pub symbol: String,

    pub uri: String,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct LiquidityDistributionConfig {
    pub sqrt_price: u128,

    pub liquidity: u128,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct LiquidityDistributionParameters {
    pub sqrt_price: u128,

    pub liquidity: u128,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct LockEscrow {
    pub pool: TridentPubkey,

    pub owner: TridentPubkey,

    pub escrow_vault: TridentPubkey,

    pub bump: u8,

    pub total_locked_amount: u64,

    pub lp_per_token: u128,

    pub unclaimed_fee_pending: u64,

    pub a_fee: u64,

    pub b_fee: u64,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct LockedVestingConfig {
    pub amount_per_period: u64,

    pub cliff_duration_from_migration_time: u64,

    pub frequency: u64,

    pub number_of_period: u64,

    pub cliff_unlock_amount: u64,

    pub _padding: u64,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct LockedVestingParams {
    pub amount_per_period: u64,

    pub cliff_duration_from_migration_time: u64,

    pub frequency: u64,

    pub number_of_period: u64,

    pub cliff_unlock_amount: u64,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct MeteoraDammMigrationMetadata {
    pub virtual_pool: TridentPubkey,

    pub padding_0: [u8; 32],

    pub partner: TridentPubkey,

    pub lp_mint: TridentPubkey,

    pub partner_locked_lp: u64,

    pub partner_lp: u64,

    pub creator_locked_lp: u64,

    pub creator_lp: u64,

    pub _padding_0: u8,

    pub creator_locked_status: u8,

    pub partner_locked_status: u8,

    pub creator_claim_status: u8,

    pub partner_claim_status: u8,

    //pub _padding: [u8; 107], - ошибка компиляции не может по дефолту заполнить массив 0, т.к. по умолчанию реализовано только для массивов из 32 элементов
    pub _padding: [u8; 107],
}

impl Default for MeteoraDammMigrationMetadata {
    fn default() -> Self {
        Self {
            virtual_pool: TridentPubkey::default(),
            padding_0: [0; 32],
            partner: TridentPubkey::default(),
            lp_mint: TridentPubkey::default(),
            partner_locked_lp: 0,
            partner_lp: 0,
            creator_locked_lp: 0,
            creator_lp: 0,
            _padding_0: 0,
            creator_locked_status: 0,
            partner_locked_status: 0,
            creator_claim_status: 0,
            partner_claim_status: 0,
            _padding: [0; 107], // Ручная инициализация массива
        }
    }
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct MeteoraDammV2Metadata {
    pub virtual_pool: TridentPubkey,

    pub padding_0: [u8; 32],

    pub partner: TridentPubkey,

    // pub _padding: [u8; 126], - ошибка компиляции не может по дефолту заполнить массив 0, т.к. по умолчанию реализовано только для массивов из 32 элементов
    pub _padding: [u8; 126],
}

impl Default for MeteoraDammV2Metadata {
    fn default() -> Self {
        Self {
            virtual_pool: TridentPubkey::default(),
            padding_0: [0; 32],
            partner: TridentPubkey::default(), 
            _padding: [0; 126], // Ручная инициализация большого массива
        }
    }
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct MigratedPoolFee {
    pub collect_fee_mode: u8,

    pub dynamic_fee: u8,

    pub pool_fee_bps: u16,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct MigrationFee {
    pub fee_percentage: u8,

    pub creator_fee_percentage: u8,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct PartnerMetadata {
    pub fee_claimer: TridentPubkey,

    pub padding: [u128; 6],

    pub name: String,

    pub website: String,

    pub logo: String,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct PoolConfig {
    pub quote_mint: TridentPubkey,

    pub fee_claimer: TridentPubkey,

    pub leftover_receiver: TridentPubkey,

    pub pool_fees: PoolFeesConfig,

    pub collect_fee_mode: u8,

    pub migration_option: u8,

    pub activation_type: u8,

    pub token_decimal: u8,

    pub version: u8,

    pub token_type: u8,

    pub quote_token_flag: u8,

    pub partner_locked_lp_percentage: u8,

    pub partner_lp_percentage: u8,

    pub creator_locked_lp_percentage: u8,

    pub creator_lp_percentage: u8,

    pub migration_fee_option: u8,

    pub fixed_token_supply_flag: u8,

    pub creator_trading_fee_percentage: u8,

    pub token_update_authority: u8,

    pub migration_fee_percentage: u8,

    pub creator_migration_fee_percentage: u8,

    pub _padding_0: [u8; 7],

    pub swap_base_amount: u64,

    pub migration_quote_threshold: u64,

    pub migration_base_threshold: u64,

    pub migration_sqrt_price: u128,

    pub locked_vesting_config: LockedVestingConfig,

    pub pre_migration_token_supply: u64,

    pub post_migration_token_supply: u64,

    pub migrated_collect_fee_mode: u8,

    pub migrated_dynamic_fee: u8,

    pub migrated_pool_fee_bps: u16,

    pub _padding_1: [u8; 12],

    pub _padding_2: u128,

    pub sqrt_start_price: u128,

    pub curve: [LiquidityDistributionConfig; 20],
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct PoolFeeParameters {
    pub base_fee: BaseFeeParameters,

    pub dynamic_fee: Option<DynamicFeeParameters>,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct PoolFees {
    pub trade_fee_numerator: u64,

    pub trade_fee_denominator: u64,

    pub protocol_trade_fee_numerator: u64,

    pub protocol_trade_fee_denominator: u64,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct PoolFeesConfig {
    pub base_fee: BaseFeeConfig,

    pub dynamic_fee: DynamicFeeConfig,

    pub padding_0: [u64; 5],

    pub padding_1: [u8; 6],

    pub protocol_fee_percent: u8,

    pub referral_fee_percent: u8,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct PoolMetrics {
    pub total_protocol_base_fee: u64,

    pub total_protocol_quote_fee: u64,

    pub total_trading_base_fee: u64,

    pub total_trading_quote_fee: u64,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct SwapParameters {
    pub amount_in: u64,

    pub minimum_amount_out: u64,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct SwapParameters2 {
    pub amount_0: u64,

    pub amount_1: u64,

    pub swap_mode: u8,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct SwapResult {
    pub actual_input_amount: u64,

    pub output_amount: u64,

    pub next_sqrt_price: u128,

    pub trading_fee: u64,

    pub protocol_fee: u64,

    pub referral_fee: u64,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct SwapResult2 {
    pub included_fee_input_amount: u64,

    pub excluded_fee_input_amount: u64,

    pub amount_left: u64,

    pub output_amount: u64,

    pub next_sqrt_price: u128,

    pub trading_fee: u64,

    pub protocol_fee: u64,

    pub referral_fee: u64,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct TokenSupplyParams {
    pub pre_migration_token_supply: u64,

    pub post_migration_token_supply: u64,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct VirtualPool {
    pub volatility_tracker: VolatilityTracker,

    pub config: TridentPubkey,

    pub creator: TridentPubkey,

    pub base_mint: TridentPubkey,

    pub base_vault: TridentPubkey,

    pub quote_vault: TridentPubkey,

    pub base_reserve: u64,

    pub quote_reserve: u64,

    pub protocol_base_fee: u64,

    pub protocol_quote_fee: u64,

    pub partner_base_fee: u64,

    pub partner_quote_fee: u64,

    pub sqrt_price: u128,

    pub activation_point: u64,

    pub pool_type: u8,

    pub is_migrated: u8,

    pub is_partner_withdraw_surplus: u8,

    pub is_protocol_withdraw_surplus: u8,

    pub migration_progress: u8,

    pub is_withdraw_leftover: u8,

    pub is_creator_withdraw_surplus: u8,

    pub migration_fee_withdraw_status: u8,

    pub metrics: PoolMetrics,

    pub finish_curve_timestamp: u64,

    pub creator_base_fee: u64,

    pub creator_quote_fee: u64,

    pub _padding_1: [u64; 7],
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct VirtualPoolMetadata {
    pub virtual_pool: TridentPubkey,

    pub padding: [u128; 6],

    pub name: String,

    pub website: String,

    pub logo: String,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct VolatilityTracker {
    pub last_update_timestamp: u64,

    pub padding: [u8; 8],

    pub sqrt_price_reference: u128,

    pub volatility_accumulator: u128,

    pub volatility_reference: u128,
}
