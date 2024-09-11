use anchor_lang::prelude::*;

declare_id!("Dmb9m2ToyyWGbBTBkuJQuZTipXVr79da6rvvg15pQA36");

#[program]
pub mod pls_verify {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("need to verify!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
