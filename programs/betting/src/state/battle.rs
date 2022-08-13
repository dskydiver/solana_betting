use crate::utils::*;

use anchor_lang::prelude::*;

#[account]
#[derive(Copy, Default)]
pub struct Battle {
  pub authority: Pubkey,
  pub start: i64,
  pub end: i64,
  pub winner: Option<Winner>,
  pub is_finished: bool,
  pub left_pool: u64,
  pub right_pool: u64,
  pub admin_claimed: bool,
}