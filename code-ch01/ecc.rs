//! Chapter 1: arbitrary-precision finite-field arithmetic.

use num_bigint::{BigInt, BigUint, Sign};
use num_traits::{One, Zero};
use std::error::Error;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

// tag::source1[]
/// An error returned when finite-field arithmetic cannot be performed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldElementError {
    InvalidPrime {
        prime: BigUint,
    },
    ValueOutOfRange {
        value: BigUint,
        prime: BigUint,
    },
    MismatchedFields {
        left_prime: BigUint,
        right_prime: BigUint,
    },
    DivisionByZero,
}

/// An element of the prime finite field $F_p$.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FieldElement {
    value: BigUint,
    prime: BigUint,
}

impl FieldElement {
    /// Creates an element from Chapter 1's small integer values.
    pub fn new(value: u64, prime: u64) -> Result<Self, FieldElementError> {
        Self::from_biguint(BigUint::from(value), BigUint::from(prime))
    }

    /// Creates an element with arbitrary-precision value and field order.
    pub fn from_biguint(value: BigUint, prime: BigUint) -> Result<Self, FieldElementError> {
        if prime < BigUint::from(2_u8) {
            return Err(FieldElementError::InvalidPrime { prime });
        }
        if value >= prime {
            return Err(FieldElementError::ValueOutOfRange { value, prime });
        }

        Ok(Self { value, prime })
    }
    // end::source1[]

    /// Returns this element's canonical value, in the range $0..p$.
    pub const fn value(&self) -> &BigUint {
        &self.value
    }

    /// Returns the order of the field containing this element.
    pub const fn prime(&self) -> &BigUint {
        &self.prime
    }

    /// Returns whether this element is zero.
    pub fn is_zero(&self) -> bool {
        self.value.is_zero()
    }

    // tag::source2[]
    /// Adds two elements from the same field.
    pub fn checked_add(&self, other: &Self) -> Result<Self, FieldElementError> {
        self.ensure_same_field(other)?; // <1>
        Ok(Self::from_reduced(
            (&self.value + &other.value) % &self.prime, // <2>
            self.prime.clone(),                         // <3>
        ))
    }
    // end::source2[]

    /// Subtracts two elements from the same field.
    pub fn checked_sub(&self, other: &Self) -> Result<Self, FieldElementError> {
        self.ensure_same_field(other)?;
        let value = if self.value >= other.value {
            &self.value - &other.value
        } else {
            &self.value + &self.prime - &other.value
        };

        Ok(Self::from_reduced(value, self.prime.clone()))
    }

    /// Multiplies two elements from the same field.
    pub fn checked_mul(&self, other: &Self) -> Result<Self, FieldElementError> {
        self.ensure_same_field(other)?;
        Ok(Self::from_reduced(
            (&self.value * &other.value) % &self.prime,
            self.prime.clone(),
        ))
    }

    /// Multiplies this element by an integer and reduces it in this field.
    pub fn checked_scalar_mul(&self, coefficient: &BigUint) -> Self {
        Self::from_reduced(
            (&self.value * coefficient) % &self.prime,
            self.prime.clone(),
        )
    }

    // tag::source3[]
    /// Raises this element to an integer power, including negative powers.
    pub fn pow(&self, exponent: i128) -> Self {
        let group_order = &self.prime - BigUint::one();
        let group_order_signed = BigInt::from_biguint(Sign::Plus, group_order.clone());
        let mut reduced = BigInt::from(exponent) % &group_order_signed;
        if reduced.sign() == Sign::Minus {
            reduced += group_order_signed;
        }
        let reduced = reduced
            .to_biguint()
            .expect("the reduced finite-field exponent is non-negative");

        self.pow_biguint(&reduced)
    }
    // end::source3[]

    /// Raises this element to an arbitrary-precision non-negative power.
    pub fn pow_biguint(&self, exponent: &BigUint) -> Self {
        Self::from_reduced(self.value.modpow(exponent, &self.prime), self.prime.clone())
    }

    /// Divides two elements from the same field using Fermat's little theorem.
    pub fn checked_div(&self, other: &Self) -> Result<Self, FieldElementError> {
        self.ensure_same_field(other)?;
        if other.is_zero() {
            return Err(FieldElementError::DivisionByZero);
        }

        let exponent = &self.prime - BigUint::from(2_u8);
        self.checked_mul(&other.pow_biguint(&exponent))
    }

