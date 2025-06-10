use crate::constants::*;
use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub admin: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Metadata {
    pub job_title: Option<String>,
    pub bio: Option<String>,
    pub icon_link: Option<String>,
    pub github_link: Option<String>,
    pub social_link: Option<String>,
}

#[account]
pub struct Record {
    pub owner: Pubkey,
    pub name: String,
    pub metadata: Option<Metadata>,
}

impl Record {
    pub const MAX_SIZE: usize = 32   // owner
        + 4 + MAX_NAME_LEN           // name string
        + 1 + (                      // Option<Metadata>
            1 + 4 + MAX_TITLE_LEN +  // job_title
            1 + 4 + MAX_BIO_LEN +    // bio
            1 + 4 + MAX_LINK_LEN +   // icon_link
            1 + 4 + MAX_LINK_LEN +   // github_link
            1 + 4 + MAX_LINK_LEN     // social_link
        );
}

#[account]
pub struct UserRecord {
    pub name: String,
}
