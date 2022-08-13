use crate::utils::*;

use anchor_lang::prelude::*;

// The Battle account (PDA)
// authority creates it
#[account]
#[derive(Copy, Default)]
pub struct Battle {
  pub authority: Pubkey, // battle creator
  pub start: i64, // start time
  pub end: i64, // end time
  pub winner: Option<Winner>, // winner to be chosen by authority
  pub is_finished: bool, // whether the battle is finished or not (so that users can claim)
  pub left_pool: u64, // the balance for betting left
  pub right_pool: u64, //  the balance fot betting right
  pub admin_claimed: bool, // used for the re-distribution of the team reward
}