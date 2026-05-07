use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

declare_id!("CNvr5dMyG3ytxYwQ78CvbdDQjtnzyknNDSZs7cHcXNzP");

#[program]
pub mod token_mint_pda {
    use super::*;

    pub fn initialize(ctx: Context<CreateMint>) -> Result<()> {
        msg!("Created Mint Account: {:?}",ctx.accounts.mint.key());
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
        mint::authority = mint.key(),
        mint::freeze_authority = mint.key(),
        seeds = [b"mint"],
        bump,
    )]
    pub mint: InterfaceAccount<'info,Mint>,
    pub token_program: Interface<'info,TokenInterface>,
    pub system_program: Program<'info, System>,
}
