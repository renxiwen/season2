pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
//pub use state::*;

declare_id!("7LSz4stbs3QRXKTYtptLBJN5Rfx2Hk54uCePcPX8JhV9");

#[program]
pub mod season2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
