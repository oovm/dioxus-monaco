use num::{BigInt, Integer, Zero};
use std::ops::{Div, Mul};

pub fn tetration_mod(a: BigInt, t: BigInt, m: BigInt) -> BigInt {
    match t {
        _ if t.is_zero() => BigInt::from(0),
        _ if t.eq(&BigInt::from(1)) => a.mod_floor(&m),
        _ if t.eq(&BigInt::from(2)) => a.modpow(&a, &m),
        _ => {
            // let phi = euler_phi(m.clone());
            let phi = m.clone().mul(BigInt::from(2)).div(BigInt::from(5));
            let sub = phi.clone() + tetration_mod(a.clone(), t - 1, phi);
            a.modpow(&sub, &m)
        }
    }
}

#[test]
fn test() {
    let digits = 100;
    let out = tetration_mod(BigInt::from(3), BigInt::from(digits + 1), BigInt::from(10).pow(digits));
    println!("{}", out);
}

fn one() -> BigInt {
    BigInt::from(1)
}
fn zero() -> BigInt {
    BigInt::from(0)
}

fn euler_phi(a: BigInt) -> BigInt {
    let mut out = zero();
    let mut o = a.clone();
    while o != zero() {
        if a.gcd(&o) == one() {
            out += one();
        }
        o -= one()
    }
    out
}
