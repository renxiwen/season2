use anchor_lang::prelude::*;

declare_id!("9SLyQCnidtq2qaW2h2Zn3okJyPVgYof47EqLvCqdaxkG");

declare_program!(example);
use example::{
    accounts::Counter,
    cpi::{
        self,
        accounts::{Increment,Initialize}
    },
    program::Example,
};
#[program]
pub mod dfc {

    use super::*;

    pub fn initialize(ctx: Context<InitializeCpi>) -> Result<()> {
        // Create CPI context for initialize
        let cpi_ctx = CpiContext::new(
            ctx.accounts.example_program.key(),
            Initialize {
                payer: ctx.accounts.payer.to_account_info(),
                counter: ctx.accounts.counter.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            }
        );
        cpi::initialize(cpi_ctx)?;
        Ok(())
    }
    
    pub fn increment(ctx: Context<IncrementCpi>) -> Result<()> {
        //Create CPI context for increment
        let cpi_ctx = CpiContext::new(
            ctx.accounts.example_program.key(),
            Increment {
                counter: ctx.accounts.counter.to_account_info(),
            }
        );
        // Invoke the increment function via CPI
        cpi::increment(cpi_ctx)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCpi<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub counter: Signer<'info>,
    pub system_program: Program<'info,System>,
    pub example_program: Program<'info,Example>,
}

#[derive(Accounts)]
pub struct IncrementCpi<'info> {
    #[account(mut)]
    pub counter: Account<'info,Counter>,
    pub example_program: Program<'info,Example>,
}
