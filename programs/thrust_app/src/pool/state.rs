use anchor_lang::prelude::*;
use crate::constants::REAL_SOL_THRESHOLD;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub enum TaxDuration {
    FixedDuration(u64), // Number of days
    Lifetime,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub struct ReductionTier {
    pub days_held: u64,
    pub tax_rate: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub enum TaxType {
    Disabled,
    HigherSellTax {
        threshold_percentage: u64,
        higher_tax_rate: u64,
        standard_tax_rate: u64,
        duration: TaxDuration,
    },
    DecayTax {
        initial_tax_rate: u64,
        reduction_tiers: [Option<ReductionTier>; 4],
        min_tax_rate: u64,
        duration: TaxDuration,
    },
    FixedTax {
        rate: u64,
        duration: TaxDuration,
    },
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct WaitingRoomConfig {
    pub min_trades: u32,
    pub max_participants: u32,
    pub wallet_limit_percent: u8,
    pub closure_condition: ClosureCondition,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub enum WaitingRoomState {
    Disabled,
    Enabled {
        min_trades: u32,
        max_participants: u32,
        wallet_limit_percent: u8,
        closure_condition: ClosureCondition,
        participants: u32,
        total_buy_volume: u64,
        closed: bool,
    },
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub enum ClosureCondition {
    TimeBased(i64), // Unix timestamp
    ParticipantCount(u32),
    BuyVolume(u64),
}

#[account]
pub struct PoolState {
    pub owner: Pubkey,
    pub konst: u128,
    pub mint: Pubkey,
    pub start_trade_timestamp: u64,
    pub virt_base_reserves: u64,
    pub real_base_reserves: u64,
    pub virt_quote_reserves: u64,
    pub real_quote_reserves: u64,
    pub complete: bool,
    pub withdrawn: bool,
    pub tax_type: TaxType,
    pub tax_start_timestamp: u64,
    pub waiting_room_state: WaitingRoomState,
}

impl PoolState {
    pub const MAX_SIZE: usize = std::mem::size_of::<Self>() + 100;
    pub const PREFIX_SEED: &'static [u8] = b"pool";

    pub fn is_tax_active(&self, current_timestamp: u64) -> bool {
        match &self.tax_type {
            TaxType::Disabled => false,
            TaxType::HigherSellTax { duration, .. }
            | TaxType::DecayTax { duration, .. }
            | TaxType::FixedTax { duration, .. } => match duration {
                TaxDuration::Lifetime => true,
                TaxDuration::FixedDuration(days) => {
                    let elapsed_days = (current_timestamp - self.tax_start_timestamp) / 86400;
                    elapsed_days <= *days
                }
            },
        }
    }

    pub fn compute_receivable_amount_on_buy(&mut self, quote_amount: u64) -> u64 {
        let mut amount = quote_amount;
        if (amount + self.real_quote_reserves > REAL_SOL_THRESHOLD) {
            amount = REAL_SOL_THRESHOLD - self.real_quote_reserves;
        }
        let base_amount = calculate_output_amount(
            amount,
            self.virt_quote_reserves + self.real_quote_reserves,
            self.real_base_reserves + self.virt_base_reserves,
        );
        self.real_base_reserves -= base_amount;
        self.real_quote_reserves += amount;
        base_amount
    }

    pub fn compute_receivable_amount_on_sell(&mut self, base_amount: u64) -> u64 {
        let quote_amount = calculate_output_amount(
            base_amount,
            self.real_base_reserves + self.virt_base_reserves,
            self.virt_quote_reserves + self.real_quote_reserves,
        );
        self.real_base_reserves += base_amount;
        self.real_quote_reserves -= quote_amount;
        quote_amount
    }
}

fn calculate_output_amount(input_amount: u64, input_reserve: u64, output_reserve: u64) -> u64 {
    let output_amount = (output_reserve as u128)
        .checked_mul(input_amount as u128)
        .unwrap()
        .checked_div((input_reserve as u128) + (input_amount as u128))
        .unwrap();
    output_amount as u64
}
