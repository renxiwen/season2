use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};


declare_id!("5bstEFi7q1vnPQqTCrjYY7pzChPJH2829LtPZCiucex6");

#[program]
pub mod token_mint {
    use super::*;
    
    pub fn create_mint(ctx: Context<CreateMint>) -> Result<()>{
        msg!("CreateMint: mint={:?}", ctx.accounts.mint.key());
        Ok(())
    }

}

#[derive(Accounts)]
pub struct CreateMint<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        mint::decimals = 6,
        mint::authority = signer,
        mint::freeze_authority = signer,
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
