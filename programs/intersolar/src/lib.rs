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
        let token_mint = &ctx.accounts.token_mint;
        
        intersolar.token_mint = token_mint.key();
        intersolar.key = 0; // TODO this has to be set depending on the token mint
        intersolar.bump = bump;

        Ok(())
    }

    pub fn update(ctx: Context<Update>, name: String) -> ProgramResult {
        let intersolar = &mut ctx.accounts.intersolar;
        let token_mint = &ctx.accounts.token_mint;
        let token_account = &ctx.accounts.token_account;
        let user = &ctx.accounts.user;

        let token_account: spl_token::state::Account = assert_initialized(token_account)?;

        if token_account.owner != *user.key {
            return Err(ErrorCode::OwnerMismatch.into())
        }

        if token_account.mint != *token_mint.key {
            return Err(ErrorCode::MintMismatch.into());
        }

        if token_account.amount != 1 {
            return Err(ErrorCode::InsufficientAmount.into())
        }

        if intersolar.token_mint != *token_mint.key {
            return Err(ErrorCode::MintMismatch.into()); 
        }

        if name.len() > MAX_NAME_LENGTH {
            return Err(ErrorCode::NameTooLong.into()); 
        }

        intersolar.name = Some(string_to_fixed_len_byte_array(name, MAX_NAME_LENGTH));

        Ok(())
    }
}


#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Initialize<'info> {
    #[account(
        init, 
        seeds=[PREFIX.as_bytes(), token_mint.key().as_ref()],
        bump=bump,
        payer=user,
        space=
        32 // Pubkey
        + 1 // Key
        + 1 + MAX_NAME_LENGTH // Optional + Name
        + 1 // Bump
    )]
    pub intersolar: Account<'info, Intersolar>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub token_mint: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(
        mut,
        seeds=[PREFIX.as_bytes(), program_id.key().as_ref(), token_mint.key().as_ref()],
        bump=1
    )]
    pub intersolar: Account<'info, Intersolar>,

    pub user: Signer<'info>,

    pub token_mint: AccountInfo<'info>,

    #[account(constraint = token_account.owner == &spl_token::id())]
    pub token_account: AccountInfo<'info>,
}

#[account]
pub struct Intersolar {
    pub token_mint: Pubkey,
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

fn string_to_fixed_len_byte_array(s: String, fixed_len: usize) -> String {
    let mut array_of_zeroes = vec![];
    while array_of_zeroes.len() < fixed_len - s.len() {
        array_of_zeroes.push(0u8);
    }
    s.clone() + std::str::from_utf8(&array_of_zeroes).unwrap()
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