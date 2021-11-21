use {
    anchor_lang::{
        prelude::*,
        solana_program::{
            program_pack::{IsInitialized, Pack},
            entrypoint::ProgramResult
        } 
    },
};

declare_id!("Gv88Apj2oxHTWnECLF4bHnuftMXasfYHyBu3gyfs8XEe");

const PREFIX: &str = "intersolar";

const MAX_NAME_LENGTH: usize = 32;

#[program]
 mod intersolar {
    use super::*;
   
    // Initializes the intersolar object. 
    // This can be called by anyone that pays for it. 
    // It sets the type for the intersolar object by looking up the intersolar-type-mapper with the update_authority of the metadata and provided symbol.
    pub fn initialize(ctx: Context<Initialize>, bump: u8, _type_mapper_bump: u8, symbol: String) -> ProgramResult {

        let intersolar = &mut ctx.accounts.intersolar;

        let mint = &ctx.accounts.mint;
        let metadata = &ctx.accounts.metadata;
        let update_authority = &ctx.accounts.update_authority;
        let type_mapper = &ctx.accounts.type_mapper;

        // Deserialize the metadata account to check if it is correct
        let metadata = &spl_token_metadata::state::Metadata::from_account_info(metadata)?;

        // Check that the given mint account has the given update_authority
        match spl_token_metadata::utils::assert_update_authority_is_correct(metadata, update_authority) {
            Err(error) => return Err(error),
            _ => ()
        }

        // Check that the given mint belongs to the given metadata
        if metadata.mint != mint.key() {
            return Err(ErrorCode::MintMismatch.into())
        }

        // Check that the given symbol matches the metadata symbol
        if symbol != metadata.data.symbol {
            return Err(ErrorCode::SymbolMismatch.into())
        }

        // Set the type of the intersolar account to the type in the type account
        intersolar.r#type = type_mapper.r#type;

        intersolar.mint = mint.key();
        intersolar.bump = bump;

        Ok(())
    }

    pub fn rename(ctx: Context<Rename>, name: String) -> ProgramResult {

        let intersolar = &mut ctx.accounts.intersolar;

        let mint = &ctx.accounts.mint;
        let token_account = &ctx.accounts.token_account;
        let user = &ctx.accounts.user;

        // Check that the token account is initialized
        let token_account: spl_token::state::Account = assert_initialized(token_account)?;

        // Check that the user is the owner of the token_account
        if token_account.owner != *user.key {
            return Err(ErrorCode::OwnerMismatch.into())
        }

        // check that the token_account belongs to the mint
        if token_account.mint != *mint.key {
            return Err(ErrorCode::MintMismatch.into());
        }

        // check that the user holds the nft
        if token_account.amount != 1 {
            return Err(ErrorCode::InsufficientAmount.into())
        }

        // check that the mint belongs to the intersolar account
        if intersolar.mint != *mint.key {
            return Err(ErrorCode::MintMismatch.into()); 
        }

        // Check that the new name doesnt exceed the length limit
        if name.len() > MAX_NAME_LENGTH {
            return Err(ErrorCode::NameTooLong.into()); 
        }

        // Set the new name for the intersolar object
        intersolar.name = Some(name);

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8, _type_mapper_bump: u8, symbol: String)]
pub struct Initialize<'info> {
    #[account(
        init, 
        seeds=[PREFIX.as_bytes(), mint.key().as_ref()],
        bump=bump,
        payer=user,
        space=
        8 // discriminator
        + 32 // Pubkey
        + 1 // Type
        + 1 + 4 + MAX_NAME_LENGTH // Optional + len as u32 (borsh) + Name
        + 1 // Bump
    )]
    pub intersolar: Account<'info, Intersolar>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(constraint = mint.owner == &spl_token::native_mint::id())]
    pub mint: AccountInfo<'info>,

    #[account(constraint = metadata.owner == &spl_token_metadata::id())]
    pub metadata: AccountInfo<'info>,

    pub update_authority: AccountInfo<'info>,

    #[account(
        seeds=[intersolar_type_mapper::PREFIX.as_bytes(), symbol.as_bytes(), update_authority.key().as_ref()],
        bump=_type_mapper_bump
    )]
    pub type_mapper: Account<'info, intersolar_type_mapper::IntersolarTypeMapper>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Rename<'info> {
    #[account(
        mut,
        seeds=[PREFIX.as_bytes(), mint.key().as_ref()],
        bump=intersolar.bump
    )]
    pub intersolar: Account<'info, Intersolar>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(constraint = mint.owner == &spl_token::native_mint::id())]
    pub mint: AccountInfo<'info>,

    #[account(constraint = token_account.owner == &spl_token::id())]
    pub token_account: AccountInfo<'info>,
}

#[account]
pub struct Intersolar {
    pub mint: Pubkey,
    pub r#type: u8,
    pub name: Option<String>,
    pub bump: u8
}

pub fn assert_initialized<T: Pack + IsInitialized>(
    account_info: &AccountInfo,
) -> Result<T> {
    let account: T = T::unpack_unchecked(&account_info.data.borrow())?;
    if !account.is_initialized() {
        Err(ErrorCode::Uninitialized.into())
    } else {
        Ok(account)
    }
}

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
}