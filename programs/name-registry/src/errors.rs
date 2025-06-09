use anchor_lang::prelude::*;

#[error_code]
pub enum NameRegistryError {
    #[msg("The provided name is too long.")]
    NameTooLong,
}
