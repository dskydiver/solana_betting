use crate::state::{Battle, UserBetting};
use crate::utils;

use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: it's alright
    #[account(mut)]
    pub admin: AccountInfo<'info>,

    #[account(
      mut,
      constraint = user_betting.authority == authority.key()
    )]
    pub user_betting: Account<'info, UserBetting>,

    #[account(
      mut,
      constraint = battle.authority == admin.key()
    )]
    pub battle: Box<Account<'info, Battle>>,

    /// CHECK: it's alright
    #[account(
      mut,
      seeds = [
        utils::ESCROW_SEED.as_ref(),
        admin.key().as_ref(),
      ],
      bump,
    )]
    pub escrow: UncheckedAccount<'info>,

    pub clock_sysvar: Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
}
