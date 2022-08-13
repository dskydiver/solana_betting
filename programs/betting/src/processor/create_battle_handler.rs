use crate::{structures::CreateBattle, utils};

use anchor_lang::{prelude::*};

impl<'info> CreateBattle<'info> {
    pub fn process(&mut self, start: i64, end: i64, id: &Pubkey, escrow_bump: u8) -> Result<()> {
        self.battle.authority = self.authority.key().clone();
        self.battle.start = start;
        self.battle.end = end;
        self.battle.is_finished = false;

        // create escrow account
        utils::sys_create_account(
            &self.authority.to_account_info(),
            &self.escrow.to_account_info(),
            self.rent_sysvar.minimum_balance(utils::ORDER_ESCROW_NATIVE_SIZE),
            utils::ORDER_ESCROW_NATIVE_SIZE,
            id,
            &[
                utils::ESCROW_SEED.as_ref(),
                self.authority.key().as_ref(),
                &[escrow_bump],
            ],
        )?;

        Ok(())
    }
}
