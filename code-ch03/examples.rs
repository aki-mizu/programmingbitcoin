//! Runnable versions of the Chapter 3 examples.

use crate::ecc::{
    generator, parse_hex, secp256k1_order, secp256k1_prime, EccError, FieldElement, Point,
    PointError, PrivateKey, S256Point, Signature,
};
use crate::helper::hash256;
use num_bigint::BigUint;

fn field_223(value: u64) -> Result<FieldElement, PointError> {
    Ok(FieldElement::new(value, 223)?)
}

fn point_223(x: u64, y: u64) -> Result<Point, PointError> {
    Point::new(
        Some(field_223(x)?),
        Some(field_223(y)?),
        field_223(0)?,
        field_223(7)?,
    )
}

// tag::example1[]
/// Constructs a finite-field elliptic-curve point.
pub fn example_1() -> Result<Point, PointError> {
    point_223(192, 105)
}
// end::example1[]

// tag::example3[]
/// Adds two finite-field elliptic-curve points.
pub fn example_3() -> Result<Point, PointError> {
    point_223(192, 105)?.checked_add(&point_223(17, 56)?)
}
// end::example3[]

// tag::example4[]
/// Returns scalar multiples 1 through 20 of (47,71) over F_223.
pub fn example_4() -> Result<Vec<Point>, PointError> {
    let point = point_223(47, 71)?;
    (1_u8..=20)
        .map(|coefficient| point.scalar_mul(&BigUint::from(coefficient)))
        .collect()
}
// end::example4[]

// tag::example5[]
/// Multiplies the Exercise 5 point by the order of its generated group.
pub fn example_5() -> Result<Point, PointError> {
    point_223(15, 86)?.scalar_mul(&BigUint::from(7_u8))
}
// end::example5[]

// tag::example6[]
/// Verifies that the secp256k1 generator coordinates satisfy the curve equation.
pub fn example_6() -> Result<bool, EccError> {
    let prime = secp256k1_prime();
    let x = parse_hex("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798")?;
    let y = parse_hex("483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8")?;
    let left = y.modpow(&BigUint::from(2_u8), &prime);
    let right = (x.modpow(&BigUint::from(3_u8), &prime) + BigUint::from(7_u8)) % prime;

    Ok(left == right)
}
// end::example6[]

// tag::example7[]
/// Verifies that multiplying the generator by N produces infinity.
pub fn example_7() -> Result<bool, EccError> {
    Ok(generator()?.scalar_mul(&secp256k1_order())?.is_infinity())
}
// end::example7[]

// tag::example8[]
/// Verifies the generator's order through the secp256k1-specific API.
pub fn example_8() -> Result<bool, EccError> {
    Ok(generator()?.scalar_mul(&secp256k1_order())?.is_infinity())
}
// end::example8[]

// tag::example9[]
/// Verifies the chapter's ECDSA signature example.
pub fn example_9() -> Result<bool, EccError> {
    let z = parse_hex("bc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74423")?;
    let signature = Signature::new(
        parse_hex("37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6")?,
        parse_hex("8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec")?,
    );
    let point = S256Point::new(
        Some(parse_hex(
            "04519fac3d910ca7e7138f7013706f619fa8f033e6ec6e09370ea38cee6a7574",
        )?),
        Some(parse_hex(
            "82b51eab8c27c66e26c858a079bcdf4f1ada34cec420cafc7eac1a42216fb6c4",
        )?),
    )?;

    point.verify(&z, &signature)
}
// end::example9[]

// tag::example10[]
/// Signs the chapter's message example with deterministic ECDSA.
pub fn example_10() -> Result<(S256Point, BigUint, Signature), EccError> {
    let private_key = PrivateKey::new(BigUint::from_bytes_be(&hash256(b"my secret")))?;
    let z = BigUint::from_bytes_be(&hash256(b"my message"));
    let signature = private_key.sign(&z)?;

    Ok((private_key.point().clone(), z, signature))
}
// end::example10[]

#[cfg(test)]
mod tests {
    use super::{
        example_1, example_10, example_3, example_4, example_5, example_6, example_7, example_8,
        example_9,
    };

    #[test]
    fn reproduces_the_finite_field_examples() {
        assert_eq!(
            example_1().unwrap().to_string(),
            "Point(192,105)_0_7 FieldElement(223)"
        );
        assert_eq!(
            example_3().unwrap().to_string(),
            "Point(170,142)_0_7 FieldElement(223)"
        );
        let products = example_4().unwrap();
        assert_eq!(products.len(), 20);
        assert_eq!(
            products[0].to_string(),
            "Point(47,71)_0_7 FieldElement(223)"
        );
        assert_eq!(
            products[19].to_string(),
            "Point(47,152)_0_7 FieldElement(223)"
        );
        assert!(example_5().unwrap().is_infinity());
    }

    #[test]
    fn reproduces_the_secp256k1_examples() {
        assert!(example_6().unwrap());
        assert!(example_7().unwrap());
        assert!(example_8().unwrap());
        assert!(example_9().unwrap());
        let (point, z, signature) = example_10().unwrap();
        assert!(point.verify(&z, &signature).unwrap());
    }
}
