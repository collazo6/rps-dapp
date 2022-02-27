use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

// State struct which holds contract owner address.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    // pub count: i32,
    pub owner: Addr,
}

// Create GameData struct with relevant game information.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GameData {
    pub host: Addr,
    pub opponent: Addr,
    pub host_move: GameMove,
    pub opp_move: GameMove,
    pub result: GameResult,
}

// Create GameMove enum with possible moves to make in game.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum GameMove {
    Waiting,
    Rock,
    Paper,
    Scissors,
}

// Create GameResult with possible game results.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum GameResult {
    InProgress,
    HostWins,
    OpponentWins,
    Tie,
}

// Create storage variables.
pub const STATE: Item<State> = Item::new("state");
pub const GAMEDATA: Map<&Addr, GameData> = Map::new("game_data");
