use anchor_lang::prelude::*;
use anchor_spl::token_interface::{TokenAccount,Mint,self, MintTo,TokenInterface};

declare_id!("8KchvHKeHX3p284j9qnmdoHqzorcVSZj9u6egxw52vuh");

#[program]
pub mod mint_token {
    use super::*;

    pub fn mint_instroduct(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        let cpi_contest = CpiContext::new(
            ctx.accounts.token_program.key(),
            MintTo{
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.signer.to_account_info(),
            });
        token_interface::mint_to(cpi_contest, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub mint: InterfaceAccount<'info,Mint>,
    #[account(mut)]
    pub token_account: InterfaceAccount<'info,TokenAccount>,
    pub token_program: Interface<'info,TokenInterface>
}