    /// Returns an error unless the two elements belong to the same field.
    pub fn ensure_same_field(&self, other: &Self) -> Result<(), FieldElementError> {
        if self.prime == other.prime {
            Ok(())
        } else {
            Err(FieldElementError::MismatchedFields {
                left_prime: self.prime.clone(),
                right_prime: other.prime.clone(),
            })
        }
    }

    fn from_reduced(value: BigUint, prime: BigUint) -> Self {
        Self { value, prime }
    }
}

impl fmt::Display for FieldElementError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPrime { prime } => {
                write!(formatter, "field prime must be at least 2, got {prime}")
            }
            Self::ValueOutOfRange { value, prime } => {
                write!(
                    formatter,
                    "value {value} is not in field range 0 to {}",
                    prime - BigUint::one()
                )
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
        self.checked_add(&other)
            .expect("cannot add elements from different finite fields")
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self.checked_sub(&other)
            .expect("cannot subtract elements from different finite fields")
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        self.checked_mul(&other)
            .expect("cannot multiply elements from different finite fields")
    }
}

impl Div for FieldElement {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        self.checked_div(&other)
            .expect("cannot divide elements from different finite fields or by zero")
    }
}

#[cfg(test)]
mod tests {
    use super::{FieldElement, FieldElementError};
    use num_bigint::BigUint;
    use num_traits::{One, Zero};

    fn element(value: u64, prime: u64) -> FieldElement {
        FieldElement::new(value, prime).unwrap()
    }

    #[test]
    fn rejects_values_outside_the_field() {
        assert_eq!(
            FieldElement::new(31, 31),
            Err(FieldElementError::ValueOutOfRange {
                value: BigUint::from(31_u8),
                prime: BigUint::from(31_u8),
            })
        );
        assert_eq!(
            FieldElement::new(0, 1),
            Err(FieldElementError::InvalidPrime {
                prime: BigUint::one(),
            })
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
        assert_eq!(
            element(2, 31).checked_add(&element(15, 31)).unwrap(),
            element(17, 31)
        );
        assert_eq!(
            element(17, 31).checked_add(&element(21, 31)).unwrap(),
            element(7, 31)
        );
    }

    #[test]
    fn subtracts_elements_modulo_the_field_order() {
        assert_eq!(
            element(29, 31).checked_sub(&element(4, 31)).unwrap(),
            element(25, 31)
        );
        assert_eq!(
            element(15, 31).checked_sub(&element(30, 31)).unwrap(),
            element(16, 31)
        );
    }

    #[test]
    fn multiplies_elements_modulo_the_field_order() {
        assert_eq!(
            element(24, 31).checked_mul(&element(19, 31)).unwrap(),
            element(22, 31)
        );
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
        assert_eq!(
            element(3, 31).checked_div(&element(24, 31)).unwrap(),
            element(4, 31)
        );
    }

    #[test]
    fn rejects_incompatible_fields_and_zero_divisors() {
        assert_eq!(
            element(1, 31).checked_add(&element(1, 29)),
            Err(FieldElementError::MismatchedFields {
                left_prime: BigUint::from(31_u8),
                right_prime: BigUint::from(29_u8),
            })
        );
        assert_eq!(
            element(1, 31).checked_div(&element(0, 31)),
            Err(FieldElementError::DivisionByZero)
        );
    }

    #[test]
    fn supports_secp256k1_sized_fields() {
        let prime =
            (BigUint::one() << 256_usize) - (BigUint::one() << 32_usize) - BigUint::from(977_u16);
        let almost_prime =
            FieldElement::from_biguint(&prime - BigUint::one(), prime.clone()).unwrap();
        let one = FieldElement::from_biguint(BigUint::one(), prime.clone()).unwrap();

        assert_eq!(
            almost_prime.checked_add(&one).unwrap().value(),
            &BigUint::zero()
        );
        assert_eq!(
            FieldElement::from_biguint(BigUint::from(3_u8), prime.clone())
                .unwrap()
                .checked_mul(
                    &FieldElement::from_biguint(BigUint::from(7_u8), prime.clone()).unwrap()
                )
                .unwrap()
                .value(),
            &BigUint::from(21_u8)
        );
    }
}
