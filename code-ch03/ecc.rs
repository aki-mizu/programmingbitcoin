//! Chapter 3: finite-field elliptic curves and secp256k1 signatures.

use hmac::{Hmac, Mac};
use num_bigint::BigUint;
use num_traits::{One, Zero};
pub use programmingbitcoin_ch01::ecc::{FieldElement, FieldElementError};
use sha2::Sha256;
use std::error::Error;
use std::fmt;

type HmacSha256 = Hmac<Sha256>;

/// An error returned when an elliptic-curve point cannot be constructed or combined.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PointError {
    CoordinatesMustBothBePresentOrAbsent,
    PointNotOnCurve {
        x: Box<FieldElement>,
        y: Box<FieldElement>,
        a: Box<FieldElement>,
        b: Box<FieldElement>,
    },
    MismatchedCurves {
        left_a: Box<FieldElement>,
        left_b: Box<FieldElement>,
        right_a: Box<FieldElement>,
        right_b: Box<FieldElement>,
    },
    Field(Box<FieldElementError>),
}

impl fmt::Display for PointError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CoordinatesMustBothBePresentOrAbsent => {
                write!(formatter, "point coordinates must both be present or both be absent")
            }
            Self::PointNotOnCurve { x, y, a, b } => write!(
                formatter,
                "({x}, {y}) is not on the curve y^2 = x^3 + {a}x + {b}"
            ),
            Self::MismatchedCurves {
                left_a,
                left_b,
                right_a,
                right_b,
            } => write!(
                formatter,
                "cannot add points on y^2 = x^3 + {left_a}x + {left_b} and y^2 = x^3 + {right_a}x + {right_b}"
            ),
            Self::Field(error) => error.fmt(formatter),
        }
    }
}

impl Error for PointError {}

impl From<FieldElementError> for PointError {
    fn from(error: FieldElementError) -> Self {
        Self::Field(Box::new(error))
    }
}

// tag::source1[]
/// A point on the finite-field elliptic curve $y^2 = x^3 + ax + b$.
#[derive(Clone, PartialEq, Eq)]
pub struct Point {
    x: Option<FieldElement>,
    y: Option<FieldElement>,
    a: FieldElement,
    b: FieldElement,
}

impl Point {
    /// Creates a finite point or the point at infinity when both coordinates are absent.
    pub fn new(
        x: Option<FieldElement>,
        y: Option<FieldElement>,
        a: FieldElement,
        b: FieldElement,
    ) -> Result<Self, PointError> {
        a.ensure_same_field(&b)?;
        match (x, y) {
            (None, None) => Ok(Self {
                x: None,
                y: None,
                a,
                b,
            }),
            (Some(x), Some(y)) => {
                Self::validate_finite_coordinates(&x, &y, &a, &b)?;
                Ok(Self {
                    x: Some(x),
                    y: Some(y),
                    a,
                    b,
                })
            }
            _ => Err(PointError::CoordinatesMustBothBePresentOrAbsent),
        }
    }
    // end::source1[]

    fn validate_finite_coordinates(
        x: &FieldElement,
        y: &FieldElement,
        a: &FieldElement,
        b: &FieldElement,
    ) -> Result<(), PointError> {
        a.ensure_same_field(x)?;
        a.ensure_same_field(y)?;

        let left = y.pow_biguint(&BigUint::from(2_u8));
        let right = x
            .pow_biguint(&BigUint::from(3_u8))
            .checked_add(&a.checked_mul(x)?)?
            .checked_add(b)?;
        if left == right {
            Ok(())
        } else {
            Err(PointError::PointNotOnCurve {
                x: Box::new(x.clone()),
                y: Box::new(y.clone()),
                a: Box::new(a.clone()),
                b: Box::new(b.clone()),
            })
        }
    }

    /// Creates the additive identity for the curve $y^2 = x^3 + ax + b$.
    pub fn infinity(a: FieldElement, b: FieldElement) -> Result<Self, PointError> {
        Self::new(None, None, a, b)
    }

    /// Returns the x coordinate, or `None` for the point at infinity.
    pub const fn x(&self) -> Option<&FieldElement> {
        self.x.as_ref()
    }

    /// Returns the y coordinate, or `None` for the point at infinity.
    pub const fn y(&self) -> Option<&FieldElement> {
        self.y.as_ref()
    }

    /// Returns the curve's a coefficient.
    pub const fn a(&self) -> &FieldElement {
        &self.a
    }

