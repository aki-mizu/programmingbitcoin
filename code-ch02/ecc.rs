//! Chapter 2: elliptic-curve points.

use std::error::Error;
use std::fmt;
use std::ops::Add;

const FLOAT_TOLERANCE: f64 = 1e-10;

/// An error returned when an elliptic-curve point cannot be constructed or added.
#[derive(Debug, Clone, PartialEq)]
pub enum PointError {
    CoordinatesMustBothBePresentOrAbsent,
    NonFiniteValue {
        name: &'static str,
        value: f64,
    },
    PointNotOnCurve {
        x: f64,
        y: f64,
        a: f64,
        b: f64,
    },
    MismatchedCurves {
        left_a: f64,
        left_b: f64,
        right_a: f64,
        right_b: f64,
    },
}

impl fmt::Display for PointError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CoordinatesMustBothBePresentOrAbsent => {
                write!(formatter, "point coordinates must both be present or both be absent")
            }
            Self::NonFiniteValue { name, value } => {
                write!(formatter, "{name} must be finite, got {value}")
            }
            Self::PointNotOnCurve { x, y, a, b } => {
                write!(formatter, "({x}, {y}) is not on the curve y^2 = x^3 + {a}x + {b}")
            }
            Self::MismatchedCurves {
                left_a,
                left_b,
                right_a,
                right_b,
            } => write!(
                formatter,
                "cannot add points on y^2 = x^3 + {left_a}x + {left_b} and y^2 = x^3 + {right_a}x + {right_b}"
            ),
        }
    }
}

impl Error for PointError {}

// tag::source1[]
/// A point on the elliptic curve $y^2 = x^3 + ax + b$.
#[derive(Clone, Copy)]
pub struct Point {
    x: Option<f64>,
    y: Option<f64>,
    a: f64,
    b: f64,
}

impl Point {
    /// Creates a finite point or the point at infinity when both coordinates are absent.
    pub fn new(x: Option<f64>, y: Option<f64>, a: f64, b: f64) -> Result<Self, PointError> {
        validate_point_inputs(x, y, a, b)?;
        match (x, y) {
            (None, None) => Ok(Self {
                x: None,
                y: None,
                a,
                b,
            }),
            (Some(x), Some(y)) if is_on_curve(x, y, a, b) => Ok(Self {
                x: Some(x),
                y: Some(y),
                a,
                b,
            }),
            (Some(x), Some(y)) => Err(PointError::PointNotOnCurve { x, y, a, b }),
            _ => Err(PointError::CoordinatesMustBothBePresentOrAbsent),
        }
    }
    // end::source1[]

    /// Creates the additive identity for the curve $y^2 = x^3 + ax + b$.
    pub fn infinity(a: f64, b: f64) -> Result<Self, PointError> {
        Self::new(None, None, a, b)
    }

    /// Returns the finite point coordinates, or `None` for the point at infinity.
    pub const fn coordinates(self) -> Option<(f64, f64)> {
        match (self.x, self.y) {
            (Some(x), Some(y)) => Some((x, y)),
            _ => None,
        }
    }

    /// Returns the curve's $a$ coefficient.
    pub const fn a(self) -> f64 {
        self.a
    }

    /// Returns the curve's $b$ coefficient.
    pub const fn b(self) -> f64 {
        self.b
    }

    /// Returns whether this point is the additive identity.
    pub const fn is_infinity(self) -> bool {
        self.x.is_none() && self.y.is_none()
    }

    // tag::source3[]
    /// Adds two points on the same elliptic curve.
    pub fn checked_add(self, other: Self) -> Result<Self, PointError> {
        self.ensure_same_curve(other)?;
        if self.is_infinity() {
            return Ok(other);
        }
        if other.is_infinity() {
            return Ok(self);
        }

        let (Some(x1), Some(y1), Some(x2), Some(y2)) = (self.x, self.y, other.x, other.y) else {
            return Err(PointError::CoordinatesMustBothBePresentOrAbsent);
        };

        if approximately_equal(x1, x2) && !approximately_equal(y1, y2) {
            return Self::infinity(self.a, self.b);
        }

        if !approximately_equal(x1, x2) {
            let slope = (y2 - y1) / (x2 - x1);
            let x3 = slope * slope - x1 - x2;
            let y3 = slope * (x1 - x3) - y1;
            return Self::new(Some(x3), Some(y3), self.a, self.b);
        }

        // tag::source4[]
        if approximately_equal(y1, 0.0) {
            return Self::infinity(self.a, self.b);
        }
        // end::source4[]

        let slope = (3.0 * x1 * x1 + self.a) / (2.0 * y1);
        let x3 = slope * slope - 2.0 * x1;
        let y3 = slope * (x1 - x3) - y1;
        Self::new(Some(x3), Some(y3), self.a, self.b)
    }
    // end::source3[]

