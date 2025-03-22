use anchor_lang::prelude::*;
use crate::{error::ThrustAppError, MainState};

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone, Copy)]
pub struct UpdateMainStateInput {
    owner: Pubkey,
    fee_recipient: Pubkey,
    trading_fee: u64,
    sol_price: u64,
    referral_reward_fee: u64,
    referral_trade_limit: u64,
    total_token_supply: Option<u64>,
    init_virt_base_reserves: Option<u64>,
    init_real_base_reserves: Option<u64>,
    init_virt_quote_reserves: Option<u64>,
}

pub fn update_main_state(
    ctx: Context<AUpdateMainState>,
    input: UpdateMainStateInput,
) -> Result<()> {
    let state = &mut ctx.accounts.main_state;
    require!(state.initialized.eq(&true), ThrustAppError::Uninitialized);

    msg!("owner: {}", input.owner);
    msg!("fee_recipient: {}", input.fee_recipient);
    msg!("trading_fee: {}", input.trading_fee);
    msg!("referral_reward_fee: {}", input.referral_reward_fee);
    msg!("referral_trade_limit: {}", input.referral_trade_limit);

    state.owner = input.owner;
    state.fee_recipient = input.fee_recipient;
    state.trading_fee = input.trading_fee;
    state.referral_reward_fee = input.referral_reward_fee;
    state.referral_trade_limit = input.referral_trade_limit;
    state.total_token_supply = input.total_token_supply.unwrap_or(state.total_token_supply);
    state.init_virt_base_reserves = input
        .init_virt_base_reserves
        .unwrap_or(state.init_virt_base_reserves);
    state.init_real_base_reserves = input
        .init_real_base_reserves
        .unwrap_or(state.init_real_base_reserves);
    state.init_virt_quote_reserves = input
        .init_virt_quote_reserves
        .unwrap_or(state.init_virt_quote_reserves);
    state.verify_signer_pubkey = ctx.accounts.verify_signer_pubkey.key(); // signer pubkey for verify message
    msg!("Updated mainState");

    Ok(())
}

pub fn update_sol_price(ctx: Context<AUpdateMainState>, price: u64) -> Result<()> {
    let state = &mut ctx.accounts.main_state;
    require!(state.initialized.eq(&true), ThrustAppError::Uninitialized);
    state.sol_price = price;

    msg!("Sol Price Updated {}", price);

    Ok(())
}

#[derive(Accounts)]
pub struct AUpdateMainState<'info> {
    #[account(mut, address = main_state.owner @ ThrustAppError::Unauthorised)]
    pub owner: Signer<'info>,

    /// CHECK: signer public key for verification
    #[account(mut)]
    pub verify_signer_pubkey: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [MainState::PREFIX_SEED],
        bump,
        has_one = owner,
    )]
    pub main_state: Account<'info, MainState>,
}
