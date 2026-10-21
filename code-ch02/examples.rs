//! Runnable versions of the Chapter 2 examples.

use crate::ecc::{Point, PointError};

// tag::example1[]
/// Corresponds to Example 1 and returns the expected construction error.
pub fn example_1() -> Result<Point, PointError> {
    Point::new(Some(-1.0), Some(-2.0), 5.0, 7.0)
}
// end::example1[]

// tag::example2[]
/// Corresponds to Example 2.
pub fn example_2() -> Result<[Point; 3], PointError> {
    let first = Point::new(Some(-1.0), Some(-1.0), 5.0, 7.0)?;
    let second = Point::new(Some(-1.0), Some(1.0), 5.0, 7.0)?;
    let infinity = Point::infinity(5.0, 7.0)?;

    Ok([
        first.checked_add(infinity)?,
        infinity.checked_add(second)?,
        first.checked_add(second)?,
    ])
}
// end::example2[]

#[cfg(test)]
mod tests {
    use super::{example_1, example_2};
    use crate::ecc::{Point, PointError};

    #[test]
    fn rejects_the_invalid_example_point() {
        assert!(matches!(
            example_1(),
            Err(PointError::PointNotOnCurve { .. })
        ));
    }

    #[test]
    fn reproduces_the_identity_and_inverse_examples() {
        let first = Point::new(Some(-1.0), Some(-1.0), 5.0, 7.0).unwrap();
        let second = Point::new(Some(-1.0), Some(1.0), 5.0, 7.0).unwrap();
        let infinity = Point::infinity(5.0, 7.0).unwrap();

        assert_eq!(example_2().unwrap(), [first, second, infinity]);
    }
}
