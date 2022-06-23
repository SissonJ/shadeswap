use cosmwasm_math_compat::Uint128;
use cosmwasm_std::{StdResult, Env};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::token_type::TokenType;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenAmount<A> {
    pub token: TokenType<A>,
    pub amount: Uint128
}

impl<A: Clone> TokenAmount<A> {
    pub fn assert_sent_native_token_balance(&self, env: &Env) -> StdResult<()> {
        self.token.assert_sent_native_token_balance(env, self.amount)
    }
}