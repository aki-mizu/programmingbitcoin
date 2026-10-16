//! Runnable versions of the Chapter 1 examples.

use crate::ecc::{FieldElement, FieldElementError};

// tag::example1[]
/// Corresponds to Example 1.
pub fn example_1() -> Result<[bool; 2], FieldElementError> {
    let first = FieldElement::new(7, 13)?;
    let second = FieldElement::new(6, 13)?;
    let equivalent = FieldElement::new(7, 13)?;

    Ok([first == second, first == equivalent])
}
// end::example1[]

// tag::example2[]
/// Corresponds to Example 2.
pub const fn example_2() -> u64 {
    7 % 3
}
// end::example2[]

// tag::example3[]
/// Corresponds to Example 3, using Rust's Euclidean remainder for mathematical modulo.
pub const fn example_3() -> i64 {
    (-27_i64).rem_euclid(13)
}
// end::example3[]

// tag::example4[]
/// Corresponds to Example 4.
pub fn example_4() -> Result<bool, FieldElementError> {
    let first = FieldElement::new(7, 13)?;
    let second = FieldElement::new(12, 13)?;
    let expected = FieldElement::new(6, 13)?;

    Ok(first.checked_add(second)? == expected)
}
// end::example4[]

// tag::example5[]
/// Corresponds to Example 5.
pub fn example_5() -> Result<bool, FieldElementError> {
    let first = FieldElement::new(3, 13)?;
    let second = FieldElement::new(12, 13)?;
    let expected = FieldElement::new(10, 13)?;

    Ok(first.checked_mul(second)? == expected)
}
// end::example5[]

// tag::example6[]
/// Corresponds to Example 6.
pub fn example_6() -> Result<bool, FieldElementError> {
    let first = FieldElement::new(3, 13)?;
    let expected = FieldElement::new(1, 13)?;

    Ok(first.pow(3) == expected)
}
// end::example6[]

// tag::example7[]
/// Corresponds to Example 7.
pub fn example_7() -> Result<bool, FieldElementError> {
    let first = FieldElement::new(7, 13)?;
    let expected = FieldElement::new(8, 13)?;

    Ok(first.pow(-3) == expected)
}
// end::example7[]

#[cfg(test)]
mod tests {
    use super::{example_1, example_2, example_3, example_4, example_5, example_6, example_7};

    #[test]
    fn reproduces_the_chapter_examples() {
        assert_eq!(example_1().unwrap(), [false, true]);
        assert_eq!(example_2(), 1);
        assert_eq!(example_3(), 12);
        assert!(example_4().unwrap());
        assert!(example_5().unwrap());
        assert!(example_6().unwrap());
        assert!(example_7().unwrap());
    }
}
