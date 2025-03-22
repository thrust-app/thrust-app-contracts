use anchor_lang::solana_program::{hash::hash, secp256k1_recover::secp256k1_recover};
use anchor_lang::{prelude::*, system_program};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};
use crate::{
    constants::{FEE_PER_DIV, RESERVE_SEED},
    error::ThrustAppError,
    main_state,
    utils::{calculate_tax_rate, calculate_trading_fee},
    MainState, PoolState, TradeEvent, UserState,
};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct SellInput {
    pub amount: u64,             // Amount of tokens to sell
    pub signed_message: Vec<u8>, // Signed message containing last_received_time
    pub signature: [u8; 65],     // ECDSA signature of the message
}

/// Verifies the signed message and extracts the last_received_time
fn verify_signed_message(
    signed_message: &[u8],
    signature: &[u8; 65],
    signer_pubkey: &Pubkey,
) -> Result<u64> {
    // Hash the message
    let message_hash = hash(signed_message).to_bytes();

    // Recover public key
    let recovery_id = signature[64];
    let recovered_pubkey = secp256k1_recover(&message_hash, recovery_id, &signature[..64])
        .map_err(|_| ThrustAppError::InvalidSignature)?;

    // Convert 64-byte ECDSA pubkey to 32-byte Solana address
    let hashed_pubkey = hash(&recovered_pubkey.to_bytes()).to_bytes();
    let recovered_solana_pubkey =
        Pubkey::try_from(hashed_pubkey).map_err(|_| ThrustAppError::InvalidPubkey)?;

    // Verify match
    if recovered_solana_pubkey != *signer_pubkey {
        return Err(ThrustAppError::InvalidSignature.into());
    }

    let last_received_time = u64::from_le_bytes(
        signed_message
            .get(..8)
            .ok_or(ThrustAppError::InvalidMessage)?
            .try_into()
            .unwrap(),
    );

    Ok(last_received_time)
}

pub fn sell(ctx: Context<ASell>, input: SellInput) -> Result<()> {
    let main_state = &mut ctx.accounts.main_state;
    let pool_state = &mut ctx.accounts.pool_state;
    let reserve_pda = &mut ctx.accounts.reserve_pda;
    let user_state = &mut ctx.accounts.user_state;
    let current_timestamp = Clock::get()?.unix_timestamp;

    // Verify the signed message
    let last_received_time = verify_signed_message(
        &input.signed_message,
        &input.signature,
        &main_state.verify_signer_pubkey, // Predefined public key for verification
    )?;

    require!(
        main_state.initialized.eq(&true),
        ThrustAppError::Uninitialized
    );
    require!(
        current_timestamp as u64 > pool_state.start_trade_timestamp,
        ThrustAppError::TradeStartTimeNotReached
    );
    require!(
        pool_state.complete.eq(&false),
        ThrustAppError::BondingCurveComplete
    );

    let input_amount = input.amount;
    let _output_amount = pool_state.compute_receivable_amount_on_sell(input_amount);

    let current_timestamp = Clock::get()?.unix_timestamp as u64;

    let fee_rate;
    if pool_state.is_tax_active(current_timestamp) {
        let seller_balance = ctx.accounts.seller_base_ata.amount;

        fee_rate = calculate_tax_rate(
            &pool_state.tax_type,
            &user_state,
            main_state.total_token_supply,
            _output_amount,
            current_timestamp,
            seller_balance,
            main_state.trading_fee,
            last_received_time,
        );
    } else {
        fee_rate = main_state.trading_fee;
    }

    let fee = calculate_trading_fee(fee_rate, _output_amount);
    let output_amount = _output_amount - fee;
    let mut referral_reward = 0;

    const STALENESS_THRESHOLD: u64 = 60; // staleness threshold in seconds
    let sol_price = main_state.sol_price;

    let trading_volume_usd = _output_amount * sol_price / 1_000_000_000;
    user_state.trade_count += 1;
    user_state.trading_volume_sol += _output_amount;
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
                from: ctx.accounts.seller.to_account_info(),
                to: ctx.accounts.referrer.to_account_info(),
            },
        );
        system_program::transfer(referral_transfer_ctx, referral_reward)?;
        user_state.refer_trade_num += 1;
    }

    let pool_signer_seeds: &[&[u8]] = &[
        RESERVE_SEED,
        pool_state.mint.as_ref(),
        &[ctx.bumps.reserve_pda],
    ];
    let pool_signer: &[&[&[u8]]] = &[pool_signer_seeds];

    // Sending Fee in SOL from pool to fee address
    let fee_transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: reserve_pda.to_account_info(),
            to: ctx.accounts.fee_recipient.to_account_info(),
        },
        pool_signer,
    );
    system_program::transfer(fee_transfer_ctx, fee - referral_reward)?;

    // Transfer Mint tokens from seller to pool
    let token_transfer_cpi_account = Transfer {
        from: ctx.accounts.seller_base_ata.to_account_info(),
        to: ctx.accounts.reserver_base_ata.to_account_info(),
        authority: ctx.accounts.seller.to_account_info(),
    };
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token_transfer_cpi_account,
        ),
        input_amount,
    )?;

    // Transfer SOL from pool to seller
    let sol_transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: reserve_pda.to_account_info(),
            to: ctx.accounts.seller.to_account_info(),
        },
        pool_signer,
    );
    system_program::transfer(sol_transfer_ctx, output_amount)?;

    emit!(TradeEvent {
        user: ctx.accounts.seller.to_account_info().key(),
        mint: pool_state.mint,
        token_amount: input_amount,
        sol_amount: output_amount,
        base_reserves: pool_state.real_base_reserves + pool_state.virt_base_reserves,
        quote_reserves: pool_state.virt_quote_reserves + pool_state.real_quote_reserves,
        is_buy: false,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct ASell<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(
        mut,
        seeds = [MainState::PREFIX_SEED],
        bump,
    )]
    pub main_state: Box<Account<'info, MainState>>,

    /// CHECK: This address is fee recipient address
    #[account(mut, address = main_state.fee_recipient,)]
    pub fee_recipient: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = seller,
        space = 8 + UserState::MAX_SIZE,
        seeds =[
            UserState::PREFIX_SEED,
            seller.key().as_ref(),
        ],
        bump,
    )]
    pub user_state: Box<Account<'info, UserState>>,

    /// CHECK: Ensure referrer is valid address
    pub referrer: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [
            PoolState::PREFIX_SEED,
            mint.key().as_ref(), 
        ],
        bump,
    )]
    pub pool_state: Box<Account<'info, PoolState>>,

    #[account(address = pool_state.mint)]
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
        payer = seller,
        associated_token::mint = mint,
        associated_token::authority = seller,
    )]
    pub seller_base_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = seller,
        associated_token::mint = mint,
        associated_token::authority = pool_state,
    )]
    pub reserver_base_ata: Box<Account<'info, TokenAccount>>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
