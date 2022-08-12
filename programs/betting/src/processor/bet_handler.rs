use crate::{error, structures::Bet, utils};

use anchor_lang::{prelude::*};

impl<'info> Bet<'info> {
  pub fn process(&mut self, chosen: utils::Winner) -> Result<()> {
    Ok(())
  }
}