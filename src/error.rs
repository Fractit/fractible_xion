use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("OnlyOwner is Allowed")]
    OnlyOwner {},

    #[error("Withdraw is paused")]
    WithdrawPause {},

    #[error("Config is already set")]
    ConfigSetAlready {},

    #[error("You are trying to deposit more than max")]
    DepositMoreThanMax {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
