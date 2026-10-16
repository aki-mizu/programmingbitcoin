//! Chapter 1 exercise answers.

use crate::ecc::{FieldElement, FieldElementError};

/*
// tag::exercise1[]
==== Exercise 1

Use the derived `PartialEq` implementation to verify that two `FieldElement` values are _not equal_ when their values or field orders differ.
// end::exercise1[]
*/

/*
// tag::exercise2[]
==== Exercise 2

Solve these problems in __F__~57~ (assume all pass:[+'s] here are pass:[+<sub><em>f</em></sub>] and -'s here are pass:[-<sub><em>f</em></sub>]):

* 44 + 33
* 9 - 29
* 17 + 42 + 49
* 52 - 30 - 38
// end::exercise2[]
*/

/// Returns the four answers for Exercise 2 in $F_{57}$.
pub fn exercise_2() -> [u64; 4] {
    let prime = 57;
    [
        (44 + 33) % prime,
        (9 + prime - 29) % prime,
        (17 + 42 + 49) % prime,
        (52 + prime - 30 - 38) % prime,
    ]
}

/*
// tag::exercise3[]
==== Exercise 3

Write the corresponding `checked_sub` method that defines the subtraction of two `FieldElement` objects.
// end::exercise3[]
*/

/*
// tag::exercise4[]
==== Exercise 4

Solve the following equations in __F__~97~ (again, assume multiplication and exponentiation are field versions):

* 95 * 45 * 31
* 17 * 13 * 19 * 44
* 12^7^ * 77^49^
// end::exercise4[]
*/

/// Returns the three answers for Exercise 4 in $F_{97}$.
pub fn exercise_4() -> Result<[u64; 3], FieldElementError> {
    let prime = 97;
    let first = FieldElement::new(95, prime)?
        .checked_mul(FieldElement::new(45, prime)?)?
        .checked_mul(FieldElement::new(31, prime)?)?
        .value();
    let second = FieldElement::new(17, prime)?
        .checked_mul(FieldElement::new(13, prime)?)?
        .checked_mul(FieldElement::new(19, prime)?)?
        .checked_mul(FieldElement::new(44, prime)?)?
        .value();
    let third = FieldElement::new(12, prime)?
        .pow(7)
        .checked_mul(FieldElement::new(77, prime)?.pow(49))?
        .value();

    Ok([first, second, third])
}

/*
// tag::exercise5[]
==== Exercise 5

For _k_ = 1, 3, 7, 13, 18, what is this set in __F__~19~?

++++
<ul class="simplelist">
<li>{<em>k</em> * 0, <em>k</em> * 1, <em>k</em> * 2, <em>k</em> * 3, ... <em>k</em> * 18}</li>
</ul>
++++

Do you notice anything about these sets?
// end::exercise5[]
*/

/// Returns the Exercise 5 multiples for $k = 1, 3, 7, 13, 18$ in $F_{19}$.
pub fn exercise_5() -> [[u64; 19]; 5] {
    const PRIME: u64 = 19;
    const MULTIPLIERS: [u64; 5] = [1, 3, 7, 13, 18];

    std::array::from_fn(|multiplier_index| {
        let multiplier = MULTIPLIERS[multiplier_index];
        std::array::from_fn(|value| multiplier * value as u64 % PRIME)
    })
}

/*
// tag::exercise6[]
==== Exercise 6

Write the corresponding `checked_mul` method that defines the multiplication of two finite field elements.
// end::exercise6[]
*/

/*
// tag::exercise7[]
==== Exercise 7

For _p_ = 7, 11, 17, 31, what is this set in __F~p~__?

++++
<ul class="simplelist">
<li>{1<sup>(<em>p</em> - 1)</sup>, 2<sup>(<em>p</em> - 1)</sup>, 3<sup>(<em>p</em> - 1)</sup>, 4<sup>(<em>p</em> - 1)</sup>, ... (<em>p</em> - 1)<sup>(<em>p</em> - 1)</sup>}</li>
</ul>
++++
// end::exercise7[]
*/

/// Returns the Exercise 7 results for $p = 7, 11, 17, 31$.
pub fn exercise_7() -> Result<Vec<Vec<u64>>, FieldElementError> {
    [7, 11, 17, 31]
        .into_iter()
        .map(|prime| {
            (1..prime)
                .map(|value| {
                    FieldElement::new(value, prime)
                        .map(|element| element.pow((prime - 1) as i128).value())
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect()
}

/*
// tag::exercise8[]
==== Exercise 8

Solve the following equations in __F__~31~:

* 3 / 24
* 17^-3^
* 4^-4^ * 11
// end::exercise8[]
*/

/// Returns the three answers for Exercise 8 in $F_{31}$.
pub fn exercise_8() -> Result<[u64; 3], FieldElementError> {
    let prime = 31;
    let first = FieldElement::new(3, prime)?
        .checked_div(FieldElement::new(24, prime)?)?
        .value();
    let second = FieldElement::new(17, prime)?.pow(-3).value();
    let third = FieldElement::new(4, prime)?
        .pow(-4)
        .checked_mul(FieldElement::new(11, prime)?)?
        .value();

    Ok([first, second, third])
}

/*
// tag::exercise9[]
==== Exercise 9

Write the corresponding `checked_div` method that defines the division of two field elements.
// end::exercise9[]
*/

#[cfg(test)]
mod tests {
    use super::{exercise_2, exercise_4, exercise_5, exercise_7, exercise_8};

    #[test]
    fn computes_the_scalar_exercise_answers() {
        assert_eq!(exercise_2(), [20, 37, 51, 41]);
        assert_eq!(exercise_4().unwrap(), [23, 68, 63]);
        assert_eq!(exercise_8().unwrap(), [4, 29, 13]);
    }

    #[test]
    fn verifies_prime_field_multiplication_and_fermat_results() {
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
