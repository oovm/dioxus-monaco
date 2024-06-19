#![doc=include_str!("../Readme.md")]

use num::{BigInt, Integer, One, Zero};
use std::ops::{Div, Mul};

/// Computes the last n digits of [graham's number](https://en.wikipedia.org/wiki/Graham%27s_number).
///
/// Returns a string because of leading zeroes
///
/// # Examples
///
/// ```rust
/// use graham_number::graham_last_digits;
///
/// assert_eq!("2464195387", graham_last_digits(10))
/// ```
pub fn graham_last_digits(digits: u32) -> String {
    let last = tetration_mod(BigInt::from(3), digits + 1, BigInt::from(10).pow(digits));
    format!("{:0>d$}", last, d = digits as usize)
}

fn tetration_mod(a: BigInt, t: u32, m: BigInt) -> BigInt {
    if m.is_zero() {
        panic!("Cannot divide by zero {} {}", a, t)
    }
    match t {
        0 => BigInt::from(0),
        1 => a.mod_floor(&m),
        2 => a.modpow(&a, &m),
        _ => {
            let phi = m.clone().mul(BigInt::from(2)).div(BigInt::from(5));
            let sub = phi.clone() + tetration_mod(a.clone(), t - 1, phi);
            a.modpow(&sub, &m)
        }
    }
}