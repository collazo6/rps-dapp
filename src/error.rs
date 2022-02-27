use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {

    // Initialize standard error.
    #[error("{0}")]
    Std(#[from] StdError),

    // Alert if user is unauthorized to execute action.
    #[error("Unauthorized")]
    Unauthorized {},

    // Ensure user may only have one game in session at a time.
    #[error("A game is already in session")]
    GameInSession {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
