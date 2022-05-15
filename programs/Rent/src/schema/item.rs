use anchor_lang::prelude::*;
use std::option::Option;
#[account]
pub struct Item {
    pub price: u64,
    pub start_date: u64,
    pub num_of_day: u64,
    pub is_continue_listing: u8,
    pub nft_address: Pubkey,
    pub owner_address: Pubkey,
    pub rent_address: Pubkey,
}

impl Item {
    pub const SIZE: usize = 8 + 8 + 8 + 8 + 1 + 32 + 32 + 32;
}