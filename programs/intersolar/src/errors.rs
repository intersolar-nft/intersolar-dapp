use anchor_lang::prelude::*;

#[error]
pub enum ErrorCode {
    #[msg("Account is not initialized!")]
    Uninitialized,
    #[msg("Mint mismatch!")]
    MintMismatch,
    #[msg("Insufficient Token Amount!")]
    InsufficientAmount,
    #[msg("Owner mismatch!")]
    OwnerMismatch,
    #[msg("Name is too long!")]
    NameTooLong,
    #[msg("Symbol not matching with metadata!")]
    SymbolMismatch,
    #[msg("The given mint is not an NFT!")]
    NoNFT,
}