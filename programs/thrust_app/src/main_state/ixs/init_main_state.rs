use anchor_lang::prelude::*;
use crate::{
    constants::{TOTAL_SUPPLY, VIRT_SOL_RESERVE},
    error::ThrustAppError,
    MainState,
};

pub fn init_main_state(ctx: Context<AInitMainState>) -> Result<()> {
    let state = &mut ctx.accounts.main_state;
    require!(
        state.initialized.eq(&false),
        ThrustAppError::AlreadyInitialized
    );

    state.initialized = true;
    state.owner = ctx.accounts.owner.key();
    state.fee_recipient = ctx.accounts.owner.key();
    state.total_token_supply = TOTAL_SUPPLY; // default: 1 billion
    state.init_real_base_reserves = state.total_token_supply * 8 / 10; // deposit only 80% tokens
    state.init_virt_base_reserves = state.total_token_supply - state.init_real_base_reserves; // reserve 20% tokens
    state.init_virt_quote_reserves = VIRT_SOL_RESERVE; // default: 24 SOL
    state.trading_fee = 1_000; // default: 1%
    state.referral_reward_fee = 10_000; // default: 10% of platform fee = 0.1% of trading fee
    state.referral_trade_limit = 100; // default value is 100. will get reward fee until 100 trades
    state.verify_signer_pubkey = ctx.accounts.verify_signer_pubkey.key(); // signer pubkey for verify message
    Ok(())
}

#[derive(Accounts)]
pub struct AInitMainState<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub verify_signer_pubkey: AccountInfo<'info>,

    #[account(
        init,
        payer = owner,
        seeds = [MainState::PREFIX_SEED],
        bump,
        space = 8 + MainState::MAX_SIZE
    )]
    pub main_state: Account<'info, MainState>,
    pub system_program: Program<'info, System>,
}
