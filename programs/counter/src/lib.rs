use anchor_lang::prelude::*;

declare_id!("8fVLhVFu2zPFyZedD4T267DpgFCuKhoF83n2om63zV7M");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.bump = ctx.bumps.counter;
        msg!("Initialized counter {}", counter.count);
        msg!("Bump {}", ctx.bumps.counter);
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter {}", counter.count);
        counter.count = counter.count.checked_add(1).unwrap();
        msg!("Current counter {}", counter.count);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(init,seeds=[b"counter"],bump, payer=user, space=8 + Counter::INIT_SPACE)]
    pub counter: Account<'info, Counter>,

    pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut,seeds=[b"counter"], bump=counter.bump)]
    pub counter: Account<'info, Counter>,
}

#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub count: u64,
    pub bump: u8,
}