    /// Returns the curve's b coefficient.
    pub const fn b(&self) -> &FieldElement {
        &self.b
    }

    /// Returns whether this point is the additive identity.
    pub fn is_infinity(&self) -> bool {
        self.x.is_none() && self.y.is_none()
    }

    /// Adds two points on the same finite-field elliptic curve.
    pub fn checked_add(&self, other: &Self) -> Result<Self, PointError> {
        self.ensure_same_curve(other)?;
        if self.is_infinity() {
            return Ok(other.clone());
        }
        if other.is_infinity() {
            return Ok(self.clone());
        }

        let (Some(x1), Some(y1), Some(x2), Some(y2)) = (self.x(), self.y(), other.x(), other.y())
        else {
            return Err(PointError::CoordinatesMustBothBePresentOrAbsent);
        };

        if x1 == x2 && y1 != y2 {
            return Self::infinity(self.a.clone(), self.b.clone());
        }

        if x1 != x2 {
            let slope = y2.checked_sub(y1)?.checked_div(&x2.checked_sub(x1)?)?;
            let x3 = slope
                .pow_biguint(&BigUint::from(2_u8))
                .checked_sub(x1)?
                .checked_sub(x2)?;
            let y3 = slope.checked_mul(&x1.checked_sub(&x3)?)?.checked_sub(y1)?;
            return Self::new(Some(x3), Some(y3), self.a.clone(), self.b.clone());
        }

        if y1.is_zero() {
            return Self::infinity(self.a.clone(), self.b.clone());
        }

        let slope = x1
            .pow_biguint(&BigUint::from(2_u8))
            .checked_scalar_mul(&BigUint::from(3_u8))
            .checked_add(&self.a)?
            .checked_div(&y1.checked_scalar_mul(&BigUint::from(2_u8)))?;
        let x3 = slope
            .pow_biguint(&BigUint::from(2_u8))
            .checked_sub(&x1.checked_scalar_mul(&BigUint::from(2_u8)))?;
        let y3 = slope.checked_mul(&x1.checked_sub(&x3)?)?.checked_sub(y1)?;
        Self::new(Some(x3), Some(y3), self.a.clone(), self.b.clone())
    }

    // tag::source3[]
    /// Multiplies this point by a non-negative scalar with binary expansion.
    pub fn scalar_mul(&self, coefficient: &BigUint) -> Result<Self, PointError> {
        let mut coefficient = coefficient.clone();
        let mut current = self.clone();
        let mut result = Self::infinity(self.a.clone(), self.b.clone())?;

        while !coefficient.is_zero() {
            if (&coefficient % BigUint::from(2_u8)).is_one() {
                result = result.checked_add(&current)?;
            }
            coefficient >>= 1_usize;
            if !coefficient.is_zero() {
                current = current.checked_add(&current)?;
            }
        }

        Ok(result)
    }
    // end::source3[]

    fn ensure_same_curve(&self, other: &Self) -> Result<(), PointError> {
        if self.a == other.a && self.b == other.b {
            Ok(())
        } else {
            Err(PointError::MismatchedCurves {
                left_a: Box::new(self.a.clone()),
                left_b: Box::new(self.b.clone()),
                right_a: Box::new(other.a.clone()),
                right_b: Box::new(other.b.clone()),
            })
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.x(), self.y()) {
            (Some(x), Some(y)) => write!(
                formatter,
                "Point({},{})_{}_{} FieldElement({})",
                x.value(),
                y.value(),
                self.a.value(),
                self.b.value(),
                x.prime()
            ),
            _ => write!(formatter, "Point(infinity)"),
        }
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, formatter)
    }
}

/// An error returned by secp256k1 parsing, signing, or verification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EccError {
    Point(Box<PointError>),
    InvalidHex { value: String },
    InvalidSecret { secret: Box<BigUint> },
    InvalidSignature { r: Box<BigUint>, s: Box<BigUint> },
    InvalidNonce,
    PointAtInfinity,
    ScalarTooWide { bits: u64 },
    HmacInitialization,
}

