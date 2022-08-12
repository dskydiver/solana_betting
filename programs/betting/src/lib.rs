pub mod error;
pub mod processor;
pub mod state;
pub mod structures;
pub mod utils;

use structures::{
    create_battle::*,
    bet::*,
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

    pub fn bet(_ctx: Context<Bet>, chosen: Winner) -> Result<()> {
        _ctx.accounts.process(chosen)
    }
}
