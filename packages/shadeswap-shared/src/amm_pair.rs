use crate::token_pair::TokenPair;
use cosmwasm_std::{HumanAddr, CanonicalAddr, StdResult, Api};
use fadroma::prelude::{Humanize, Canonize};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use shade_protocol::utils::asset::Contract;

/// Represents the address of an exchange and the pair that it manages
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug)]
pub struct AMMPair<A: Clone> {
    /// The pair that the contract manages.
    pub pair: TokenPair<A>,
    /// Address of the contract that manages the exchange.
    pub address: A,
}

impl Canonize for AMMPair<HumanAddr> {
    fn canonize(&self, api: &impl Api) -> StdResult<AMMPair<CanonicalAddr>> {
        Ok(AMMPair {
            pair: self.pair.canonize(api)?,
            address: self.address.canonize(api)?,
        })
    }
}

impl Humanize for AMMPair<CanonicalAddr> {
    fn humanize(&self, api: &impl Api) -> StdResult<AMMPair<HumanAddr>> {
        Ok(AMMPair {
            pair: self.pair.humanize(api)?,
            address: api.human_address(&self.address)?,
        })
    }
}

#[derive(Serialize, Deserialize, JsonSchema, PartialEq, Debug, Clone)]
pub struct AMMSettings {
    pub lp_fee: Fee,
    pub shade_dao_fee: Fee,
    pub shade_dao_address: Contract
}

impl AMMSettings {
    pub fn canonize(&self, api: &impl Api) -> StdResult<AMMSettings> {
        Ok(AMMSettings {
            lp_fee: self.lp_fee,
            shade_dao_fee: self.shade_dao_fee,
            shade_dao_address: self.shade_dao_address.canonize(api)?
        })
    }
}

impl AMMSettings {
    pub fn humanize(self, api: &impl Api) -> StdResult<AMMSettings> {
        Ok(AMMSettings {
            lp_fee: self.lp_fee,
            shade_dao_fee: self.shade_dao_fee,
            shade_dao_address: self.shade_dao_address.humanize(api)?
        })
    }
}

#[derive(Serialize, Deserialize, JsonSchema, PartialEq, Clone, Copy, Debug)]
pub struct Fee {
    pub nom: u8,
    pub denom: u16,
}

impl Fee {
    pub fn new(nom: u8, denom: u16) -> Self {
        Self { nom, denom }
    }
}
