use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    /// 6000.
    #[msg("Battle not started yet.")]
    BattleNotStarted,

    /// 6001
    #[msg("Battle ended.")]
    BattleEnded,

    /// 6002
    #[msg("Battle finished.")]
    BattleFinished,
}
