use anchor_lang::{prelude::*, system_program};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, CloseAccount, Mint, Token, TokenAccount, Transfer},
};
use crate::{constants::RESERVE_SEED, error::ThrustAppError, MainState, PoolState};
use std::str::FromStr;

pub fn withdraw(ctx: Context<AWithdrawState>) -> Result<()> {
    let reserve_pda = &mut ctx.accounts.reserve_pda;
    let owner = ctx.accounts.owner.to_account_info();
    let main_state = &ctx.accounts.main_state;
    require!(
        main_state.initialized.eq(&true),
        ThrustAppError::Uninitialized
    );
    let pool_state = &mut ctx.accounts.pool_state;
    require!(
        pool_state.complete.eq(&true),
        ThrustAppError::BondingCurveIncomplete
    );

    require!(
        pool_state.withdrawn.eq(&false),
        ThrustAppError::AlreadyWithdrawn
    );

    let owner_base_ata = ctx.accounts.owner_base_ata.to_account_info();
    let token_program = ctx.accounts.token_program.to_account_info();

    pool_state.withdrawn = true;
    // send tokens in pool and virt
    let pool_base_transfer_cpi_account = Transfer {
        from: ctx.accounts.reserver_base_ata.to_account_info(),
        to: owner_base_ata.clone(),
        authority: pool_state.to_account_info(),
    };
    token::transfer(
        CpiContext::new_with_signer(
            token_program.clone(),
            pool_base_transfer_cpi_account,
            &[&[
                PoolState::PREFIX_SEED,
                pool_state.mint.as_ref(),
                &[ctx.bumps.pool_state],
            ]],
        ),
        pool_state.virt_base_reserves + pool_state.real_base_reserves,
    )?;

    // Get the current balance of the PDA
    let lamports_to_withdraw = **reserve_pda.to_account_info().lamports.borrow();

    system_program::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: reserve_pda.to_account_info(),
                to: ctx.accounts.owner.to_account_info(),
            },
            &[&[
                RESERVE_SEED,
                pool_state.mint.as_ref(),
                &[ctx.bumps.reserve_pda],
            ]],
        ),
        lamports_to_withdraw,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct AWithdrawState<'info> {
    #[account(mut, address = main_state.owner)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [MainState::PREFIX_SEED],
        bump,
        has_one = owner,
    )]
    pub main_state: Box<Account<'info, MainState>>,

    #[account(
        mut,
        seeds = [
            PoolState::PREFIX_SEED,
            mint.key().as_ref(), 
        ],
        bump,
    )]
    pub pool_state: Box<Account<'info, PoolState>>,

    #[account(mut,)]
    pub mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        seeds = [
            RESERVE_SEED,
            mint.key().as_ref(),
        ],
        bump,
    )]
    pub reserve_pda: SystemAccount<'info>,

    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = mint,
        associated_token::authority = pool_state,
    )]
    pub reserver_base_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = mint,
        associated_token::authority = owner,
    )]
    pub owner_base_ata: Box<Account<'info, TokenAccount>>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
