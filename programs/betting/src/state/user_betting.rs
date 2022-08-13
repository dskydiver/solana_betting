use crate::utils::*;

use anchor_lang::prelude::*;

// User's betting account
#[account]
#[derive(Copy, Debug)]
pub struct UserBetting {
  pub authority: Pubkey, // user
  pub battle: Pubkey, // battle to bet
  pub chosen: Winner, // which side to bet on
  pub amount: u64, // amount of sol to bet
}