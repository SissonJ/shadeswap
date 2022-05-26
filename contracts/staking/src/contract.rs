use shadeswap_shared::msg::staking::{{InitMsg, QueryMsg, InvokeMsg, HandleMsg}};
use crate::state::{{Config, ClaimRewardsInfo, store_config, load_claim_reward_timestamp,  store_claim_reward_timestamp,
    get_total_staking_amount, load_stakers, load_config, is_address_already_staker, store_claim_reward_info,
    store_staker, load_staker_info, store_staker_info, remove_staker, StakingInfo, load_claim_reward_info}};
use shadeswap_shared::{ 
    fadroma::{
        scrt::{
            from_binary, log, to_binary, Api, BankMsg, Binary, Coin, CosmosMsg, Decimal, Env,
            Extern, HandleResponse, HumanAddr, InitResponse, Querier, QueryRequest, QueryResult,
            StdError, StdResult, Storage, Uint128, WasmMsg, WasmQuery, 
            secret_toolkit::snip20,        
        },
        scrt_uint256::Uint256,
        scrt_callback::Callback,
        scrt_link::ContractLink,
        scrt_vk::ViewingKey,
    }
};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {

    let config = Config {
        contract_owner: env.message.sender,
        daily_reward_amount: msg.staking_amount,
        reward_token: msg.reward_token.clone(),
        lp_token: msg.lp_token.clone()
    };
    store_config(deps, &config)?;
    Ok(InitResponse {
        messages: vec![],
        log: vec![
           log("staking_contract_addr", env.contract.address),
           log("reward_token", msg.reward_token.clone()),
           log("lp_token", msg.lp_token.clone()),
        ],
    })
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Receive {
           msg, ..
        } => receiver_callback(deps, env, msg),
        HandleMsg::ClaimRewards { } => {
            claim_rewards(deps, env)
        }
        HandleMsg::Unstake {} => unstake(deps,env),
    }    
}

// This should be callback from Snip20 Receiver
// needs to check for the amount
pub fn stake<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    amount: Uint128,
    from: HumanAddr
) -> StdResult<HandleResponse>{

    claim_rewards_for_all_stakers(deps,env.block.time)?;
    let caller = from;
    // check if caller exist
    let is_staker = is_address_already_staker(&deps, caller.clone())?;
    if is_staker == true {
        let mut stake_info = load_staker_info(deps, caller.clone())?;
        stake_info.amount += amount;
        stake_info.last_time_updated = env.block.time;        
        store_staker_info(deps, &stake_info)?;
    }
    else{
        store_staker(deps, caller.clone())?;
        store_staker_info(deps, &StakingInfo{
            staker: caller.clone(),
            amount: amount,
            last_time_updated: env.block.time
        })?;
    }

    // store zero for claim rewards
    store_claim_reward_info(deps, &ClaimRewardsInfo{
        staker: caller.clone(),
        amount: Uint128(0u128),
        last_time_claimed: env.block.time
    })?;

    // return response
    Ok(HandleResponse {
        messages: vec![],
        log: vec![
                log("action", "stake"),
                log("staker", caller.as_str()),
                log("amount", amount),
        ],
        data: None,
    })
}


fn receiver_callback<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: Option<Binary>,
) -> StdResult<HandleResponse> {
    let msg = msg.ok_or_else(|| {
        StdError::generic_err("Receiver callback \"msg\" parameter cannot be empty.")
    })?;

    match from_binary(&msg)? {
        InvokeMsg::Stake {         
            amount,
            from,
        } => {
            return stake(deps,env, amount,from)
        },
        _ => Err(StdError::unauthorized())
    }
}

pub fn claim_rewards<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env
) -> StdResult<HandleResponse>{

    let receiver = env.message.sender.clone();
    let is_user_staker = is_address_already_staker(deps, receiver.clone())?;
    if is_user_staker != true {
        return Err(StdError::unauthorized())
    }

    let mut messages = Vec::new();
    // calculate for all also for user
    claim_rewards_for_all_stakers(deps, env.block.time)?;
    let mut claim_info = load_claim_reward_info(deps, receiver.clone())?;
    let claim_amount = claim_info.amount;
    claim_info.amount = Uint128(0u128);
    claim_info.last_time_claimed = env.block.time;
    store_claim_reward_info(deps, &claim_info)?;   
    let config = load_config(deps)?;
    // send the message
    messages.push(config.reward_token.create_send_msg(
        env.contract.address.clone(),
        receiver.clone(),
        claim_amount,
    )?);    

   
    Ok(HandleResponse {
        messages: messages,
        log: vec![
                log("action", "claim_rewards"),
                log("caller", receiver.as_str().clone()),
                log("reward_amount",claim_amount),
        ],
        data: None,
    })
}

