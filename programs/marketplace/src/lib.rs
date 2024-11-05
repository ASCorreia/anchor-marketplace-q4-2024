use anchor_lang::prelude::*;

mod state;
mod instructions;
mod errors;

use instructions::*;

declare_id!("EQdo53GUM11GBviXdoGk268AXhpwXtqEyfZbRfrEhAw1");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
