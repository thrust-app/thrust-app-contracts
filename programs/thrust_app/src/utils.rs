use anchor_lang::{prelude::*, solana_program::program::invoke};
use anchor_spl::token::{self, CloseAccount, SyncNative, TokenAccount};
use crate::{
    constants::{FEE_PER_DIV, NATIVE_MINT_STR},
    error::ThrustAppError,
    TaxType, UserState,
};

pub fn check_balance_on_pool_creator(ata: &TokenAccount, require_amount: u64) -> bool {
    if (ata.mint.to_string() == NATIVE_MINT_STR) {
        return true;
    }
    if ata.amount < require_amount {
        return false;
    }
    true
}

pub fn calculate_trading_fee(fee: u64, amount: u64) -> u64 {
    (amount as u128)
        .checked_mul(fee.into())
        .unwrap()
        .checked_div(FEE_PER_DIV)
        .unwrap()
        .checked_div(100)
        .unwrap() as u64
}

pub fn calculate_tax_rate(
    tax_type: &TaxType,
    user_state: &UserState,
    total_supply: u64,
    sell_amount: u64,
    current_timestamp: u64,
    seller_balance: u64,
    main_trading_fee_rate: u64,
    last_received_time: u64,
) -> u64 {
    match tax_type {
        TaxType::HigherSellTax {
            threshold_percentage,
            higher_tax_rate,
            standard_tax_rate,
            ..
        } => {
            let threshold = (total_supply as u128)
                .checked_mul(*threshold_percentage as u128)
                .unwrap()
                .checked_div(FEE_PER_DIV)
                .unwrap()
                .checked_div(100)
                .unwrap() as u64;

            if seller_balance >= threshold {
                *higher_tax_rate
            } else {
                *standard_tax_rate
            }
        }
        TaxType::DecayTax {
            initial_tax_rate,
            reduction_tiers,
            min_tax_rate,
            ..
        } => {
            let holding_time_days = current_timestamp
                .checked_sub(last_received_time)
                .unwrap()
                .checked_div(86400)
                .unwrap() as u64;

            let applicable_rate = reduction_tiers
                .iter()
                .flatten() // Skip None values
                .filter(|tier| tier.days_held <= holding_time_days)
                .map(|tier| tier.tax_rate)
                .max()
                .unwrap_or(*min_tax_rate);

            applicable_rate.clamp(*min_tax_rate, *initial_tax_rate)
        }
        TaxType::FixedTax { rate, .. } => *rate,
        _ => main_trading_fee_rate,
    }
}

pub fn close_token_account<'a>(
    owner: AccountInfo<'a>,
    ata: AccountInfo<'a>,
    token_program: AccountInfo<'a>,
) -> Result<()> {
    let cpi_accounts = CloseAccount {
        account: ata,
        authority: owner.clone(),
        destination: owner,
    };
    token::close_account(CpiContext::new(token_program, cpi_accounts))?;
    Ok(())
}

pub fn sync_native_amount<'a>(
    owner: AccountInfo<'a>,
    ata: &Account<'a, TokenAccount>,
    require_amount: u64,
    system_program: AccountInfo<'a>,
    token_program: AccountInfo<'a>,
) -> Result<()> {
    let ata_balance = ata.amount;
    let mut sync_amount = 0;
    if require_amount > ata_balance {
        sync_amount = require_amount - ata_balance
    }
    if sync_amount != 0 {
        if (owner.lamports.borrow().clone() < require_amount) {
            return Err(ThrustAppError::InsufficientFund.into());
        }
        let sol_transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
            owner.key,
            &ata.key(),
            sync_amount,
        );
        invoke(
            &sol_transfer_ix,
            &[
                owner.to_account_info(),
                ata.to_account_info(),
                system_program,
            ],
        )?;
        let sync_accounts = SyncNative {
            account: ata.to_account_info(),
        };
        token::sync_native(CpiContext::new(token_program, sync_accounts))?;
    }
    Ok(())
}
