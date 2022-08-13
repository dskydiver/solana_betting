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

declare_id!("Ch9LB7Yawyz5PJ9A88hzWZV2z87ZjrxAcFjBqeUs28C6");

#[program]
pub mod betting {
    use super::*;

    pub fn create_battle(_ctx: Context<CreateBattle>, start: i64, end: i64) -> Result<()> {
        _ctx.accounts.process(start, end, _ctx.program_id, *_ctx.bumps.get("escrow").unwrap())
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
