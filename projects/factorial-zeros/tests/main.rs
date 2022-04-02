use factorial_zeros::{factorial_zeros, factorial_zeros_fast};
use num::BigInt;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let out = factorial_zeros_fast(123);
    println!("{}", out);
    let out = factorial_zeros(BigInt::from(123));
    println!("{}", out);
}
