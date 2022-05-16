use anchor_lang::prelude::*;

declare_id!("2QofFxHF1vPYKp6hUhi83iZpwLfXSKZsb1QKZwZoDSR8");

pub mod schema;
pub use schema::*;

pub mod instructions;
pub use instructions::*;

pub mod errors;


#[program]
pub mod rent {
    use super::*;
    pub fn list_item(ctx: Context<ListItem>, price: u64, rental_period: u64, is_continue_list: u8)
        -> Result<()> {
        list_item::exec(ctx,  price, rental_period, is_continue_list)
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        claim::exec(ctx)
    }

    pub fn rent(ctx: Context<RentItem>) -> Result<()> {
        rent_item::exec(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
