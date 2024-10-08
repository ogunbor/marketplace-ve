use anchor_lang::prelude::*;

declare_id!("E45HbtzPeXavaFeMFeXr83GiNcQFqhkn6xJCoJnY6uuA");

pub mod contexts;
pub mod errors;
pub mod state;

pub use contexts::*;
pub use errors::*;

#[program]
pub mod market_place {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
