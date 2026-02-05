use anchor_lang::{
    prelude::*,
    system_program::{transfer,Transfer}
};

declare_id!("JDobQqgaVwPbnxWkxXvLmAL4wZpywosvwWjFHPNaEsFq");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
    }
    

}

#[derive(Accounts)]
pub struct Initialize {}
