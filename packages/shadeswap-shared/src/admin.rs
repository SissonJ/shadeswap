use cosmwasm_std::{HumanAddr, Storage, StdResult, StdError, HandleResponse, Querier, log, Extern, Env, Api};
use cosmwasm_storage::{Singleton, singleton, ReadonlySingleton, singleton_read};



pub static ADMIN: &[u8] =b"contract_pair_admin";

pub fn apply_admin_guard(
    caller: HumanAddr,
    storage: &impl Storage,
) -> StdResult<bool> {    
    let admin_address = load_admin(storage)?;
    if caller.as_str() != admin_address.as_str() {
         return Err(StdError::unauthorized())
    }
    return Ok(true)
}

pub fn store_admin <S: Storage, A: Api, Q: Querier>(
    deps:  &mut Extern<S, A, Q>,
    admin: &HumanAddr
) -> StdResult<()> {
    admin_w(&mut deps.storage).save(admin)
}

pub fn load_admin(storage: &impl Storage) -> StdResult<HumanAddr> {
    let admin = admin_r(storage).load()?;
    Ok(admin)
}

pub fn admin_w<S: Storage>(storage: &mut S) -> Singleton<S, HumanAddr> {
    singleton(storage, ADMIN)
}

pub fn admin_r<S: Storage>(storage: &S) -> ReadonlySingleton<S, HumanAddr> {
    singleton_read(storage, ADMIN)
}

pub fn set_admin_guard<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>, 
    env: Env,
    admin: HumanAddr
) -> StdResult<HandleResponse>{
    let sender = env.message.sender.clone();
    apply_admin_guard(sender.clone(), &deps.storage)?;
    store_admin(deps,&admin)?;
    Ok(HandleResponse {
        messages: vec![],
        log: vec![
                log("action", "set_admin_guard"),
                log("caller", sender.clone()),
                log("admin", admin),
        ],
        data: None,
    })

}