impl fmt::Display for EccError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Point(error) => error.fmt(formatter),
            Self::InvalidHex { value } => write!(formatter, "invalid hexadecimal integer: {value}"),
            Self::InvalidSecret { secret } => {
                write!(formatter, "private key must be in 1..N, got {secret}")
            }
            Self::InvalidSignature { r, s } => {
                write!(formatter, "signature values must be in 1..N: r={r}, s={s}")
            }
            Self::InvalidNonce => write!(
                formatter,
                "deterministic nonce produced an invalid signature"
            ),
            Self::PointAtInfinity => write!(formatter, "operation unexpectedly produced infinity"),
            Self::ScalarTooWide { bits } => {
                write!(formatter, "scalar must fit in 256 bits, got {bits} bits")
            }
            Self::HmacInitialization => write!(formatter, "could not initialize HMAC-SHA256"),
        }
    }
}

impl Error for EccError {}

impl From<PointError> for EccError {
    fn from(error: PointError) -> Self {
        Self::Point(Box::new(error))
    }
}

/// Parses a non-prefixed or `0x`-prefixed hexadecimal integer.
pub fn parse_hex(value: &str) -> Result<BigUint, EccError> {
    let digits = value.strip_prefix("0x").unwrap_or(value);
    BigUint::parse_bytes(digits.as_bytes(), 16).ok_or_else(|| EccError::InvalidHex {
        value: value.to_owned(),
    })
}

// tag::source4[]
/// Returns the secp256k1 prime $2^{256} - 2^{32} - 977$.
pub fn secp256k1_prime() -> BigUint {
    (BigUint::one() << 256_usize) - (BigUint::one() << 32_usize) - BigUint::from(977_u16)
}
// end::source4[]

// tag::source9[]
/// Returns the order of the secp256k1 generator point.
pub fn secp256k1_order() -> BigUint {
    BigUint::parse_bytes(
        b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
        16,
    )
    .expect("the hard-coded secp256k1 order is valid hexadecimal")
}
// end::source9[]

// tag::source5[]
/// A field element in secp256k1's fixed prime field.
#[derive(Clone, PartialEq, Eq)]
pub struct S256Field(FieldElement);

impl S256Field {
    /// Creates a secp256k1 field element.
    pub fn new(value: BigUint) -> Result<Self, FieldElementError> {
        Ok(Self(FieldElement::from_biguint(value, secp256k1_prime())?))
    }

    /// Returns the field element's canonical value.
    pub const fn value(&self) -> &BigUint {
        self.0.value()
    }

    /// Returns the underlying generic field element.
    pub const fn as_field_element(&self) -> &FieldElement {
        &self.0
    }
}

impl fmt::Display for S256Field {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:064x}", self.value())
    }
}

impl fmt::Debug for S256Field {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, formatter)
    }
}
// end::source5[]

// tag::source6[]
/// Returns the secp256k1 curve coefficient a.
pub fn secp256k1_a() -> BigUint {
    BigUint::zero()
}

/// Returns the secp256k1 curve coefficient b.
pub fn secp256k1_b() -> BigUint {
    BigUint::from(7_u8)
}
// end::source6[]

/// A point on the secp256k1 curve.
#[derive(Clone, PartialEq, Eq)]
pub struct S256Point {
    point: Point,
}

impl S256Point {
    // tag::source7[]
    /// Creates a finite secp256k1 point or the point at infinity.
    pub fn new(x: Option<BigUint>, y: Option<BigUint>) -> Result<Self, PointError> {
        let prime = secp256k1_prime();
        let a = FieldElement::from_biguint(secp256k1_a(), prime.clone())?;
        let b = FieldElement::from_biguint(secp256k1_b(), prime.clone())?;
        let x = x
            .map(|value| FieldElement::from_biguint(value, prime.clone()))
            .transpose()?;
        let y = y
            .map(|value| FieldElement::from_biguint(value, prime.clone()))
            .transpose()?;

        Ok(Self {
            point: Point::new(x, y, a, b)?,
        })
    }
    // end::source7[]

    /// Returns the secp256k1 x coordinate, or `None` for infinity.
    pub fn x(&self) -> Option<&BigUint> {
        self.point.x().map(FieldElement::value)
    }

    /// Returns the secp256k1 y coordinate, or `None` for infinity.
    pub fn y(&self) -> Option<&BigUint> {
        self.point.y().map(FieldElement::value)
    }

    /// Returns whether this is the point at infinity.
    pub fn is_infinity(&self) -> bool {
        self.point.is_infinity()
    }

    /// Adds two secp256k1 points.
    pub fn checked_add(&self, other: &Self) -> Result<Self, PointError> {
        Ok(Self {
            point: self.point.checked_add(&other.point)?,
        })
    }

