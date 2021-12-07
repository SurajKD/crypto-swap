use crate::errors::ErrorCode;
use crate::structs::oracle::Oracle;
use crate::structs::pool::Pool;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeOracle<'info> {
    // will uncomment after pool address generation update
    // #[account(init,
    //     seeds = [b"poolv1", fee_tier.to_account_info().key.as_ref(), token_x.key.as_ref(), token_y.key.as_ref()],
    //     bump = bump, payer = payer
    // )]
    #[account(mut)]
    pub pool: Loader<'info, Pool>,
    #[account(zero)]
    pub oracle: Loader<'info, Oracle>,
    pub token_x: AccountInfo<'info>,
    pub token_y: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: AccountInfo<'info>,
}

pub fn handler(ctx: Context<InitializeOracle>) -> ProgramResult {
    msg!("INVARIANT: INITIALIZE ORACLE");

    let oracle = &mut ctx.accounts.oracle.load_init()?;
    let pool = &mut ctx.accounts.pool.load_mut()?;

    require!(
        pool.oracle_initialized == false,
        ErrorCode::OracleAlreadyInitialized
    );

    pool.set_oracle(ctx.accounts.oracle.key());
    oracle.init();

    Ok(())
}
