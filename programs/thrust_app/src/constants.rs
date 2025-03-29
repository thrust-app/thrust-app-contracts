use std::str::FromStr;

use anchor_lang::solana_program::pubkey::Pubkey;

pub const NATIVE_MINT_STR: &'static str = "So11111111111111111111111111111111111111112"; //TODO:
pub const FEE_PER_DIV: u128 = 1000;

pub const TOTAL_SUPPLY: u64 = 1_000_000_000_000_000; // 1 billion * 6 decimals
pub const GRADUATE_FEE: u64 = 5_000_000_000; // 5 SOL
pub const VIRT_SOL_RESERVE: u64 = 24_000_000_000; // 24 SOL
pub const REAL_SOL_THRESHOLD: u64 = 100_000_000_000; // 95 + 5 SOL (GRADUATE_FEE) calculated at $200 sol price
pub const RESERVE_SEED: &'static [u8] = b"reserve";
