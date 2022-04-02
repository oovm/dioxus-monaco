use power_mod::{power_mod, power_mod_fast};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let out = power_mod_fast(5, 3, 13);
    print!("{}", out);
    let out = power_mod(&5, &3, &13);
    print!("{}", out);
}
