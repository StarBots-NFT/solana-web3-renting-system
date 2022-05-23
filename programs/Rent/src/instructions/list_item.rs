use crate::schema::*;
use anchor_lang::prelude::*;
use anchor_spl::{associated_token, token};
use crate::errors::ErrorCode;
use mpl_token_metadata::state::Metadata;

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
    #[account(
    init_if_needed,
    payer = authority,
    associated_token::mint = mint,
    associated_token::authority = treasurer
    )]
    pub nft_holder: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub nft_ata: Account<'info, token::TokenAccount>,
    /// CHECK: Just a pure account
    #[account()]
    pub nft_metadata_address: AccountInfo<'info>,
    // System Program Address
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn exec(ctx: Context<ListItem>, price: u64, rental_period: u64, is_continue_list: u8) -> Result<()> {
    let item = &mut ctx.accounts.item;
    let nft_metadata_address= &ctx.accounts.nft_metadata_address.to_account_info();
    let nft_metadata = Metadata::from_account_info(nft_metadata_address)?;
    let nft_collection = nft_metadata.collection.ok_or(ErrorCode::WrongCollectionAddress)?;
    msg!("metadata: {:?}", nft_collection.key.to_string());
    if (nft_collection.key.to_string() != "2wT1CWy9qHA8znjzugihbXhVTJ1ABkum4ZVutTi4Qz7L"
        && nft_collection.key.to_string() != "JC9QVSYBaDfiZybFqT3T8F7v2HGzaWsuUEMaJcSb4cm4")
        || nft_collection.verified != true {
        msg!("wrong collection address: {:?}", nft_collection.key);
        return err!(ErrorCode::WrongCollectionAddress);
    }




    if rental_period < 0 {
        return err!(ErrorCode::InvalidatePeriodTime);
    }
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