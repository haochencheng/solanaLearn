mod creatSolNewAccount;

use anchor_lang::prelude::*;

declare_id!("37pa16Ug6LRes3cdCAGr5L6PG86G7o6aYB26e7TKyx4q");

#[program]
pub mod my_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