// Total Available Rewards = Daily_Rewards / 24*60*60*1000 * (current_date_time - last_calculated_date_time).miliseconds()
// User Incremental Rewards = Total Available Rewards * Staked Percentage
// User Total Rewards = User Owed Rewards + (User Incremental Rewards)
pub fn claim_rewards_for_all_stakers<S:Storage, A:Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    current_timestamp: u64
) -> StdResult<()> {
    let stakers = load_stakers(deps)?;
    let last_timestamp = load_claim_reward_timestamp(deps)?;
    for staker in stakers.into_iter() {
        let mut claim_info = load_claim_reward_info(deps, staker.clone())?;
        let staking_reward = calculate_staking_reward(deps, staker.clone(),last_timestamp, current_timestamp)?;
        claim_info.amount += staking_reward;
        claim_info.last_time_claimed = current_timestamp;
        store_claim_reward_info(deps, &claim_info)?;
    }
    store_claim_reward_timestamp(deps, current_timestamp)?;
    Ok(())
}

pub fn calculate_staking_reward<S:Storage, A:Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    staker: HumanAddr,
    last_timestamp: u64,
    current_timestamp: u64
) -> StdResult<Uint128>{
    let cons = Uint128(100u128);
    let percentage = Uint256::from(get_staking_percentage(deps,staker, cons)?);
    let config = load_config(deps)?;
    let milisec = Uint256::from(Uint128(24u128 * 60u128 *60u128 * 1000u128));   
    let converted_current_time = Uint256::from(Uint128(current_timestamp as u128));
    let converted_last_time = Uint256::from(Uint128(last_timestamp as u128));
    let time_dif = (converted_current_time - converted_last_time)?;   
    let milisec_offset = ((milisec * time_dif)? * Uint256::from(Uint128(1000u128)))?;            
    if milisec_offset != Uint256::from(Uint128(0u128)) {
        let total_available_reward = (Uint256::from(config.daily_reward_amount) / milisec_offset)?;
        let result = ((total_available_reward * percentage)? / Uint256::from(cons))?;
        Ok(Uint128(result.clamp_u128()?))
    }else{
        Ok(Uint128(0u128))
    }
   
}

pub fn get_staking_percentage<S:Storage, A:Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    staker: HumanAddr,
    cons: Uint128
) -> StdResult<Uint128> {
    let total_staking = Uint256::from(get_total_staking_amount(deps)?);
    let stake_info = load_staker_info(&deps, staker)?;
    let stake_amount = Uint256::from(stake_info.amount);   
    let percentage =((stake_amount * Uint256::from(cons))? / total_staking)?;
    Ok(Uint128(percentage.clamp_u128()?))
}


pub fn unstake<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env
) -> StdResult<HandleResponse>{
    let caller = env.message.sender.clone();
    let is_user_staker = is_address_already_staker(deps, caller.clone())?;
    let config = load_config(deps)?;
    if is_user_staker != true {
        return Err(StdError::unauthorized())
    }
    // claim rewards
    claim_rewards_for_all_stakers(deps, env.block.time)?;
    // remove staker
    remove_staker(deps, caller.clone())?;
    let mut messages = Vec::new();
    // update stake_info
    let mut staker_info = load_staker_info(deps, caller.clone())?;
    // send back lp token
    messages.push(config.lp_token.create_send_msg(
        env.contract.address.clone(),
        caller.clone(),
        staker_info.amount,
    )?);   
        
    staker_info.amount = Uint128(0);
    staker_info.last_time_updated = env.block.time;
    store_staker_info(deps, &staker_info)?;

    // send reward if any and 
    let mut claim_reward = load_claim_reward_info(deps, caller.clone())?;
    // send all remaing reward token
    messages.push(config.reward_token.create_send_msg(
        env.contract.address.clone(),
        caller.clone(),
        claim_reward.amount,
    )?); 

    // update claim  reward for staker
    claim_reward.amount = Uint128(0);
    claim_reward.last_time_claimed = 0;
    store_claim_reward_info(deps, &ClaimRewardsInfo{
        staker: caller.clone(),
        amount: Uint128(0),
        last_time_claimed: 0
    })?;
  
    Ok(HandleResponse {
        messages: messages,
        log: vec![
                log("action", "unstake"),
                log("staker", caller.as_str()),
        ],
        data: None,
    })
}