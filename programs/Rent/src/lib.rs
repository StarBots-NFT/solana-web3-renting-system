use anchor_lang::prelude::*;

declare_id!("7fHmrXy8ydjLmA629AtqzvWRkLzH91ex9NEat5nNDKH4");

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

    pub fn update_item(ctx: Context<UpdateItem>, price: u64, rental_period: u64, is_continue_list: u8) -> Result<()> {
        update_item::exec(ctx, price, rental_period, is_continue_list)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
