use anchor_lang::prelude::*;

#[error_code]
pub enum NameRegistryError {
    #[msg("The provided name is too long.")]
    NameTooLong,

    #[msg("User is not owner of name record.")]
    UserIsNotOwner,
}
