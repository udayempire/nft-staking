use anchor_lang::prelude::*;

declare_id!("H2dPs9GHvp46vWdgy4trREG9FN4modF7yWWTmiv1ffNv");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
