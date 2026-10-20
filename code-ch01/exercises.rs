//! Runnable equivalents of the Chapter 1 notebook exercise cells.

use crate::answers;
use crate::ecc::FieldElementError;

/// Runs the Exercise 1 equality checks.
pub fn exercise_1() -> Result<[bool; 2], FieldElementError> {
    answers::answer_1()
}

/// Runs the Exercise 2 finite-field additions and subtractions.
pub fn exercise_2() -> [u64; 4] {
    answers::answer_2()
}

/// Runs the Exercise 3 finite-field subtractions.
pub fn exercise_3() -> Result<[u64; 2], FieldElementError> {
    answers::answer_3()
}

/// Runs the Exercise 4 finite-field multiplications and exponentiations.
pub fn exercise_4() -> Result<[u64; 3], FieldElementError> {
    answers::answer_4()
}

/// Runs the Exercise 5 finite-field multiplication sets.
pub fn exercise_5() -> [[u64; 19]; 5] {
    answers::answer_5()
}

/// Runs the Exercise 6 finite-field multiplication.
pub fn exercise_6() -> Result<u64, FieldElementError> {
    answers::answer_6()
}

/// Runs the Exercise 7 Fermat's little theorem calculations.
pub fn exercise_7() -> Result<Vec<Vec<u64>>, FieldElementError> {
    answers::answer_7()
}

/// Runs the Exercise 8 finite-field divisions and negative powers.
pub fn exercise_8() -> Result<[u64; 3], FieldElementError> {
    answers::answer_8()
}

/// Runs the Exercise 9 finite-field division.
pub fn exercise_9() -> Result<u64, FieldElementError> {
    answers::answer_9()
}

#[cfg(test)]
mod tests {
    use super::{
        exercise_1, exercise_2, exercise_3, exercise_4, exercise_5, exercise_6, exercise_7,
        exercise_8, exercise_9,
    };

    #[test]
    fn runs_the_inequality_exercise() {
        assert_eq!(exercise_1().unwrap(), [true, true]);
    }

    #[test]
    fn runs_the_scalar_exercises() {
        assert_eq!(exercise_2(), [20, 37, 51, 41]);
        assert_eq!(exercise_3().unwrap(), [25, 16]);
        assert_eq!(exercise_4().unwrap(), [23, 68, 63]);
        assert_eq!(exercise_6().unwrap(), 22);
        assert_eq!(exercise_8().unwrap(), [4, 29, 13]);
        assert_eq!(exercise_9().unwrap(), 4);
    }

    #[test]
    fn runs_the_set_and_fermat_exercises() {
        let expected_values: [u64; 19] = std::array::from_fn(|value| value as u64);
        assert!(exercise_5().iter().all(|values| {
            let mut values = *values;
            values.sort_unstable();
            values == expected_values
        }));
        assert!(exercise_7()
            .unwrap()
            .iter()
            .flatten()
            .all(|&value| value == 1));
    }
}
