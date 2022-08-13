use crate::{error, structures::Claim, utils};

use anchor_lang::prelude::*;

impl<'info> Claim<'info> {
    pub fn process(&mut self, escrow_bump: u8) -> Result<()> {
        if !self.battle.is_finished {
            return Err(error::ErrorCode::BattleNotFinished.into());
        }

        let winner: utils::Winner = self.battle.winner.unwrap();

        if winner == self.user_betting.chosen {
            let total = self.battle.left_pool + self.battle.right_pool;
            let colleague = match self.user_betting.chosen {
                utils::Winner::Left => self.battle.left_pool,
                utils::Winner::Right => self.battle.right_pool,
            };
            let amount = (total * 9 / 10).checked_mul(self.user_betting.amount).unwrap().checked_div(colleague).unwrap();

            utils::move_lamports(
                &self.escrow.to_account_info(),
                &self.authority.to_account_info(),
                amount,
            )?;

            // utils::sys_transfer(
            //     &self.escrow.to_account_info(),
            //     &self.authority.to_account_info(),
            //     amount,
            //     &[utils::ESCROW_SEED.as_ref(), &[escrow_bump]],
            // )?;

            if !self.battle.admin_claimed {
                self.battle.admin_claimed = true;
                let admin_share = total / 10;
                utils::move_lamports(
                    &self.escrow.to_account_info(),
                    &self.admin.to_account_info(),
                    admin_share,
                )?;

                // utils::sys_transfer(
                //     &self.escrow.to_account_info(),
                //     &self.admin.to_account_info(),
                //     admin_share,
                //     &[utils::ESCROW_SEED.as_ref(), &[escrow_bump]],
                // )?;
            }
        }

        // Delete the vault account
        utils::delete_account(
            &self.user_betting.to_account_info(),
            &self.authority.to_account_info(),
        )?;

        Ok(())
    }
}
