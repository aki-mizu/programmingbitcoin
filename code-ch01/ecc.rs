//! Chapter 1: finite-field arithmetic.

use std::error::Error;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

// tag::source1[]
/// An error returned when finite-field arithmetic cannot be performed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldElementError {
    InvalidPrime { prime: u64 },
    ValueOutOfRange { value: u64, prime: u64 },
    MismatchedFields { left_prime: u64, right_prime: u64 },
    DivisionByZero,
}

/// An element of the prime finite field $F_p$.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct FieldElement {
    value: u64,
    prime: u64,
}

impl FieldElement {
    /// Creates an element whose value belongs to the field $F_p$.
    pub fn new(value: u64, prime: u64) -> Result<Self, FieldElementError> {
        if prime < 2 {
            return Err(FieldElementError::InvalidPrime { prime });
        }
        if value >= prime {
            return Err(FieldElementError::ValueOutOfRange { value, prime });
        }

        Ok(Self { value, prime })
    }
// end::source1[]

    /// Returns this element's canonical value, in the range $0..p$.
    pub const fn value(self) -> u64 {
        self.value
    }

    /// Returns the order of the field containing this element.
    pub const fn prime(self) -> u64 {
        self.prime
    }

    // tag::source2[]
    /// Adds two elements from the same field.
    pub fn checked_add(self, other: Self) -> Result<Self, FieldElementError> {
        self.ensure_same_field(other)?;
        Ok(Self::from_reduced(
            ((self.value as u128 + other.value as u128) % self.prime as u128) as u64,
            self.prime,
        ))
    }
    // end::source2[]

    /// Subtracts two elements from the same field.
    pub fn checked_sub(self, other: Self) -> Result<Self, FieldElementError> {
        self.ensure_same_field(other)?;
        Ok(Self::from_reduced(
            ((self.value as u128 + self.prime as u128 - other.value as u128) % self.prime as u128)
                as u64,
            self.prime,
        ))
    }

    /// Multiplies two elements from the same field.
    pub fn checked_mul(self, other: Self) -> Result<Self, FieldElementError> {
        self.ensure_same_field(other)?;
        Ok(Self::from_reduced(
            ((self.value as u128 * other.value as u128) % self.prime as u128) as u64,
            self.prime,
        ))
    }

    // tag::source3[]
    /// Raises this element to an integer power, including negative powers.
    pub fn pow(self, exponent: i128) -> Self {
        let group_order = self.prime - 1;
        let reduced_exponent = exponent.rem_euclid(group_order as i128) as u64;
        Self::from_reduced(
            modular_pow(self.value, reduced_exponent, self.prime),
            self.prime,
        )
    }
    // end::source3[]

    /// Divides two elements from the same field using Fermat's little theorem.
    pub fn checked_div(self, other: Self) -> Result<Self, FieldElementError> {
        self.ensure_same_field(other)?;
        if other.value == 0 {
            return Err(FieldElementError::DivisionByZero);
        }

        self.checked_mul(other.pow((self.prime - 2) as i128))
    }

    fn from_reduced(value: u64, prime: u64) -> Self {
        Self { value, prime }
    }

    fn ensure_same_field(self, other: Self) -> Result<(), FieldElementError> {
        if self.prime == other.prime {
            Ok(())
        } else {
            Err(FieldElementError::MismatchedFields {
                left_prime: self.prime,
                right_prime: other.prime,
            })
        }
    }
}

impl fmt::Display for FieldElementError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPrime { prime } => {
                write!(formatter, "field prime must be at least 2, got {prime}")
            }
            Self::ValueOutOfRange { value, prime } => {
                write!(formatter, "value {value} is not in field range 0 to {}", prime - 1)
            }
            Self::MismatchedFields {
                left_prime,
                right_prime,
            } => write!(
                formatter,
                "cannot operate on elements of F_{left_prime} and F_{right_prime}"
            ),
            Self::DivisionByZero => write!(formatter, "cannot divide by zero in a finite field"),
        }
    }
}

impl Error for FieldElementError {}

impl fmt::Display for FieldElement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "FieldElement_{}({})", self.prime, self.value)
    }
}

impl fmt::Debug for FieldElement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, formatter)
    }
}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        self.checked_add(other)
            .expect("cannot add elements from different finite fields")
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self.checked_sub(other)
            .expect("cannot subtract elements from different finite fields")
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        self.checked_mul(other)
            .expect("cannot multiply elements from different finite fields")
    }
}

impl Div for FieldElement {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        self.checked_div(other)
            .expect("cannot divide elements from different finite fields or by zero")
    }
}

fn modular_pow(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    let mut result = 1_u64;

    while exponent > 0 {
        if exponent % 2 == 1 {
            result = ((result as u128 * base as u128) % modulus as u128) as u64;
        }
        base = ((base as u128 * base as u128) % modulus as u128) as u64;
        exponent /= 2;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::{FieldElement, FieldElementError};

    fn element(value: u64, prime: u64) -> FieldElement {
        FieldElement::new(value, prime).unwrap()
    }

    #[test]
    fn rejects_values_outside_the_field() {
        assert_eq!(
            FieldElement::new(31, 31),
            Err(FieldElementError::ValueOutOfRange {
                value: 31,
                prime: 31,
            })
        );
        assert_eq!(
            FieldElement::new(0, 1),
            Err(FieldElementError::InvalidPrime { prime: 1 })
        );
    }

    #[test]
    fn compares_elements_by_value_and_field() {
        assert_eq!(element(2, 31), element(2, 31));
        assert_ne!(element(2, 31), element(15, 31));
        assert_ne!(element(2, 31), element(2, 29));
    }

    #[test]
    fn adds_elements_modulo_the_field_order() {
        assert_eq!(element(2, 31) + element(15, 31), element(17, 31));
        assert_eq!(element(17, 31) + element(21, 31), element(7, 31));
    }

    #[test]
    fn subtracts_elements_modulo_the_field_order() {
        assert_eq!(element(29, 31) - element(4, 31), element(25, 31));
        assert_eq!(element(15, 31) - element(30, 31), element(16, 31));
    }

    #[test]
    fn multiplies_elements_modulo_the_field_order() {
        assert_eq!(element(24, 31) * element(19, 31), element(22, 31));
    }

    #[test]
    fn reduces_positive_and_negative_exponents() {
        assert_eq!(element(17, 31).pow(3), element(15, 31));
        assert_eq!(element(5, 31).pow(5) * element(18, 31), element(16, 31));
        assert_eq!(element(17, 31).pow(-3), element(29, 31));
        assert_eq!(element(4, 31).pow(-4) * element(11, 31), element(13, 31));
    }

    #[test]
    fn divides_elements_with_their_multiplicative_inverse() {
        assert_eq!(element(3, 31) / element(24, 31), element(4, 31));
    }

    #[test]
    fn rejects_incompatible_fields_and_zero_divisors() {
        assert_eq!(
            element(1, 31).checked_add(element(1, 29)),
            Err(FieldElementError::MismatchedFields {
                left_prime: 31,
                right_prime: 29,
            })
        );
        assert_eq!(
            element(1, 31).checked_div(element(0, 31)),
            Err(FieldElementError::DivisionByZero)
        );
    }
}