    // tag::source8[]
    /// Multiplies this secp256k1 point by a scalar reduced modulo the group order.
    pub fn scalar_mul(&self, coefficient: &BigUint) -> Result<Self, PointError> {
        let coefficient = coefficient % secp256k1_order();
        Ok(Self {
            point: self.point.scalar_mul(&coefficient)?,
        })
    }
    // end::source8[]

    // tag::source12[]
    /// Verifies an ECDSA signature for the supplied signature hash.
    pub fn verify(&self, z: &BigUint, signature: &Signature) -> Result<bool, EccError> {
        let order = secp256k1_order();
        if signature.r.is_zero()
            || signature.s.is_zero()
            || signature.r >= order
            || signature.s >= order
        {
            return Err(EccError::InvalidSignature {
                r: Box::new(signature.r.clone()),
                s: Box::new(signature.s.clone()),
            });
        }

        let inverse = signature.s.modpow(&(&order - BigUint::from(2_u8)), &order);
        let u = ((z % &order) * &inverse) % &order;
        let v = (&signature.r * inverse) % &order;
        let total = generator()?
            .scalar_mul(&u)?
            .checked_add(&self.scalar_mul(&v)?)?;

        Ok(total.x().is_some_and(|x| x == &signature.r))
    }
    // end::source12[]
}

impl fmt::Display for S256Point {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.x(), self.y()) {
            (Some(x), Some(y)) => write!(formatter, "S256Point({x:064x},{y:064x})"),
            _ => write!(formatter, "S256Point(infinity)"),
        }
    }
}

impl fmt::Debug for S256Point {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, formatter)
    }
}

// tag::source10[]
/// Returns secp256k1's generator point G.
pub fn generator() -> Result<S256Point, PointError> {
    S256Point::new(
        Some(
            BigUint::parse_bytes(
                b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
                16,
            )
            .expect("the hard-coded secp256k1 generator x coordinate is valid hexadecimal"),
        ),
        Some(
            BigUint::parse_bytes(
                b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
                16,
            )
            .expect("the hard-coded secp256k1 generator y coordinate is valid hexadecimal"),
        ),
    )
}
// end::source10[]

// tag::source11[]
/// An ECDSA signature's r and s scalar values.
#[derive(Clone, PartialEq, Eq)]
pub struct Signature {
    r: BigUint,
    s: BigUint,
}

impl Signature {
    /// Creates a signature from its two scalar values.
    pub const fn new(r: BigUint, s: BigUint) -> Self {
        Self { r, s }
    }

    /// Returns the signature's r value.
    pub const fn r(&self) -> &BigUint {
        &self.r
    }

    /// Returns the signature's s value.
    pub const fn s(&self) -> &BigUint {
        &self.s
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Signature({:x},{:x})", self.r, self.s)
    }
}

impl fmt::Debug for Signature {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, formatter)
    }
}
// end::source11[]

// tag::source13[]
/// A secp256k1 private key and its derived public point.
#[derive(Clone, PartialEq, Eq)]
pub struct PrivateKey {
    secret: BigUint,
    point: S256Point,
}

impl PrivateKey {
    /// Creates a valid private key in the range 1 through N - 1.
    pub fn new(secret: BigUint) -> Result<Self, EccError> {
        let order = secp256k1_order();
        if secret.is_zero() || secret >= order {
            return Err(EccError::InvalidSecret {
                secret: Box::new(secret),
            });
        }
        let point = generator()?.scalar_mul(&secret)?;

        Ok(Self { secret, point })
    }
    // end::source13[]

    /// Returns the private scalar.
    pub const fn secret(&self) -> &BigUint {
        &self.secret
    }

    /// Returns the derived public point.
    pub const fn point(&self) -> &S256Point {
        &self.point
    }

    /// Returns the private scalar as a 64-character hexadecimal string.
    pub fn hex(&self) -> String {
        format!("{:064x}", self.secret)
    }

    // tag::source14[]
    /// Signs a signature hash with RFC 6979 deterministic nonce generation.
    pub fn sign(&self, z: &BigUint) -> Result<Signature, EccError> {
        let order = secp256k1_order();
        let k = self.deterministic_k(z)?;
        let point = generator()?.scalar_mul(&k)?;
        let r = point.x().ok_or(EccError::PointAtInfinity)?.clone() % &order;
        if r.is_zero() {
            return Err(EccError::InvalidNonce);
        }

        let inverse = k.modpow(&(&order - BigUint::from(2_u8)), &order);
        let numerator = ((z % &order) + (&r * &self.secret)) % &order;
        let mut s = (numerator * inverse) % &order;
        if s.is_zero() {
            return Err(EccError::InvalidNonce);
        }
        if s > (&order >> 1_usize) {
            s = &order - s;
        }

        Ok(Signature::new(r, s))
    }

