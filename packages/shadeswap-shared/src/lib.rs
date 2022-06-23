pub mod msg;
pub use fadroma;
pub use token_pair::*;
pub use token_type::*;
pub use token_amount::*;
pub use msg::*;
pub use token_pair_amount::*;
pub mod token_pair;
pub mod token_type;
pub mod token_amount;
pub mod token_pair_amount;
pub mod amm_pair;
pub mod admin;
pub mod stake_contract;

#[cfg(not(target_arch = "wasm32"))]
pub mod querier;
mod display;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Pagination {
    pub start: u64,
    pub limit: u8,
}
