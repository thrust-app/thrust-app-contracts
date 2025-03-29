use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, MintTo, SetAuthority, SyncNative, Token, TokenAccount, Transfer},
};
use mpl_token_metadata::instructions::CreateMetadataAccountV3Builder;
use mpl_token_metadata::types::DataV2;
use mpl_token_metadata::ID as METADATA_PROGRAM_ID;
use solana_program::program::invoke_signed;

use crate::{
    constants::RESERVE_SEED, constants::TOTAL_SUPPLY, error::ThrustAppError, CreateEvent,
    MainState, PoolState, TaxType, UserState,
};

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug)]
pub struct CreatePoolInput {
    pub mint_name: String,
    pub mint_symbol: String,
    pub mint_uri: String,
    pub trade_start_time: u64,
    pub tax_type: TaxType,
}

pub fn create_pool(ctx: Context<ACreatePool>, input: CreatePoolInput) -> Result<()> {
    let main_state = &mut ctx.accounts.main_state;
    require!(
        main_state.initialized.eq(&true),
        ThrustAppError::Uninitialized
    );

    let mint_key = ctx.accounts.mint.key();
    let creator_key = ctx.accounts.creator.key();
    let metadata_key = ctx.accounts.metadata_account.key();
    let metadata_seeds = &[b"metadata", METADATA_PROGRAM_ID.as_ref(), mint_key.as_ref()];
    // let (metadata_key, _bump) = Pubkey::find_program_address(metadata_seeds, &METADATA_PROGRAM_ID);

    let pool_state = &mut ctx.accounts.pool_state;
    let user_state = &mut ctx.accounts.user_state;

    // Store referrer to user state, only 1 time store.
    let default_pubkey = Pubkey::default();
    if user_state.referrer == default_pubkey {
        if let Some(referrer) = &ctx.accounts.referrer {
            user_state.referrer = referrer.key();
        }
    }

    // Define token metadata (name, symbol, URI, etc.)
    let metadata = DataV2 {
        name: input.mint_name,      // Set Token Name
        symbol: input.mint_symbol,  // Set Token Symbol
        uri: input.mint_uri,        // Set Metadata URI
        seller_fee_basis_points: 0, // No royalty
        creators: None,             // No creators
        collection: None,
        uses: None,
    };

    let metadata_ix = CreateMetadataAccountV3Builder::new()
        .metadata(metadata_key)
        .mint(mint_key)
        .mint_authority(creator_key)
        .payer(creator_key)
        .update_authority(creator_key, true) // Tuple format
        .data(metadata)
        .is_mutable(false)
        .instruction();

    invoke_signed(
        &metadata_ix,
        &[
            ctx.accounts.metadata_account.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.creator.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        &[metadata_seeds],
    )?;

    // Mint tokens
    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.reserver_base_ata.to_account_info(),
        authority: ctx.accounts.creator.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::mint_to(cpi_ctx, TOTAL_SUPPLY)?; // Mint 1 token (6 decimals)

    // Revoke mint authority
    let cpi_accounts_mint = SetAuthority {
        account_or_mint: ctx.accounts.mint.to_account_info(),
        current_authority: ctx.accounts.creator.to_account_info(),
    };
    
    let cpi_ctx_mint = CpiContext::new(
        ctx.accounts.token_program.to_account_info(), 
        cpi_accounts_mint
    );
    
    token::set_authority(
        cpi_ctx_mint,
        spl_token::instruction::AuthorityType::MintTokens,
        None,
    )?;
    
    // Revoke freeze authority
    let cpi_accounts_freeze = SetAuthority {
        account_or_mint: ctx.accounts.mint.to_account_info(),
        current_authority: ctx.accounts.creator.to_account_info(),
    };
    
    let cpi_ctx_freeze = CpiContext::new(
        ctx.accounts.token_program.to_account_info(), 
        cpi_accounts_freeze
    );
    
    token::set_authority(
        cpi_ctx_freeze,
        spl_token::instruction::AuthorityType::FreezeAccount,
        None,
    )?;

    pool_state.owner = creator_key;
    pool_state.mint = ctx.accounts.mint.to_account_info().key();
    pool_state.start_trade_timestamp = input.trade_start_time;

    pool_state.real_base_reserves = main_state.init_real_base_reserves;
    pool_state.real_quote_reserves = 0;

    pool_state.virt_base_reserves = TOTAL_SUPPLY - main_state.init_real_base_reserves;
    pool_state.virt_quote_reserves = main_state.init_virt_quote_reserves;
    pool_state.konst = (pool_state.real_base_reserves as u128)
        .checked_mul((pool_state.virt_quote_reserves + pool_state.real_quote_reserves) as u128)
        .unwrap();

    let current_timestamp = Clock::get()?.unix_timestamp;

    pool_state.tax_type = input.tax_type;
    pool_state.tax_start_timestamp = current_timestamp as u64;

    emit!(CreateEvent {
        creator: pool_state.owner,
        mint: pool_state.mint,
        base_reserves: pool_state.real_base_reserves + pool_state.virt_base_reserves,
        quote_reserves: pool_state.virt_quote_reserves + pool_state.real_quote_reserves,
        timestamp: current_timestamp
    });

    Ok(())
}

#[derive(Accounts)]
pub struct ACreatePool<'info> {
    #[account(
        init,
        payer = creator,
        mint::decimals = 6,
        mint::authority = creator
    )]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub creator: Signer<'info>,

    // CHECK: Metaplex metadata account (PDA derived from mint)
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [MainState::PREFIX_SEED],
        bump,
    )]
    pub main_state: Box<Account<'info, MainState>>,

    #[account(
        init,
        payer = creator,
        seeds =[
            PoolState::PREFIX_SEED,
            mint.key().as_ref(),
        ],
        bump,
        space = 8 + PoolState::MAX_SIZE
    )]
    pub pool_state: Box<Account<'info, PoolState>>,

    #[account(
        init_if_needed,
        payer = creator,
        space = 8 + UserState::MAX_SIZE,
        seeds =[
            UserState::PREFIX_SEED,
            creator.key().as_ref(),
        ],
        bump,
    )]
    pub user_state: Box<Account<'info, UserState>>,

    pub referrer: Option<AccountInfo<'info>>,

    #[account(
        init,
        payer = creator,
        associated_token::mint = mint,
        associated_token::authority = pool_state,
    )]
    pub reserver_base_ata: Box<Account<'info, TokenAccount>>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}
