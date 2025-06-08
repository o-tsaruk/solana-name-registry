use anchor_lang::prelude::*;

#[account]
pub struct NameRecord {
    pub owner: Pubkey,
    pub name: String,
    pub metadata: String,
}
