use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::state::{GameMove, GameData};
use cosmwasm_std::Addr;

// Instantiage message.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

// Add functionality to start game
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    StartGame { opponent: Addr, host_move: GameMove },
}

// Query message to query games based on host.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetGame { host: Addr},
}

// GameResponse struct to return for each GetGame query.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GameResponse {
    pub game: GameData,
}