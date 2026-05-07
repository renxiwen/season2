use anchor_lang::prelude::*;

declare_id!("CeVtGbv79Q4XPr6AT99eT8pJfH7wZszaSTH1fNytRX8d");

#[program]
pub mod custom_err {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, input: u64) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        if input > 100 {
            return Err(CustomError::NumberTooLarge.into());
        }
        require_gte!(input,100,CustomError::NumberTooSmall);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[error_code]
pub enum CustomError {
    #[msg("The number is too large")]
    NumberTooLarge,
    #[msg("The number is too small")]
    NumberTooSmall,
}
