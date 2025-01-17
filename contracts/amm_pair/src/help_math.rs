use shadeswap_shared::fadroma::scrt::{Decimal, StdResult, Uint128};

const DECIMAL_FRACTIONAL: Uint128 = Uint128(1_000_000_000u128);

pub fn substraction(nominator: Decimal, denominator: Decimal) -> StdResult<Decimal> {
    Ok(Decimal::from_ratio(
        (nominator * DECIMAL_FRACTIONAL - denominator * DECIMAL_FRACTIONAL)?,
        DECIMAL_FRACTIONAL,
    ))
}

pub fn multiply(nominator: Decimal, denominator: Decimal) -> Decimal {
    Decimal::from_ratio(
        (nominator * DECIMAL_FRACTIONAL * denominator),
        DECIMAL_FRACTIONAL,
    )
}
