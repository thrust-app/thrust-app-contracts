use anchor_lang::prelude::*;

#[account]
pub struct UserState {
    pub user: Pubkey,
    pub trading_volume_sol:u64,
    pub trading_volume_usd:u64,
    pub referrer: Pubkey,
    pub refer_trade_num: u64,
}

impl UserState {
    pub const MAX_SIZE: usize = std::mem::size_of::<Self>();
    pub const PREFIX_SEED: &'static [u8] = b"user";
}