    /// Derives an RFC 6979 nonce from this private key and signature hash.
    pub fn deterministic_k(&self, z: &BigUint) -> Result<BigUint, EccError> {
        let z = z % secp256k1_order();
        let secret = to_32_bytes(&self.secret)?;
        let z = to_32_bytes(&z)?;
        let mut k = [0_u8; 32];
        let mut v = [1_u8; 32];

        let mut input = Vec::with_capacity(97);
        input.extend_from_slice(&v);
        input.push(0);
        input.extend_from_slice(&secret);
        input.extend_from_slice(&z);
        k = hmac_sha256(&k, &input)?;
        v = hmac_sha256(&k, &v)?;

        input.clear();
        input.extend_from_slice(&v);
        input.push(1);
        input.extend_from_slice(&secret);
        input.extend_from_slice(&z);
        k = hmac_sha256(&k, &input)?;
        v = hmac_sha256(&k, &v)?;

        loop {
            v = hmac_sha256(&k, &v)?;
            let candidate = BigUint::from_bytes_be(&v);
            if !candidate.is_zero() && candidate < secp256k1_order() {
                return Ok(candidate);
            }

            input.clear();
            input.extend_from_slice(&v);
            input.push(0);
            k = hmac_sha256(&k, &input)?;
            v = hmac_sha256(&k, &v)?;
        }
    }
    // end::source14[]
}

fn hmac_sha256(key: &[u8], value: &[u8]) -> Result<[u8; 32], EccError> {
    let mut mac = HmacSha256::new_from_slice(key).map_err(|_| EccError::HmacInitialization)?;
    mac.update(value);
    let bytes = mac.finalize().into_bytes();
    let mut result = [0_u8; 32];
    result.copy_from_slice(&bytes);
    Ok(result)
}

