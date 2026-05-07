use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer,transfer};

declare_id!("9RFddyArv28ekPdDuR8JdzhHtnpBSQUahXFcruAyNV1A");

#[program]
pub mod cpi {
    

    use super::*;

    pub fn initialize(ctx: Context<SolTransfer>,amount:u64) -> Result<()> {
        let from = ctx.accounts.sender.to_account_info();
        let to = ctx.accounts.receiver.to_account_info();
        let program_id = ctx.accounts.system_program.key();
        
        let cpi_context = CpiContext::new(
            program_id,
            Transfer {
                from,
                to,
            },
        );
        transfer(cpi_context, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SolTransfer<'info> {
    #[account(mut)]
    sender: Signer<'info>,
    #[account[mut]]
    receiver: SystemAccount<'info>,
    system_program: Program<'info,System>,
}
