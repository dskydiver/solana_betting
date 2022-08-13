use crate::{structures::Finalize, utils};

use anchor_lang::prelude::*;

impl<'info> Finalize<'info> {
    pub fn process(&mut self) -> Result<()> {
        let left_pool = self.battle.left_pool;
        let right_pool = self.battle.right_pool;
        let winner = utils::choose(left_pool, right_pool, self.clock_sysvar.unix_timestamp as u64);
        self.battle.winner = Some(winner);
        self.battle.is_finished = true;

        Ok(())
    }
}
