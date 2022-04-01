mod errors;

pub use errors::{Error, Result};
use num::{
    bigint::{BigInt, ToBigInt},
    traits::{One, Zero},
    BigUint, Bounded, Num,
};
use std::ops::Shr;

pub fn power_mod_fast<T>(base: T, exponent: T, modulus: T) -> T
where
    T: Copy + PartialOrd + Num + Bounded,
    T: Shr<T, Output = T>,
{
    let zero: T = Zero::zero();
    let one: T = One::one();
    let two: T = one + one;
    let max: T = Bounded::max_value();
    assert!(modulus != zero);
    assert!((modulus - one) < (max / (modulus - one)));

    let mut result = one;
    let mut base = base % modulus;
    let mut exponent = exponent;

    loop {
        if exponent <= zero {
            break;
        }

        if exponent % two == one {
            result = (result * base) % modulus;
        }

        exponent = exponent >> one;
        base = (base * base) % modulus;
    }

    result
}

fn power_mod<A, E, M>(a: &A, e: &E, m: &M) -> BigInt
where
    A: Into<BigInt> + Clone,
    E: Into<BigInt> + Clone,
    M: Into<BigInt> + Clone,
{
    let zero: BigInt = Zero::zero();
    let one: BigInt = One::one();

    let a: BigInt = a.clone().into();
    let e: BigInt = e.clone().into();
    let m: BigInt = m.clone().into();
    assert!(e >= zero);
    if e == zero {
        return one;
    }
    let mut result = one;
    let mut base = a % &m;
    let mut exp = e;
    loop {
        if &exp % 2 == &one {
            result *= &base;
            result %= &m;
        }

        if exp == one {
            return result;
        }

        exp /= 2;
        base *= base.clone();
        base %= &m;
    }
}

#[test]
fn test() {
    let out = power_mod(&5, &3, &13);
    print!("{}", out);
}
