use parity_sample_cargo::{absolute, square};

#[test]
fn applies_standard_discount() {
    let subtotal = square(10) as f64;
    let discounted = parity_sample_cargo::discount::apply_discount(subtotal, false);

    assert_eq!(absolute(discounted as i32), 90);
}
