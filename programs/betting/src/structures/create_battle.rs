use crate::state::Battle;
use crate::utils;

use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateBattle<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account{
      init,
      seeds = [
        utils::BATTLE_SEED.as_ref(),
        authority.key().as_ref(),
      ],
      bump,
      payer = authority,
      space = 8 + core::mem::size_of::<Battle>(),
    }]
    pub battle: Box<Account<'info, Battle>>,

    /// CHECK: it's alright
    #[account(
      mut,
      seeds = [utils::ESCROW_SEED.as_ref()],
      bump,
    )]
    pub escrow: UncheckedAccount<'info>,

    pub rent_sysvar: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}
