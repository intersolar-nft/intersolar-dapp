use anchor_lang::prelude::*;

declare_id!("6R5e1nbRBT94tfZRNZHYCmrydqBipEAPPsYrA8QbTftT");

const PREFIX: &str = "intersolar-type-mapper";

#[program]
pub mod intersolar_type_mapper {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, bump: u8, _symbol: String, r#type: u8) -> ProgramResult {
        let intersolar_type_mapper = &mut ctx.accounts.intersolar_type_mapper;

        intersolar_type_mapper.bump = bump;
        intersolar_type_mapper.r#type = r#type;
        Ok(())
    }

    pub fn update(ctx: Context<Initialize>, r#type: u8) -> ProgramResult {
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
    pub r#type: u8,
    pub bump: u8
}
