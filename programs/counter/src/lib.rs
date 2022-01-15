use anchor_lang::prelude::*;

declare_id!("9b1LijhFmtMDfQAuq9aRVqRiFLeG6Edm5JGiDHZu3AUM");

#[program]
pub mod counter {

    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let counter = &mut ctx.accounts.counter;
        counter.admin_auth = *ctx.accounts.user_auth.key;
        counter.count = 0;

        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> ProgramResult {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;

        Ok(())
    }

    pub fn add(ctx: Context<Add>, _value: u32) -> ProgramResult {
        let counter = &mut ctx.accounts.counter;
        counter.count += _value;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user_auth, space = 8 + 32 + 4)]
    pub counter: Account<'info, Counter>,
    pub user_auth: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Add<'info> {
    #[account(mut, has_one = admin_auth)]
    pub counter: Account<'info, Counter>,
    pub admin_auth: Signer<'info>,
}


#[account]
pub struct Counter {
    pub admin_auth: Pubkey,
    pub count: u32,
}
