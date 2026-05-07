use anchor_lang::prelude::*;

declare_id!("7yefzEXZGxsj14hDR4ZaV8WexRAJ9ma6PXtKbu8AH9pM");

#[program]
pub mod zero_copy1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let data_account = &mut ctx.accounts.data_account.load_init()?;
        data_account.data = [1;10232];
        Ok(())
    }
    
    pub fn update(ctx: Context<Update>) -> Result<()> {
        let account = &mut ctx.accounts.data_account.load_mut()?;
        account.data = [2;10232];
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = payer, space = 8 + 10232)]
    pub data_account: AccountLoader<'info, Data>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub data_account: AccountLoader<'info, Data>,
}

#[account(zero_copy)]
pub struct Data {
    pub data: [u8; 10232],
}
