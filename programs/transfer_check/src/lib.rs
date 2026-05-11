use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self,Mint, TokenAccount, TokenInterface,TransferChecked};

declare_id!("8n4Pr3rkXtnXNVqbW6iR1P1fKTxQauXrWbJmurrPGLri");

#[program]
pub mod transfer_check {

use super::*;

    pub fn transfer_tokens(ctx: Context<TransferTokens>,amount: u64) -> Result<()> {
        let decimal = ctx.accounts.mint.decimals;
        let cpi_accounts = TransferChecked {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.sender_token_account.to_account_info(),
            to: ctx.accounts.recipient_token_account.to_account_info(),
            authority: ctx.accounts.signer.to_account_info()
        };
        let cpi_program = ctx.accounts.token_program.key();
        let cpi_context = CpiContext::new(cpi_program,cpi_accounts);
        token_interface::transfer_checked(cpi_context, amount,decimal)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub mint: InterfaceAccount<'info,Mint>,
    #[account(mut)]
    pub sender_token_account: InterfaceAccount<'info,TokenAccount>,
    #[account(mut)]
    pub recipient_token_account: InterfaceAccount<'info,TokenAccount>,
    pub token_program: Interface<'info,TokenInterface>
}
