use crate::schema::*;
use anchor_lang::prelude::*;
use anchor_spl::{associated_token, token};

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
    pub mint: Box<Account<'info, token::Mint>>,
    #[account(
    init,
    payer = authority,
    associated_token::mint = mint,
    associated_token::authority = treasurer
    )]
    pub nft_holder: Account<'info, token::TokenAccount>,
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
    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::Transfer {
            from: ctx.accounts.nft_ata.to_account_info(),
            to: ctx.accounts.nft_holder.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        },
    );

    token::transfer(transfer_ctx, 1)?;
    item.owner_address = ctx.accounts.authority.key();
    item.nft_address = ctx.accounts.mint.key();
    item.price = price;
    item.num_of_day = rental_period;
    item.is_continue_listing = is_continue_list;
    Ok(())
}