use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self,TokenAccount, TokenInterface,Mint,MintTo};
use anchor_spl::associated_token::AssociatedToken;
declare_id!("kgij56r35n1rUmUxsSt8sMx6d2D6XbbgKri43Hs2fvA");

#[program]
pub mod mint_token_to_pda {
use super::*;

    pub fn create_mint(ctx: Context<CreateMint>) -> Result<()> {
        msg!("Create Mint Accounts: {:?}", ctx.accounts.mint.key());
        Ok(())
    }

    pub fn mint_token(ctx: Context<MintTokens>,amount: u64) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[b"mint",&[ctx.bumps.mint]]];
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.mint.to_account_info()
        };
        let cpi_program_id = ctx.accounts.token_program.key();
        let cpi_context = CpiContext::new(cpi_program_id,cpi_accounts).with_signer(signer_seeds);
        token_interface::mint_to(cpi_context, amount)?;
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
        mint::authority = mint,
        mint::freeze_authority = mint,
        seeds = [b"mint"],
        bump
    )]
    pub mint: InterfaceAccount<'info,Mint>,
    pub token_program: Interface<'info,TokenInterface>,
    pub system_program: Program<'info,System>
}
#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program,
    )]
    pub token_account: InterfaceAccount<'info,TokenAccount>,
    #[account(
        mut,
        seeds = [b"mint"],
        bump
    )]
    pub mint: InterfaceAccount<'info,Mint>,
    pub token_program: Interface<'info,TokenInterface>,
    pub associated_token_program:Program<'info,AssociatedToken>,
    pub system_program: Program<'info,System>
}
