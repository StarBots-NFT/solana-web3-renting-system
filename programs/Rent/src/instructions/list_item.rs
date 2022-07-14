use crate::schema::*;
use anchor_lang::prelude::*;
use anchor_spl::{associated_token, token};
use crate::errors::ErrorCode;
use mpl_token_metadata::state::Metadata;
use spl_token::solana_program::program::invoke;

#[derive(Accounts)]
pub struct ListItem<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
    init,
    payer = authority,
    space = Item::SIZE,
    seeds = [b"ballot".as_ref(), &mint.key().to_bytes(), &authority.key().to_bytes()],
    bump
    )]
    pub item: Box<Account<'info, Item>>,
    #[account(seeds = [b"treasurer".as_ref(), &item.key().to_bytes()], bump)]
    /// CHECK: Just a pure account
    pub treasurer: AccountInfo<'info>,
    pub mint: Account<'info, token::Mint>,
    #[account(mut)]
    pub nft_ata: Account<'info, token::TokenAccount>,
    // System Program Address
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn exec(ctx: Context<ListItem>, price: u64, rental_period: u64, is_continue_list: u8) -> Result<()> {
    let item = &mut ctx.accounts.item;

    if is_continue_list > 2 || is_continue_list < 0 {
        return err!(ErrorCode::InvalidateIsContinute);
    }

    if rental_period < 0 {
        return err!(ErrorCode::InvalidatePeriodTime);
    }
    let owner_change_ix = spl_token::instruction::set_authority(
        &ctx.accounts.token_program.key(),
        &ctx.accounts.nft_ata.key(),
        Some(&ctx.accounts.treasurer.key()),
        spl_token::instruction::AuthorityType::AccountOwner,
        &ctx.accounts.authority.key(),
        &[&ctx.accounts.authority.key()],
    )?;
    invoke(
        &owner_change_ix,
        &[
            ctx.accounts.nft_ata.to_account_info(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
        ],
    )?;

    item.owner_address = ctx.accounts.authority.key();
    item.nft_address = ctx.accounts.mint.key();
    item.price = price;
    item.num_of_day = rental_period;
    item.is_continue_listing = is_continue_list;
    Ok(())
}