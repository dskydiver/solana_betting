use crate::state::{UserBetting, Battle};
use crate::utils;

use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Bet<'info> {
  #[account(mut)]
  pub authority: Signer<'info>,

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
  pub bet: Box<Account<'info, UserBetting>>,

  #[account(
    mut,
    constraint = battle.key() == bet.battle
  )]
  pub battle: Box<Account<'info, Battle>>,

  pub rent_sysvar: Sysvar<'info, Rent>,
  pub system_program: Program<'info, System>,
}