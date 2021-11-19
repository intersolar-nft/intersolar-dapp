use {
    anchor_lang::{
        prelude::*,
        solana_program::{
            program_pack::{IsInitialized, Pack},
        } 
    }
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

const PREFIX: &str = "intersolar";

const MAX_NAME_LENGTH: usize = 32;

#[program]
 mod intersolar {
    use super::*;
    
    pub fn initialize(ctx: Context<Initialize>, bump: u8) -> ProgramResult {
        let intersolar = &mut ctx.accounts.intersolar;
        let mint = &ctx.accounts.mint;
        
        intersolar.mint = mint.key();
        intersolar.key = 0; // TODO this has to be set depending on the token mint
        intersolar.bump = bump;

        Ok(())
    }

    pub fn rename(ctx: Context<Rename>, name: String) -> ProgramResult {
        let intersolar = &mut ctx.accounts.intersolar;
        let mint = &ctx.accounts.mint;
        let token_account = &ctx.accounts.token_account;
        let user = &ctx.accounts.user;

        let token_account: spl_token::state::Account = assert_initialized(token_account)?;

        if token_account.owner != *user.key {
            return Err(ErrorCode::OwnerMismatch.into())
        }

        if token_account.mint != *mint.key {
            return Err(ErrorCode::MintMismatch.into());
        }

        if token_account.amount != 1 {
            return Err(ErrorCode::InsufficientAmount.into())
        }

        if intersolar.mint != *mint.key {
            return Err(ErrorCode::MintMismatch.into()); 
        }

        if name.len() > MAX_NAME_LENGTH {
            return Err(ErrorCode::NameTooLong.into()); 
        }

        intersolar.name = Some(name);

        Ok(())
    }

    // TODO Update method for editing key -> use Metaplex update_authority and is_mutable flags for this
}


#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Initialize<'info> {
    #[account(
        init, 
        seeds=[PREFIX.as_bytes(), mint.key().as_ref()],
        bump=bump,
        payer=user,
        space=
        8 // discriminator
        + 32 // Pubkey
        + 1 // Key
        + 1 + 4 + MAX_NAME_LENGTH // Optional + len as u32 (borsh) + Name
        + 1 // Bump
    )]
    pub intersolar: Account<'info, Intersolar>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub mint: AccountInfo<'info>,

    pub metadata: AccountInfo<'info>,

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

    pub mint: AccountInfo<'info>,

    #[account(constraint = token_account.owner == &spl_token::id())]
    pub token_account: AccountInfo<'info>,
}

#[account]
pub struct Intersolar {
    pub mint: Pubkey,
    pub key: u8,
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
}