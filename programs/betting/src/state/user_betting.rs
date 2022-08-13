use crate::utils::*;

use anchor_lang::prelude::*;

#[account]
#[derive(Copy, Debug)]
pub struct UserBetting {
  pub authority: Pubkey,
  pub battle: Pubkey,
  pub chosen: Winner,
  pub amount: u64,
}