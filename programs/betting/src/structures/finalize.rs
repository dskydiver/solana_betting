use crate::state::Battle;
// use crate::utils;

use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Finalize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
      mut,
      constraint = battle.authority == authority.key()
    )]
    pub battle: Box<Account<'info, Battle>>,

    pub clock_sysvar: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}
