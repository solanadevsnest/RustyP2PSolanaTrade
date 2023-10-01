use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowError {
    #[msg("The current stage is not suitable for an exchange or cancellation.")]
    InvalidStage,
    #[msg("There are insufficient funds available for this operation.")]
    InsufficientFunds,
    #[msg("The specified mint account for the trade is not valid.")]
    InvalidMint,
    #[msg("A required mint for the trade is absent.")]
    MissingMint,
    #[msg("The trade type is invalid, possibly due to missing mint addresses.")]
    InvalidTradeType,
    #[msg("Invalid association between the provided token accounts.")]
    InvalidAccount,
    #[msg("Duplicate mint accounts are not allowed for this operation.")]
    DuplicateMint,
    #[msg("The account does not have a valid owner.")]
    InvalidOwner,
    #[msg("The specified partner is not valid for this trade.")]
    InvalidPartner,
    #[msg("Both trade value and receive value must be greater than zero.")]
    ZeroValue,
    #[msg("Instruction data is missing necessary parameters.")]
    MissingParams,
}
