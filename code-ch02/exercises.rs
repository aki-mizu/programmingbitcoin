//! Runnable equivalents of the Chapter 2 notebook exercise cells.

use crate::answers;
use crate::ecc::{Point, PointError};

/// Runs the Exercise 1 curve-membership checks.
pub fn exercise_1() -> [bool; 4] {
    answers::answer_1()
}

/// Runs the Exercise 2 point-equality checks.
pub fn exercise_2() -> Result<[bool; 2], PointError> {
    answers::answer_2()
}

/// Runs the Exercise 3 additive-inverse calculation.
pub fn exercise_3() -> Result<Point, PointError> {
    answers::answer_3()
}

/// Runs the Exercise 4 point-addition calculation.
pub fn exercise_4() -> Result<Point, PointError> {
    answers::answer_4()
}

/// Runs the Exercise 5 distinct-point addition calculation.
pub fn exercise_5() -> Result<Point, PointError> {
    answers::answer_5()
}

/// Runs the Exercise 6 point-doubling calculation.
pub fn exercise_6() -> Result<Point, PointError> {
    answers::answer_6()
}

/// Runs the Exercise 7 point-doubling calculation.
pub fn exercise_7() -> Result<Point, PointError> {
    answers::answer_7()
}

#[cfg(test)]
mod tests {
    use super::{
        exercise_1, exercise_2, exercise_3, exercise_4, exercise_5, exercise_6, exercise_7,
    };
    use crate::ecc::Point;

    fn point(x: f64, y: f64) -> Point {
        Point::new(Some(x), Some(y), 5.0, 7.0).unwrap()
    }

    #[test]
    fn runs_the_membership_and_equality_exercises() {
        assert_eq!(exercise_1(), [false, true, true, false]);
        assert_eq!(exercise_2().unwrap(), [true, false]);
    }

    #[test]
    fn runs_the_point_addition_exercises() {
        assert!(exercise_3().unwrap().is_infinity());
        assert_eq!(exercise_4().unwrap(), point(3.0, -7.0));
        assert_eq!(exercise_5().unwrap(), point(2.0, -5.0));
        assert_eq!(exercise_6().unwrap(), point(18.0, 77.0));
        assert_eq!(exercise_7().unwrap(), point(18.0, 77.0));
    }
}
