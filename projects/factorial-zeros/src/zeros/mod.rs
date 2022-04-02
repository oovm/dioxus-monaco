use num::{bigint::BigInt, Num};

/// Fast factorial zeros count for copy type.
pub fn factorial_zeros_fast<T>(n: T) -> T
where
    T: Copy + PartialOrd + Num,
{
    let zero = T::zero();
    let one = T::one();
    let five = one + one + one + one + one;
    let mut out = zero;
    let mut n = n;
    while n >= five {
        n = n / five;
        out = out + n;
    }
    out
}

/// Factorial zeros count for any natural number.
///
/// $O(\ln n)$
pub fn factorial_zeros(n: BigInt) -> BigInt {
    let zero = BigInt::from(0);
    let five = BigInt::from(5);
    let mut n: BigInt = n;
    assert!(n >= zero, "n must be a natural number");
    let mut out = zero;
    while n >= five {
        n /= five.clone();
        out += n.clone();
    }
    out
}