    fn ensure_same_curve(self, other: Self) -> Result<(), PointError> {
        if approximately_equal(self.a, other.a) && approximately_equal(self.b, other.b) {
            Ok(())
        } else {
            Err(PointError::MismatchedCurves {
                left_a: self.a,
                left_b: self.b,
                right_a: other.a,
                right_b: other.b,
            })
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        let matching_curve =
            approximately_equal(self.a, other.a) && approximately_equal(self.b, other.b);
        if !matching_curve {
            return false;
        }

        match ((self.x, self.y), (other.x, other.y)) {
            ((None, None), (None, None)) => true,
            ((Some(x), Some(y)), (Some(other_x), Some(other_y))) => {
                approximately_equal(x, other_x) && approximately_equal(y, other_y)
            }
            _ => false,
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.coordinates() {
            Some((x, y)) => write!(formatter, "Point({x},{y})_{}_{}", self.a, self.b),
            None => write!(formatter, "Point(infinity)"),
        }
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, formatter)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        self.checked_add(other)
            .expect("cannot add points on different elliptic curves")
    }
}

fn validate_finite(name: &'static str, value: f64) -> Result<(), PointError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(PointError::NonFiniteValue { name, value })
    }
}

fn validate_point_inputs(x: Option<f64>, y: Option<f64>, a: f64, b: f64) -> Result<(), PointError> {
    for (name, value) in [("a", a), ("b", b)] {
        validate_finite(name, value)?;
    }
    for (name, value) in [("x", x), ("y", y)] {
        if let Some(value) = value {
            validate_finite(name, value)?;
        }
    }
    Ok(())
}

fn is_on_curve(x: f64, y: f64, a: f64, b: f64) -> bool {
    approximately_equal(y * y, x * x * x + a * x + b)
}

fn approximately_equal(left: f64, right: f64) -> bool {
    let scale = left.abs().max(right.abs()).max(1.0);
    (left - right).abs() <= FLOAT_TOLERANCE * scale
}

#[cfg(test)]
mod tests {
    use super::{Point, PointError};

    fn point(x: f64, y: f64, a: f64, b: f64) -> Point {
        Point::new(Some(x), Some(y), a, b).unwrap()
    }

    fn infinity(a: f64, b: f64) -> Point {
        Point::infinity(a, b).unwrap()
    }

    #[test]
    fn validates_points_and_formats_them() {
        assert!(matches!(
            Point::new(Some(-1.0), Some(-2.0), 5.0, 7.0),
            Err(PointError::PointNotOnCurve { .. })
        ));
        assert!(matches!(
            Point::new(Some(1.0), None, 5.0, 7.0),
            Err(PointError::CoordinatesMustBothBePresentOrAbsent)
        ));
        assert_eq!(
            format!("{}", point(-1.0, -1.0, 5.0, 7.0)),
            "Point(-1,-1)_5_7"
        );
        assert_eq!(format!("{}", infinity(5.0, 7.0)), "Point(infinity)");
    }

    #[test]
    fn compares_points_by_coordinates_and_curve() {
        assert_eq!(point(-1.0, -1.0, 5.0, 7.0), point(-1.0, -1.0, 5.0, 7.0));
        assert_ne!(point(-1.0, -1.0, 5.0, 7.0), point(18.0, 77.0, 5.0, 7.0));
        assert_ne!(infinity(5.0, 7.0), infinity(6.0, 7.0));
    }

    #[test]
    fn adds_the_identity_and_additive_inverses() {
        let first = point(-1.0, -1.0, 5.0, 7.0);
        let inverse = point(-1.0, 1.0, 5.0, 7.0);
        let identity = infinity(5.0, 7.0);

        assert_eq!(identity.checked_add(first).unwrap(), first);
        assert_eq!(first.checked_add(identity).unwrap(), first);
        assert_eq!(first.checked_add(inverse).unwrap(), identity);
    }

    #[test]
    fn adds_distinct_points_and_doubles_points() {
        assert_eq!(
            point(3.0, 7.0, 5.0, 7.0)
                .checked_add(point(-1.0, -1.0, 5.0, 7.0))
                .unwrap(),
            point(2.0, -5.0, 5.0, 7.0)
        );
        assert_eq!(
            point(-1.0, -1.0, 5.0, 7.0)
                .checked_add(point(-1.0, -1.0, 5.0, 7.0))
                .unwrap(),
            point(18.0, 77.0, 5.0, 7.0)
        );
    }

    #[test]
    fn doubles_a_point_with_zero_y_to_infinity() {
        let point_with_vertical_tangent = point(0.0, 0.0, 0.0, 0.0);

        assert_eq!(
            point_with_vertical_tangent
                .checked_add(point_with_vertical_tangent)
                .unwrap(),
            infinity(0.0, 0.0)
        );
    }

    #[test]
    fn rejects_point_addition_across_curves() {
        assert!(matches!(
            infinity(5.0, 7.0).checked_add(infinity(6.0, 7.0)),
            Err(PointError::MismatchedCurves { .. })
        ));
    }
}
