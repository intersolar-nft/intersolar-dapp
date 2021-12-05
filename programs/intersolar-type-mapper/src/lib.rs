use anchor_lang::prelude::*;

declare_id!("9UrP3ZM8H6c4wBPMbNMtbJJtpVc63JDtANvAeiks53V8");

pub const PREFIX: &str = "intersolar-type-mapper";

#[program]
pub mod intersolar_type_mapper {
    use super::*;
    // Initializes a type-mapper account
    // The PDA helps to map intersolar object types to metadata symbols
    pub fn initialize(ctx: Context<Initialize>, bump: u8, _symbol: String, r#type: u8) -> ProgramResult {
        let intersolar_type_mapper = &mut ctx.accounts.intersolar_type_mapper;

        intersolar_type_mapper.r#type = r#type;
        intersolar_type_mapper.bump = bump;
        Ok(())
    }

    // Lets the creator of the type-mapper account to change the type
    pub fn update(ctx: Context<Initialize>, _symbol: String, r#type: u8) -> ProgramResult {
        let intersolar_type_mapper = &mut ctx.accounts.intersolar_type_mapper;

        intersolar_type_mapper.r#type = r#type;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8, _symbol: String)]
pub struct Initialize<'info> {
    #[account(
        init, 
        seeds=[PREFIX.as_bytes(), _symbol.as_bytes(), update_authority.key().as_ref()],
        bump=bump,
        payer=update_authority,
        space=
        8 // discriminator
        + 1 // type
        + 1 // bump
    )]
    pub intersolar_type_mapper: Account<'info, IntersolarTypeMapper>,

    #[account(mut)]
    pub update_authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(_symbol: String)]
pub struct Update<'info> {
    #[account(
        mut,
        seeds=[PREFIX.as_bytes(), _symbol.as_bytes(), update_authority.key().as_ref()],
        bump=intersolar_type_mapper.bump
    )]
    pub intersolar_type_mapper: Account<'info, IntersolarTypeMapper>,

    pub update_authority: Signer<'info>,
}

#[account]
pub struct IntersolarTypeMapper {
    // The corresponding intersolar type for the PDA symbol
    pub r#type: u8,
    // The PDA bump of the intersolar_type_mapper account
    pub bump: u8
}