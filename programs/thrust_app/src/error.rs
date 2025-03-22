use anchor_lang::prelude::error_code;

#[error_code]
pub enum ThrustAppError {
    #[msg("Uninitialized")]
    Uninitialized,

    #[msg("AlreadyInitialized")]
    AlreadyInitialized,

    #[msg("AlreadyWithdrawn")]
    AlreadyWithdrawn,

    #[msg("Unauthorised")]
    Unauthorised,

    #[msg("Trade not started yet")]
    TradeStartTimeNotReached,

    #[msg("Insufficient fund")]
    InsufficientFund,

    #[msg("One token should be Sol")]
    UnknownToken,

    #[msg("BondingCurve incomplete")]
    BondingCurveIncomplete,

    #[msg("BondingCurve complete")]
    BondingCurveComplete,

    #[msg("Invalid Signature")]
    InvalidSignature,

    #[msg("Invalid Message")]
    InvalidMessage,

    #[msg("Invalid Public Key")]
    InvalidPubkey,

    #[msg("Insufficient trades")]
    InsufficientTrades,

    #[msg("Exceeds buy limit")]
    ExceedsWalletLimit,
}
