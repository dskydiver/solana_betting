use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    /// 6000.
    #[msg("Duration must be set at locked mode.")]
    DurationMissing,

    /// 6001.
    #[msg("Staking not yet started.")]
    StakingNotYetStarted,

    /// 6002.
    #[msg("You cannot unstake the nft in locked mode before the end time.")]
    StakingIsNotFinished,
}
