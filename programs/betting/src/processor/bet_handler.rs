use crate::{error, structures::Bet, utils};

use anchor_lang::prelude::*;

impl<'info> Bet<'info> {
    pub fn process(&mut self, chosen: utils::Winner, amount: u64) -> Result<()> {
        if self.battle.start > self.clock_sysvar.unix_timestamp {
            return Err(error::ErrorCode::BattleNotStarted.into());
        }

        if self.battle.end < self.clock_sysvar.unix_timestamp {
            return Err(error::ErrorCode::BattleEnded.into());
        }

        if self.battle.is_finished {
            return Err(error::ErrorCode::BattleFinished.into());
        }

        if amount == 0 {
            return Err(error::ErrorCode::ZeroAmount.into());
        }

        self.user_betting.authority = self.authority.key().clone();
        self.user_betting.battle = self.battle.key().clone();
        self.user_betting.amount = amount;

        match chosen {
          utils::Winner::Left => {self.battle.left_pool += amount;}
          utils::Winner::Right => {self.battle.right_pool += amount;}
        }

        utils::sys_transfer_unchecked(
            &self.authority.to_account_info(),
            &self.escrow.to_account_info(),
            amount,
        )?;

        Ok(())
    }
}
