//! Runnable equivalents of the Chapter 3 notebook exercise cells.

use crate::answers;
use crate::ecc::{EccError, Point, PointError, Signature};

/// Runs the Exercise 1 finite-field curve-membership checks.
pub fn exercise_1() -> [bool; 5] {
    answers::answer_1()
}

/// Runs the Exercise 2 finite-field point additions.
pub fn exercise_2() -> Result<[Point; 3], PointError> {
    answers::answer_2()
}

/// Runs the Exercise 3 finite-field addition test.
pub fn exercise_3() -> Result<bool, PointError> {
    answers::answer_3()
}

/// Runs the Exercise 4 scalar-multiplication calculations.
pub fn exercise_4() -> Result<[Point; 6], PointError> {
    answers::answer_4()
}

/// Runs the Exercise 5 group-order calculation.
pub fn exercise_5() -> Result<usize, PointError> {
    answers::answer_5()
}

/// Runs the Exercise 6 signature-verification checks.
pub fn exercise_6() -> Result<[bool; 2], EccError> {
    answers::answer_6()
}

/// Runs the Exercise 7 deterministic signing calculation.
pub fn exercise_7() -> Result<Signature, EccError> {
    answers::answer_7()
}

#[cfg(test)]
mod tests {
    use super::{
        exercise_1, exercise_2, exercise_3, exercise_4, exercise_5, exercise_6, exercise_7,
    };
    use crate::ecc::{parse_hex, S256Point};
    use crate::helper::hash256;
    use num_bigint::BigUint;

    #[test]
    fn runs_the_finite_field_exercises() {
        assert_eq!(exercise_1(), [true, true, false, true, false]);
        assert_eq!(exercise_2().unwrap().len(), 3);
        assert!(exercise_3().unwrap());
        assert!(exercise_4().unwrap()[5].is_infinity());
        assert_eq!(exercise_5().unwrap(), 7);
    }

    #[test]
    fn runs_the_signature_exercises() {
        assert_eq!(exercise_6().unwrap(), [true, true]);
        let point = S256Point::new(
            Some(
                parse_hex("f01d6b9018ab421dd410404cb869072065522bf85734008f105cf385a023a80f")
                    .unwrap(),
            ),
            Some(
                parse_hex("0eba29d0f0c5408ed681984dc525982abefccd9f7ff01dd26da4999cf3f6a295")
                    .unwrap(),
            ),
        )
        .unwrap();
        assert!(point
            .verify(
                &BigUint::from_bytes_be(&hash256(b"Programming Bitcoin!")),
                &exercise_7().unwrap(),
            )
            .unwrap());
    }
}
