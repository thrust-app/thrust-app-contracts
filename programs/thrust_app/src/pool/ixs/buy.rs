use anchor_lang::{prelude::*, system_program};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};
use crate::{
    constants::{FEE_PER_DIV, GRADUATE_FEE, REAL_SOL_THRESHOLD, RESERVE_SEED},
    error::ThrustAppError,
    main_state,
    utils::calculate_trading_fee,
    CompleteEvent, MainState, PoolState, TradeEvent, UserState, WaitingRoomState,
};

pub fn buy(ctx: Context<ABuy>, amount: u64) -> Result<()> {
    let main_state = &mut ctx.accounts.main_state;
    let pool_state = &mut ctx.accounts.pool_state;
    let reserve_pda = &mut ctx.accounts.reserve_pda;
    let user_state = &mut ctx.accounts.user_state;
    let current_timestamp = Clock::get()?.unix_timestamp;

    require!(
        main_state.initialized.eq(&true),
        ThrustAppError::Uninitialized
    );
    require!(
        current_timestamp as u64 >= pool_state.start_trade_timestamp,
        ThrustAppError::TradeStartTimeNotReached
    );
    require!(
        pool_state.complete.eq(&false),
        ThrustAppError::BondingCurveComplete
    );

    // Check Waiting Room state
    match &mut pool_state.waiting_room_state {
        WaitingRoomState::Disabled => {
            // No restrictions, proceed with normal buy
        }
        WaitingRoomState::Enabled {
            closed,
            wallet_limit_percent,
            total_buy_volume,
            participants: _,
            min_trades: _,
            max_participants: _,
            closure_condition: _,
        } => {
            // verify user qualification for waiting room
        }
    }

    let mut fee = calculate_trading_fee(main_state.trading_fee, amount);
    let mut input_amount = amount - fee;
    if (input_amount + pool_state.real_quote_reserves > REAL_SOL_THRESHOLD) {
        input_amount = REAL_SOL_THRESHOLD - pool_state.real_quote_reserves;
        fee = calculate_trading_fee(main_state.trading_fee, input_amount);
    }
    let output_amount = pool_state.compute_receivable_amount_on_buy(input_amount);
    let mut referral_reward = 0;

    const STALENESS_THRESHOLD: u64 = 60; // staleness threshold in seconds
    let sol_price = main_state.sol_price;

    let trading_volume_usd = input_amount * sol_price / 1_000_000_000;
    user_state.trade_count += 1;
    user_state.trading_volume_sol += input_amount;
    user_state.trading_volume_usd += trading_volume_usd;

    msg!("Trading volume in USD: {}", trading_volume_usd);

    // Store referrer to user state, only 1 time store.
    if user_state.referrer == Pubkey::default() && ctx.accounts.referrer.key() != Pubkey::default()
    {
        user_state.referrer = ctx.accounts.referrer.key();
    }

    if user_state.referrer != Pubkey::default()
        && user_state.referrer == ctx.accounts.referrer.key()
        && user_state.refer_trade_num <= main_state.referral_trade_limit
    {
        referral_reward = calculate_trading_fee(main_state.referral_reward_fee, fee);

        let referral_transfer_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.buyer.to_account_info(),
                to: ctx.accounts.referrer.to_account_info(),
            },
        );
        system_program::transfer(referral_transfer_ctx, referral_reward)?;
        user_state.refer_trade_num += 1;
    }

    // Transfer Fee in SOL from buyer to fee address
    let fee_transfer_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: ctx.accounts.buyer.to_account_info(),
            to: ctx.accounts.fee_recipient.to_account_info(),
        },
    );
    system_program::transfer(fee_transfer_ctx, fee - referral_reward)?;

    // Transfer SOL from buyer to Pool
    let input_amount_transfer_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: ctx.accounts.buyer.to_account_info(),
            to: reserve_pda.to_account_info(),
        },
    );
    system_program::transfer(input_amount_transfer_ctx, input_amount)?;

    // Transfer Tokens to Buyer(User) from reserve ata(Pool)
    let token_transfer_cpi_account = Transfer {
        from: ctx.accounts.reserver_base_ata.to_account_info(),
        to: ctx.accounts.buyer_base_ata.to_account_info(),
        authority: pool_state.to_account_info(),
    };

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token_transfer_cpi_account,
            &[&[
                PoolState::PREFIX_SEED,
                pool_state.mint.as_ref(),
                &[ctx.bumps.pool_state],
            ]],
        ),
        output_amount,
    )?;

    emit!(TradeEvent {
        user: ctx.accounts.buyer.to_account_info().key(),
        mint: pool_state.mint,
        token_amount: output_amount,
        sol_amount: amount,
        base_reserves: pool_state.real_base_reserves + pool_state.virt_base_reserves,
        quote_reserves: pool_state.virt_quote_reserves + pool_state.real_quote_reserves,
        is_buy: true,
        timestamp: Clock::get()?.unix_timestamp,
    });

    if (pool_state.real_quote_reserves >= REAL_SOL_THRESHOLD) {
        pool_state.complete = true;

        let pool_signer_seeds: &[&[u8]] = &[
            RESERVE_SEED,
            pool_state.mint.as_ref(),
            &[ctx.bumps.reserve_pda],
        ];
        let pool_signer: &[&[&[u8]]] = &[pool_signer_seeds];

        // Transfer 5 SOL from pool to fee
        let graduate_solfee_transfer_ctx = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: reserve_pda.to_account_info(),
                to: ctx.accounts.fee_recipient.to_account_info(),
            },
            pool_signer,
        );
        system_program::transfer(graduate_solfee_transfer_ctx, GRADUATE_FEE)?;

        emit!(CompleteEvent {
            user: ctx.accounts.buyer.to_account_info().key(),
            mint: pool_state.mint,
            timestamp: Clock::get()?.unix_timestamp,
        });
    }

    Ok(())
}

#[derive(Accounts)]
pub struct ABuy<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(
        mut,
        seeds = [MainState::PREFIX_SEED],
        bump,
    )]
    pub main_state: Box<Account<'info, MainState>>,

    #[account(mut, address = main_state.fee_recipient,)]
    pub fee_recipient: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = buyer,
        space = 8 + UserState::MAX_SIZE,
        seeds =[
            UserState::PREFIX_SEED,
            buyer.key().as_ref(),
        ],
        bump,
    )]
    pub user_state: Box<Account<'info, UserState>>,

    pub referrer: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [
            PoolState::PREFIX_SEED,
            mint.key().as_ref()
        ],
        bump,
    )]
    pub pool_state: Box<Account<'info, PoolState>>,

    #[account(address = pool_state.mint)]
    pub mint: Box<Account<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = mint,
        associated_token::authority = buyer,
    )]
    pub buyer_base_ata: Box<Account<'info, TokenAccount>>,

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
        mut,
        associated_token::mint = mint,
        associated_token::authority = pool_state,
    )]
    pub reserver_base_ata: Box<Account<'info, TokenAccount>>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
