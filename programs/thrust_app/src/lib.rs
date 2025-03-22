#![allow(unused)]

use anchor_lang::prelude::*;

pub mod main_state;
pub mod pool;
pub mod user;

pub mod constants;
pub mod error;
pub mod utils;

use main_state::*;
use pool::*;
use user::*;

declare_id!("2mXgSGzgmd4rdDXfgUm4nbJPa4fUrz9jJEuXfgpUT83B");

#[program]
pub mod thrust_app {
    use super::*;

    pub fn init_main_state(ctx: Context<AInitMainState>) -> Result<()> {
        main_state::init_main_state(ctx)
    }

    pub fn update_main_state(
        ctx: Context<AUpdateMainState>,
        input: UpdateMainStateInput,
    ) -> Result<()> {
        main_state::update_main_state(ctx, input)
    }

    pub fn update_sol_price(ctx: Context<AUpdateMainState>, price: u64) -> Result<()> {
        main_state::update_sol_price(ctx, price)
    }

    pub fn create_pool(ctx: Context<ACreatePool>, input: CreatePoolInput) -> Result<()> {
        pool::create_pool(ctx, input)
    }

    pub fn buy(ctx: Context<ABuy>, input: BuyInput) -> Result<()> {
        pool::buy(ctx, input)
    }

    pub fn sell(ctx: Context<ASell>, input: SellInput) -> Result<()> {
        pool::sell(ctx, input)
    }

    pub fn withdraw(ctx: Context<AWithdrawState>) -> Result<()> {
        pool::withdraw(ctx)
    }

}
