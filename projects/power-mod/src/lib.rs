mod errors;

pub use errors::{Error, Result};
use num::{
    bigint::BigInt,
    traits::{One, Zero},
    Bounded, Num,
};
use std::ops::Shr;

/// Fast modular exponentiation for copy type.
pub fn power_mod_fast<A>(a: A, e: A, m: A) -> A
where
    A: Copy + PartialOrd + Num + Bounded,
    A: Shr<A, Output = A>,
{
    let zero: A = Zero::zero();
    let one: A = One::one();
    let two: A = one + one;
    let max: A = Bounded::max_value();
    assert!(m != zero);
    assert!((m - one) < (max / (m - one)));

    let mut result = one;
    let mut base = a % m;
    let mut exponent = e;

    loop {
        if exponent <= zero {
            break;
        }
        if exponent % two == one {
            result = (result * base) % m;
        }
        exponent = exponent >> one;
        base = (base * base) % m;
    }
    result
}

/// Power mod for all integer
pub fn power_mod(a: &BigInt, e: &BigInt, m: &BigInt) -> BigInt {
    a.modpow(&e, &m)
}
