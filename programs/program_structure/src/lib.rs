use anchor_lang::prelude::*;

declare_id!("59SenRKAbaVQM2pDrqy9P1uUDxb3ocd44vDqb8omAVYB");

#[program]
pub mod program_structure {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>,data: u64) -> Result<()> {
        ctx.accounts.new_account.data = data;
        msg!("Changed data to :{}",data);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 8)]
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct NewAccount {
    data: u64,
}