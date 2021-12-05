pub mod errors;
pub mod utils;
use {
    crate::errors::ErrorCode,
    crate::utils::{assert_initialized},
    anchor_lang::{
        prelude::*,
        solana_program::{
            entrypoint::ProgramResult
        } 
    },
    intersolar_type_mapper::{
        PREFIX as TYPE_MAPPER_PREFIX,
        IntersolarTypeMapper,
        program::IntersolarTypeMapper as IntersolarTypeMapperProgram,
    },
};

declare_id!("FW339gZ8MfUP3evYoDs5V88NtuV116mExdNkuciQfCfR");

const PREFIX: &str = "intersolar";

const MAX_NAME_LENGTH: usize = 32;

#[program]
 mod intersolar {
    use super::*;
   
    // Initializes the intersolar object. 
    // This can be called by anyone that pays for it. 
    // It sets the type for the intersolar object by looking it up the type_mapper with the update_authority of the metadata and provided symbol.
    pub fn initialize(ctx: Context<Initialize>, bump: u8, symbol: String) -> ProgramResult {

        let intersolar = &mut ctx.accounts.intersolar;

        let mint = &ctx.accounts.mint;
        let metadata = &ctx.accounts.metadata;
        let update_authority = &ctx.accounts.update_authority;
        let type_mapper = &ctx.accounts.type_mapper;

        // Deserialize the given mint account
        let deserialized_mint: &spl_token::state::Mint = &assert_initialized(mint)?;

        // Deserialize the metadata account to check if it is correct
        let deserialized_metadata = &spl_token_metadata::state::Metadata::from_account_info(metadata)?;

        // Construct type mapper PDA
        let (type_mapper_pubkey, _) = anchor_lang::solana_program::pubkey::Pubkey::find_program_address(
            &[TYPE_MAPPER_PREFIX.as_bytes(), symbol.as_bytes(), update_authority.key().as_ref()], 
            &IntersolarTypeMapperProgram::id()
        );

        // Check that given type mapper has correct symbol and update authority
        if type_mapper.key() != type_mapper_pubkey {
            return Err(ErrorCode::TypeMapperMismatch.into())
        }

        // Check that the given mint is an NFT mint
        if deserialized_mint.decimals != 0 || deserialized_mint.supply != 1 {
            return Err(ErrorCode::NoNFT.into())
        }

        // Check that the given mint account has the given update_authority
        if deserialized_metadata.update_authority != update_authority.key() {
            return Err(ErrorCode::UpdateAuthorityMismatch.into());
        }

        // Check that the given mint belongs to the given metadata
        if deserialized_metadata.mint != mint.key() {
            return Err(ErrorCode::MintMismatch.into())
        }

        // Trim trailing null bytes
        let metadata_symbol = deserialized_metadata.data.symbol.trim_matches(char::from(0));

        // Check that the given symbol matches the metadata symbol
        if symbol != metadata_symbol {
            return Err(ErrorCode::SymbolMismatch.into())
        }

        // Set the type of the intersolar account to the type_mapper type
        intersolar.r#type = type_mapper.r#type;

        intersolar.mint = mint.key();
        intersolar.bump = bump;

        Ok(())
    }

    // Sets the name of the intersolar object. 
    // This can be called by the holder of the token from the mint. 
    // It sets the type for the intersolar object by looking it up the type_mapper with the update_authority of the metadata and provided symbol.
    pub fn rename(ctx: Context<Rename>, name: String) -> ProgramResult {

        let intersolar = &mut ctx.accounts.intersolar;

        let mint = &ctx.accounts.mint;
        let token_account = &ctx.accounts.token_account;
        let user = &ctx.accounts.user;

        // Check that the token account is initialized
        let deserialized_token_account: spl_token::state::Account = assert_initialized(token_account)?;

        // Check that the user is the owner of the token_account
        if deserialized_token_account.owner != *user.key {
            return Err(ErrorCode::OwnerMismatch.into())
        }

        // check that the token_account belongs to the mint
        if deserialized_token_account.mint != *mint.key {
            return Err(ErrorCode::MintMismatch.into());
        }

        // check that the user holds the nft
        if deserialized_token_account.amount != 1 {
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
#[instruction(bump: u8, symbol: String)]
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

    #[account(owner = spl_token::id())]
    pub mint: AccountInfo<'info>,

    #[account(owner = spl_token_metadata::id())]
    pub metadata: AccountInfo<'info>,

    pub update_authority: AccountInfo<'info>,

    #[account(owner = intersolar_type_mapper::id())]
    pub type_mapper: Account<'info, IntersolarTypeMapper>,

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

    #[account(constraint = mint.owner == &spl_token::id())]
    pub mint: AccountInfo<'info>,

    // TODO: Check owner against TOKEN_ACCOUNT program ID, not MINT program ID
    #[account(constraint = token_account.owner == &spl_token::id())]
    pub token_account: AccountInfo<'info>,
}

#[account]
pub struct Intersolar {
    // The NFT token_mint this account belongs to
    pub mint: Pubkey,
    // The type of this intersolar object (e.g. 0 for "Planet", 1 for "Ship", ...)
    pub r#type: u8,
    // The name of the intersolar object (can be changed by the NFT owner)
    pub name: Option<String>,
    // The PDA bump of the intersolar account
    pub bump: u8
}