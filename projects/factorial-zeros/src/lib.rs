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
pub fn power_mod<A, E, M>(a: &A, e: &E, m: &M) -> BigInt
where
    A: Into<BigInt> + Clone,
    E: Into<BigInt> + Clone,
    M: Into<BigInt> + Clone,
{
    let zero: BigInt = Zero::zero();
    let a: BigInt = a.clone().into();
    let e: BigInt = e.clone().into();
    let m: BigInt = m.clone().into();
    assert!(e >= zero);
    a.modpow(&e, &m)
}
