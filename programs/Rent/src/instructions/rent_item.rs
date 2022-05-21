use crate::schema::*;
use anchor_lang::prelude::*;
use anchor_spl::{associated_token, token};
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct RentItem<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    /// CHECK: Just a pure account
    pub owner_address: AccountInfo<'info>,
    #[account(
    mut,
    has_one = owner_address
    )]
    pub item: Box<Account<'info, Item>>,
    /// CHECK: Just a pure account
    #[account[mut]]
    pub holder: UncheckedAccount<'info>,
    // System Program Address
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn exec(ctx: Context<RentItem>) -> Result<()> {
    let item = &mut ctx.accounts.item;
    let now = Clock::get().unwrap().unix_timestamp;

    msg!("now: {:?}", now);
    msg!("num_of_day: {:?}", item.num_of_day);
    msg!("owner: {:?}", item.owner_address);
    msg!("renter: {:?}", item.rent_address);
    msg!("price: {:?}", item.price);
    if item.num_of_day + item.start_date > now as u64 && item.rent_address.to_string() != "11111111111111111111111111111111"  {
        return err!(ErrorCode::NotActiveItem);
    }

    if item.rent_address.to_string() != "11111111111111111111111111111111" && item.is_continue_listing != 1 {
        return err!(ErrorCode::NotActiveItem);
    }

    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.signer.key(),
        &ctx.accounts.owner_address.key(),
        item.price * 9 / 10,
    );
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.signer.to_account_info(),
            ctx.accounts.owner_address.to_account_info(),
        ],
    );

    let ix_transfer_to_holder = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.signer.key(),
        &ctx.accounts.holder.key(),
        item.price / 10,
    );
    anchor_lang::solana_program::program::invoke(
        &ix_transfer_to_holder,
        &[
            ctx.accounts.signer.to_account_info(),
            ctx.accounts.holder.to_account_info(),
        ],
    );

    item.rent_address = ctx.accounts.signer.key();
    item.start_date = now as u64;

    Ok(())
}