fn to_32_bytes(value: &BigUint) -> Result<[u8; 32], EccError> {
    let bytes = value.to_bytes_be();
    if bytes.len() > 32 {
        return Err(EccError::ScalarTooWide { bits: value.bits() });
    }

    let mut result = [0_u8; 32];
    result[32 - bytes.len()..].copy_from_slice(&bytes);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::{
        generator, parse_hex, secp256k1_order, secp256k1_prime, EccError, FieldElement, Point,
        PointError, PrivateKey, S256Field, S256Point, Signature,
    };
    use crate::helper::hash256;
    use num_bigint::BigUint;
    use num_traits::{One, Zero};

    type ScalarMultiplication = (u64, u64, u64, Option<(u64, u64)>);

    fn field(value: u64, prime: u64) -> FieldElement {
        FieldElement::new(value, prime).unwrap()
    }

    fn point_223(x: u64, y: u64) -> Point {
        Point::new(
            Some(field(x, 223)),
            Some(field(y, 223)),
            field(0, 223),
            field(7, 223),
        )
        .unwrap()
    }

    // tag::source2[]
    #[test]
    fn verifies_points_on_a_finite_field_curve() {
        for (x, y) in [(192, 105), (17, 56), (1, 193)] {
            assert!(Point::new(
                Some(field(x, 223)),
                Some(field(y, 223)),
                field(0, 223),
                field(7, 223),
            )
            .is_ok());
        }
        for (x, y) in [(200, 119), (42, 99)] {
            assert!(matches!(
                Point::new(
                    Some(field(x, 223)),
                    Some(field(y, 223)),
                    field(0, 223),
                    field(7, 223),
                ),
                Err(PointError::PointNotOnCurve { .. })
            ));
        }
    }
    // end::source2[]

    #[test]
    fn adds_and_multiplies_finite_field_points() {
        let additions = [
            (192, 105, 17, 56, 170, 142),
            (47, 71, 117, 141, 60, 139),
            (143, 98, 76, 66, 47, 71),
        ];
        for (x1, y1, x2, y2, x3, y3) in additions {
            assert_eq!(
                point_223(x1, y1).checked_add(&point_223(x2, y2)).unwrap(),
                point_223(x3, y3)
            );
        }

        let multiplications: [ScalarMultiplication; 6] = [
            (2, 192, 105, Some((49, 71))),
            (2, 143, 98, Some((64, 168))),
            (2, 47, 71, Some((36, 111))),
            (4, 47, 71, Some((194, 51))),
            (8, 47, 71, Some((116, 55))),
            (21, 47, 71, None),
        ];
        for (coefficient, x, y, expected) in multiplications {
            let product = point_223(x, y)
                .scalar_mul(&BigUint::from(coefficient))
                .unwrap();
            match expected {
                Some((x, y)) => assert_eq!(product, point_223(x, y)),
                None => assert!(product.is_infinity()),
            }
        }
    }

    #[test]
    fn initializes_and_formats_secp256k1_values() {
        assert_eq!(
            secp256k1_prime(),
            parse_hex("fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f").unwrap()
        );
        assert_eq!(
            format!("{}", S256Field::new(BigUint::from(7_u8)).unwrap()),
            format!("{:064x}", 7)
        );
        assert!(generator().is_ok());
        assert!(S256Point::new(Some(BigUint::zero()), None).is_err());
    }

    #[test]
    fn derives_known_public_points_and_the_generator_order() {
        let points = [
            (
                BigUint::from(7_u8),
                "5cbdf0646e5db4eaa398f365f2ea7a0e3d419b7e0330e39ce92bddedcac4f9bc",
                "6aebca40ba255960a3178d6d861a54dba813d0b813fde7b5a5082628087264da",
            ),
            (
                BigUint::from(1485_u16),
                "c982196a7466fbbbb0e27a940b6af926c1a74d5ad07128c82824a11b5398afda",
                "7a91f9eae64438afb9ce6448a1c133db2d8fb9254e4546b6f001637d50901f55",
            ),
            (
                BigUint::from(1_u8) << 128_usize,
                "8f68b9d2f63b5f339239c1ad981f162ee88c5678723ea3351b7b444c9ec4c0da",
                "662a9f2dba063986de1d90c2b6be215dbbea2cfe95510bfdf23cbf79501fff82",
            ),
        ];
        for (secret, x, y) in points {
            assert_eq!(
                generator().unwrap().scalar_mul(&secret).unwrap(),
                S256Point::new(Some(parse_hex(x).unwrap()), Some(parse_hex(y).unwrap())).unwrap()
            );
        }
        assert!(generator()
            .unwrap()
            .scalar_mul(&secp256k1_order())
            .unwrap()
            .is_infinity());
    }

    #[test]
    fn verifies_the_chapter_signatures() {
        let point = S256Point::new(
            Some(
                parse_hex("887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c")
                    .unwrap(),
            ),
            Some(
                parse_hex("61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34")
                    .unwrap(),
            ),
        )
        .unwrap();
        let signatures = [
            (
                "ec208baa0fc1c19f708a9ca96fdeff3ac3f230bb4a7ba4aede4942ad003c0f60",
                "ac8d1c87e51d0d441be8b3dd5b05c8795b48875dffe00b7ffcfac23010d3a395",
                "68342ceff8935ededd102dd876ffd6ba72d6a427a3edb13d26eb0781cb423c4",
            ),
            (
                "7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d",
                "eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c",
                "c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6",
            ),
        ];
        for (z, r, s) in signatures {
            assert!(point
                .verify(
                    &parse_hex(z).unwrap(),
                    &Signature::new(parse_hex(r).unwrap(), parse_hex(s).unwrap()),
                )
                .unwrap());
        }
    }

    #[test]
    fn signs_deterministically_and_verifies_the_result() {
        let key = PrivateKey::new(BigUint::from(12_345_u16)).unwrap();
        let z = BigUint::from_bytes_be(&hash256(b"Programming Bitcoin!"));
        let first = key.sign(&z).unwrap();
        let second = key.sign(&z).unwrap();

        assert_eq!(first, second);
        assert!(key.point().verify(&z, &first).unwrap());
        assert_eq!(key.hex(), format!("{:064x}", 12_345));
    }

    #[test]
    fn rejects_invalid_private_keys_and_signatures() {
        assert!(matches!(
            PrivateKey::new(BigUint::zero()),
            Err(EccError::InvalidSecret { .. })
        ));
        assert!(matches!(
            PrivateKey::new(secp256k1_order()),
            Err(EccError::InvalidSecret { .. })
        ));
        assert!(matches!(
            generator().unwrap().verify(
                &BigUint::zero(),
                &Signature::new(BigUint::zero(), BigUint::one())
            ),
            Err(EccError::InvalidSignature { .. })
        ));
    }
}
