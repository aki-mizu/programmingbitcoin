//! Chapter 2 exercise prompts and solutions.

use crate::ecc::{Point, PointError};

/*
// tag::exercise1[]
==== Exercise 1

Determine which of these points are on the curve __y__^2^ = __x__^3^ + 5__x__ + 7:

++++
<ul class="simplelist">
<li>(2,4), (–1,–1), (18,77), (5,7)</li>
</ul>
++++

// end::exercise1[]
*/

// tag::answer1[]
/// Returns whether the four Exercise 1 coordinates belong to the curve.
pub fn answer_1() -> [bool; 4] {
    [
        Point::new(Some(2.0), Some(4.0), 5.0, 7.0).is_ok(),
        Point::new(Some(-1.0), Some(-1.0), 5.0, 7.0).is_ok(),
        Point::new(Some(18.0), Some(77.0), 5.0, 7.0).is_ok(),
        Point::new(Some(5.0), Some(7.0), 5.0, 7.0).is_ok(),
    ]
}
// end::answer1[]

/*
// tag::exercise2[]
==== Exercise 2

Use the `PartialEq` implementation to verify that unequal `Point` values compare unequal.
// end::exercise2[]
*/

// tag::answer2[]
/// Returns the Exercise 2 inequality checks for coordinates on the same curve.
pub fn answer_2() -> Result<[bool; 2], PointError> {
    let first = Point::new(Some(3.0), Some(-7.0), 5.0, 7.0)?;
    let second = Point::new(Some(18.0), Some(77.0), 5.0, 7.0)?;
    let equivalent = Point::new(Some(3.0), Some(-7.0), 5.0, 7.0)?;

    Ok([first != second, first != equivalent])
}
// end::answer2[]

/*
// tag::exercise3[]
==== Exercise 3

Handle the case where the two points are additive inverses (that is, they have the same _x_ but different _y_, causing a vertical line). This should return the point at infinity.
// end::exercise3[]
*/

// tag::answer3[]
/// Returns the point at infinity from adding additive inverses.
pub fn answer_3() -> Result<Point, PointError> {
    let first = Point::new(Some(-1.0), Some(-1.0), 5.0, 7.0)?;
    let inverse = Point::new(Some(-1.0), Some(1.0), 5.0, 7.0)?;

    first.checked_add(inverse)
}
// end::answer3[]

/*
// tag::exercise4[]
==== Exercise 4

For the curve __y__^2^ = __x__^3^ + 5__x__ + 7, what is (2,5) + (–1,–1)?
// end::exercise4[]
*/

// tag::answer4[]
/// Returns the Exercise 4 point-addition result derived from the slope formula.
pub fn answer_4() -> Result<Point, PointError> {
    let (x1, y1) = (2.0, 5.0);
    let (x2, y2) = (-1.0, -1.0);
    let slope = (y2 - y1) / (x2 - x1);
    let x3 = slope * slope - x1 - x2;
    let y3 = slope * (x1 - x3) - y1;

    Point::new(Some(x3), Some(y3), 5.0, 7.0)
}
// end::answer4[]

/*
// tag::exercise5[]
==== Exercise 5

Write the `checked_add` method where __x__~1~ ≠ __x__~2~.
// end::exercise5[]
*/

// tag::answer5[]
/// Adds two distinct points on the Chapter 2 curve.
pub fn answer_5() -> Result<Point, PointError> {
    let first = Point::new(Some(3.0), Some(7.0), 5.0, 7.0)?;
    let second = Point::new(Some(-1.0), Some(-1.0), 5.0, 7.0)?;

    first.checked_add(second)
}
// end::answer5[]

/*
// tag::exercise6[]
==== Exercise 6

For the curve __y__^2^ = __x__^3^ + 5__x__ + 7, what is (–1,–1) + (–1,–1)?
// end::exercise6[]
*/

// tag::answer6[]
/// Returns the Exercise 6 point-doubling result derived from the tangent slope.
pub fn answer_6() -> Result<Point, PointError> {
    let (a, x1, y1) = (5.0, -1.0, -1.0);
    let slope = (3.0 * x1 * x1 + a) / (2.0 * y1);
    let x3 = slope * slope - 2.0 * x1;
    let y3 = slope * (x1 - x3) - y1;

    Point::new(Some(x3), Some(y3), a, 7.0)
}
// end::answer6[]

/*
// tag::exercise7[]
==== Exercise 7

Write the `checked_add` method when __P__~1~ = __P__~2~.
// end::exercise7[]
*/

// tag::answer7[]
/// Doubles a point on the Chapter 2 curve.
pub fn answer_7() -> Result<Point, PointError> {
    let point = Point::new(Some(-1.0), Some(-1.0), 5.0, 7.0)?;

    point.checked_add(point)
}
// end::answer7[]

#[cfg(test)]
mod tests {
    use super::{answer_1, answer_2, answer_3, answer_4, answer_5, answer_6, answer_7};
    use crate::ecc::Point;

    fn point(x: f64, y: f64) -> Point {
        Point::new(Some(x), Some(y), 5.0, 7.0).unwrap()
    }

    #[test]
    fn solves_the_curve_membership_and_equality_exercises() {
        assert_eq!(answer_1(), [false, true, true, false]);
        assert_eq!(answer_2().unwrap(), [true, false]);
    }

    #[test]
    fn solves_the_point_addition_exercises() {
        assert!(answer_3().unwrap().is_infinity());
        assert_eq!(answer_4().unwrap(), point(3.0, -7.0));
        assert_eq!(answer_5().unwrap(), point(2.0, -5.0));
        assert_eq!(answer_6().unwrap(), point(18.0, 77.0));
        assert_eq!(answer_7().unwrap(), point(18.0, 77.0));
    }
}
