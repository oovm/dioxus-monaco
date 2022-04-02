pub use graham_number::graham_last_digits;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn main() {
    // Get last 100 digits of the graham number
    println!("{}", graham_last_digits(100));
}
