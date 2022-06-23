
use cosmwasm_math_compat::Uint128;
use cosmwasm_std::HumanAddr;
use fadroma::prelude::ContractInstantiationInfo;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::TokenType;

use crate::token_amount::TokenAmount;
use crate::token_pair_amount::TokenPairAmount;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct StakingContractInit{
    pub contract_info: ContractInstantiationInfo,
    pub amount: Uint128,
    pub reward_token: TokenType<HumanAddr>    
}