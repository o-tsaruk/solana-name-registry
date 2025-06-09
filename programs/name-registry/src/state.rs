use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Metadata {
    pub job_title: Option<String>,
    pub bio: Option<String>,
    pub icon_link: Option<String>,
    pub github_link: Option<String>,
    pub social_link: Option<String>,
}

#[account]
pub struct NameRecord {
    pub owner: Pubkey,
    pub name: String,
    pub metadata: Option<Metadata>,
}
