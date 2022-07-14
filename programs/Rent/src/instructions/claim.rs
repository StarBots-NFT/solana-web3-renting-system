use crate::schema::*;
use anchor_lang::prelude::*;
use anchor_spl::{associated_token, token};
use spl_token::solana_program::program::{invoke, invoke_signed};
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub owner_address: Signer<'info>,

    pub nft_address: Box<Account<'info, token::Mint>>,

    #[account(
    mut,
    has_one = owner_address,
    has_one = nft_address,
    seeds = [b"ballot".as_ref(), &nft_address.key().to_bytes(), &owner_address.key().to_bytes()],
    bump,
    close = owner_address
    )]
    pub item: Box<Account<'info, Item>>,
    #[account(seeds = [b"treasurer", &item.key().to_bytes()], bump)]
    /// CHECK: Just a pure account
    pub treasurer: AccountInfo<'info>,
    #[account(mut)]
    pub ata_address: Account<'info, token::TokenAccount>,
    // System Program Address
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn exec(ctx: Context<Claim>) -> Result<()> {
    let item = &mut ctx.accounts.item;
    let now = Clock::get().unwrap().unix_timestamp;
    msg!("now: {:?}", now);
    msg!("start_date: {:?}", item.start_date);
    if item.num_of_day + item.start_date > now as u64 {
        return err!(ErrorCode::NotActiveItem);
    }

    msg!("ata_address: {:?}", ctx.accounts.ata_address.to_account_info());
    msg!("treasurer: {:?}", ctx.accounts.treasurer.to_account_info());

    let seeds: &[&[&[u8]]] = &[&[
        "treasurer".as_ref(),
        &item.key().to_bytes(),
        &[*ctx.bumps.get("treasurer").unwrap()],
    ]];

    msg!("seeds: {:?}", seeds);
    msg!("item: {:?}", item.key());
    msg!("treasurer: {:?}", ctx.accounts.treasurer.to_account_info());

    let owner_change_ix = spl_token::instruction::set_authority(
        &ctx.accounts.token_program.key(),
        &ctx.accounts.ata_address.key(),
        Some(&ctx.accounts.owner_address.key()),
        spl_token::instruction::AuthorityType::AccountOwner,
        &ctx.accounts.treasurer.key(),
        &[],
    )?;
    invoke_signed(
        &owner_change_ix,
        &[
            ctx.accounts.ata_address.to_account_info(),
            ctx.accounts.treasurer.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
        ],
        seeds
    )?;

    // let set_authority_ctx = CpiContext::new_with_signer(
    //     ctx.accounts.token_program.to_account_info(),
    //     token::SetAuthority {
    //         current_authority: ctx.accounts.treasurer.to_account_info(),
    //         account_or_mint: ctx.accounts.owner_address.to_account_info()
    //     },
    //     seeds,
    // );
    //
    // token::set_authority(
    //     set_authority_ctx,
    //     spl_token::instruction::AuthorityType,
    //     Some(ctx.accounts.owner_address.key())
    // )?;

    msg!("done");
    Ok(())
}