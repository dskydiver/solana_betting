use crate::state::{Battle, UserBetting};
use crate::utils;

use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Bet<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: it's alright
    // #[account(mut)]
    pub admin: AccountInfo<'info>,

    #[account(
    init,
    seeds = [
      utils::BET_SEED.as_ref(),
      authority.key().as_ref(),
    ],
    bump,
    payer = authority,
    space = 8 + core::mem::size_of::<UserBetting>(),
    )]
    pub user_betting: Account<'info, UserBetting>,

    #[account(
    mut,
    constraint = battle.authority == admin.key()
    )]
    pub battle: Account<'info, Battle>,

    /// CHECK: it's alright
    #[account(
      mut,
      seeds = [utils::ESCROW_SEED.as_ref()],
      bump,
    )]
    pub escrow: AccountInfo<'info>,

    pub clock_sysvar: Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
}
