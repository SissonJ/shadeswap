use shadeswap_shared::{
    fadroma::{
        scrt_link::{ContractLink},    
        scrt_addr::{Humanize, Canonize},
        scrt::{
            Api, CanonicalAddr, Extern, HumanAddr, Uint128,
            Querier, StdResult, Storage, StdError
        },
        scrt_storage::{load, save},
        scrt_vk::ViewingKey,
    },
    token_pair::TokenPair
};

use serde::{Serialize,Deserialize};

pub static CONFIG_KEY: &[u8] = b"config";
pub const BLOCK_SIZE: usize = 256;

#[derive(Serialize, Deserialize,  PartialEq, Debug)]
pub struct Config<A: Clone> {
    pub symbol:        String,
    pub factory_info:  ContractLink<A>,
    pub lp_token_info: ContractLink<A>,
    pub pair:      TokenPair<A>,
    pub contract_addr: A,
    pub viewing_key: ViewingKey,
}


impl Canonize<Config<CanonicalAddr>> for Config<HumanAddr> {
    fn canonize (&self, api: &impl Api) -> StdResult<Config<CanonicalAddr>> {
        Ok(Config {
            symbol:        self.symbol.to_string(),
            factory_info:  self.factory_info.canonize(api)?,
            lp_token_info: self.lp_token_info.canonize(api)?,
            pair:          self.pair.canonize(api)?,
            contract_addr: self.contract_addr.canonize(api)?,
            viewing_key:   self.viewing_key.clone()
        })
    }
}
impl Humanize<Config<HumanAddr>> for Config<CanonicalAddr> {
    fn humanize (&self, api: &impl Api) -> StdResult<Config<HumanAddr>> {
        Ok(Config {
            symbol:        self.symbol.to_string(),
            factory_info:  self.factory_info.humanize(api)?,
            lp_token_info: self.lp_token_info.humanize(api)?,
            pair:      self.pair.humanize(api)?,
            contract_addr: self.contract_addr.humanize(api)?,
            viewing_key:   self.viewing_key.clone()
        })
    }
}


pub(crate) fn store_config <S: Storage, A: Api, Q: Querier>(
    deps:   &mut Extern<S, A, Q>,
    config: &Config<HumanAddr>
) -> StdResult<()> {
    save(&mut deps.storage, CONFIG_KEY, &config.canonize(&deps.api)?)
}

pub(crate) fn load_config<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>
) -> StdResult<Config<HumanAddr>> {
    let result: Config<CanonicalAddr> = load(&deps.storage, CONFIG_KEY)?.ok_or(
        StdError::generic_err("Config doesn't exist in storage.")
    )?;
    result.humanize(&deps.api)
}


pub mod swapdetails {
    use super::*;

    #[derive(Serialize, Deserialize,  PartialEq, Debug)]
    pub struct SwapInfo {
        pub total_fee_amount: Uint128,
        pub lp_fee_amount: Uint128,
        pub shade_dao_fee_amount: Uint128,
        pub result: SwapResult,
    }
    
    #[derive(Serialize, Deserialize,  PartialEq, Debug)]
    pub struct SwapResult {
        pub return_amount: Uint128,
        pub spread_amount: Uint128,
    }
    
}