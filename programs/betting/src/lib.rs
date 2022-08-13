pub mod error;
pub mod processor;
pub mod state;
pub mod structures;
pub mod utils;

use structures::{
    create_battle::*,
    bet::*,
    finalize::*,
    claim::*,
};
use utils::Winner;

use anchor_lang::prelude::*;

declare_id!("Erk5aovZ7Hd3RCn5XuwvqUfD5LdTKHq14tswLN2wVyuA");

#[program]
pub mod betting {
    use super::*;

    pub fn create_battle(_ctx: Context<CreateBattle>, start: i64, end: i64) -> Result<()> {
        _ctx.accounts.process(start, end, *_ctx.bumps.get("escrow").unwrap())
    }

    pub fn bet(_ctx: Context<Bet>, chosen: Winner, amount: u64) -> Result<()> {
        _ctx.accounts.process(chosen, amount)
    }

    pub fn finalize(_ctx: Context<Finalize>) -> Result<()> {
        _ctx.accounts.process()
    }

    pub fn claim(_ctx: Context<Claim>) -> Result<()> {
        _ctx.accounts.process(*_ctx.bumps.get("escrow").unwrap())
    }
}
