use anchor_lang::prelude::*;
declare_id!("DFwuBPK5wVHGW1kduMz6dp5pNUbZPaxj2WowqibbmKfh");

#[program]
pub mod pda {
    use super::*;

    pub fn test_instruction(ctx: Context<InstructionAccounts>) -> Result<()> {
        msg!("PDA account: {}", ctx.accounts.pda_account.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InstructionAccounts<'info> {
    pub signer: Signer<'info>,
    #[account(
        seeds = [b"hello_world",signer.key().as_ref()],
        bump
    )]
    pub pda_account: SystemAccount<'info>,
}
