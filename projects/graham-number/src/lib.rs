use num::{BigInt, Integer};
use std::ops::{Div, Mul};

pub fn graham_number(digits: u32) -> String {
    let last = tetration_mod(BigInt::from(3), digits + 1, BigInt::from(10).pow(digits));
    format!("{:0>d$}", last, d = digits as usize)
}

fn tetration_mod(a: BigInt, t: u32, m: BigInt) -> BigInt {
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

#[test]
fn test() {
    let out = graham_number(1000);
    println!("{}", out);
}
