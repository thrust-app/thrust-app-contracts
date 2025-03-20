use anchor_lang::prelude::*;
use crate::constants::REAL_SOL_THRESHOLD;
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
}

impl PoolState {
    pub const MAX_SIZE: usize = std::mem::size_of::<Self>();
    pub const PREFIX_SEED: &'static [u8] = b"pool";

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
