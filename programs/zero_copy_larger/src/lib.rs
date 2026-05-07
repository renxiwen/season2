use anchor_lang::prelude::*;

declare_id!("AVqkgwn5EGiEr9vMJXckC9UY3vF1ynQhtDwGr3Cfq14i");

#[program]
pub mod zero_copy_larger {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let account = &mut ctx.accounts.data_account.load_init()?;
        account.data = [1;10_485_752];
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(zero)]
    pub data_account: AccountLoader<'info,Data>
}

#[account(zero_copy)]
pub struct Data {
    pub data: [u8;10_485_752],
}
