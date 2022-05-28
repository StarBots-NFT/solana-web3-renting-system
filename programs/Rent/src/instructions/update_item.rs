use crate::schema::*;
use anchor_lang::prelude::*;
use anchor_spl::{associated_token, token};
use crate::errors::ErrorCode;
use mpl_token_metadata::state::Metadata;

#[derive(Accounts)]
pub struct UpdateItem<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub mint: Account<'info, token::Mint>,
    #[account(
    mut,
    seeds = [b"ballot".as_ref(), &mint.key().to_bytes(), &authority.key().to_bytes()],
    bump
    )]
    pub item: Box<Account<'info, Item>>,
    // System Program Address
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn exec(ctx: Context<UpdateItem>, price: u64, rental_period: u64, is_continue_list: u8) -> Result<()> {
    let item = &mut ctx.accounts.item;

    if rental_period < 0 {
        return err!(ErrorCode::InvalidatePeriodTime);
    }

    if item.owner_address.to_string() != ctx.accounts.authority.key().to_string() {
        return err!(ErrorCode::WrongOwnerAddress);
    }

    if item.num_of_day + item.start_date > now as u64 {
        msg!("in rent time cant change num_of_day");
        msg!("price: {:?}", item.price);
        msg!("start_date: {:?}", item.start_date);
        msg!("is_continue_listing: {:?}", item.is_continue_listing);
        msg!("num_of_day: {:?}", item.num_of_day);
        item.price = price;
        item.is_continue_listing = is_continue_list;
        msg!("price: {:?}", item.price);
        msg!("start_date: {:?}", item.start_date);
        msg!("is_continue_listing: {:?}", item.is_continue_listing);
        msg!("num_of_day: {:?}", item.num_of_day);
    } else {
        msg!("price: {:?}", item.price);
        msg!("start_date: {:?}", item.start_date);
        msg!("is_continue_listing: {:?}", item.is_continue_listing);
        msg!("num_of_day: {:?}", item.num_of_day);
        item.price = price;
        item.num_of_day = rental_period;
        item.is_continue_listing = is_continue_list;
        msg!("price: {:?}", item.price);
        msg!("start_date: {:?}", item.start_date);
        msg!("is_continue_listing: {:?}", item.is_continue_listing);
        msg!("num_of_day: {:?}", item.num_of_day);
    }
    Ok(())
}