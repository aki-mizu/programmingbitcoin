//! Chapter 1 exercise prompts and solutions.

use crate::ecc::{FieldElement, FieldElementError};
use num_traits::ToPrimitive;

/*
// tag::exercise1[]
==== Exercise 1

Use the derived `PartialEq` implementation to verify that two `FieldElement` values are _not equal_ when their values or field orders differ.
// end::exercise1[]
*/

// tag::answer1[]
/// Returns the two Exercise 1 inequality checks for different values and field orders.
pub fn answer_1() -> Result<[bool; 2], FieldElementError> {
    let element = FieldElement::new(2, 31)?;
    let different_value = FieldElement::new(15, 31)?;
    let different_field = FieldElement::new(2, 29)?;

    Ok([element != different_value, element != different_field])
}
// end::answer1[]

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

// tag::answer2[]
/// Returns the four answers for Exercise 2 in $F_{57}$.
pub fn answer_2() -> [u64; 4] {
    let prime = 57;
    [
        (44 + 33) % prime,
        (9 + prime - 29) % prime,
        (17 + 42 + 49) % prime,
        (52 + prime - 30 - 38) % prime,
    ]
}
// end::answer2[]

/*
// tag::exercise3[]
==== Exercise 3

Write the corresponding `checked_sub` method that defines the subtraction of two `FieldElement` objects.
// end::exercise3[]
*/

// tag::answer3[]
/// Returns the two answers for Exercise 3 in $F_{31}$.
pub fn answer_3() -> Result<[u64; 2], FieldElementError> {
    let prime = 31;
    let first = FieldElement::new(29, prime)?
        .checked_sub(&FieldElement::new(4, prime)?)?
        .value()
        .to_u64()
        .expect("Chapter 1 exercise values fit in u64");
    let second = FieldElement::new(15, prime)?
        .checked_sub(&FieldElement::new(30, prime)?)?
        .value()
        .to_u64()
        .expect("Chapter 1 exercise values fit in u64");

    Ok([first, second])
}
// end::answer3[]

/*
// tag::exercise4[]
==== Exercise 4

Solve the following equations in __F__~97~ (again, assume multiplication and exponentiation are field versions):

* 95 * 45 * 31
* 17 * 13 * 19 * 44
* 12^7^ * 77^49^
// end::exercise4[]
*/

// tag::answer4[]
/// Returns the three answers for Exercise 4 in $F_{97}$.
pub fn answer_4() -> Result<[u64; 3], FieldElementError> {
    let prime = 97;
    let first = FieldElement::new(95, prime)?
        .checked_mul(&FieldElement::new(45, prime)?)?
        .checked_mul(&FieldElement::new(31, prime)?)?
        .value()
        .to_u64()
        .expect("Chapter 1 exercise values fit in u64");
    let second = FieldElement::new(17, prime)?
        .checked_mul(&FieldElement::new(13, prime)?)?
        .checked_mul(&FieldElement::new(19, prime)?)?
        .checked_mul(&FieldElement::new(44, prime)?)?
        .value()
        .to_u64()
        .expect("Chapter 1 exercise values fit in u64");
    let third = FieldElement::new(12, prime)?
        .pow(7)
        .checked_mul(&FieldElement::new(77, prime)?.pow(49))?
        .value()
        .to_u64()
        .expect("Chapter 1 exercise values fit in u64");

    Ok([first, second, third])
}
// end::answer4[]

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

// tag::answer5[]
/// Returns the Exercise 5 multiples for $k = 1, 3, 7, 13, 18$ in $F_{19}$.
pub fn answer_5() -> [[u64; 19]; 5] {
    const PRIME: u64 = 19;
    const MULTIPLIERS: [u64; 5] = [1, 3, 7, 13, 18];

    std::array::from_fn(|multiplier_index| {
        let multiplier = MULTIPLIERS[multiplier_index];
        std::array::from_fn(|value| multiplier * value as u64 % PRIME)
    })
}
// end::answer5[]

/*
// tag::exercise6[]
==== Exercise 6

Write the corresponding `checked_mul` method that defines the multiplication of two finite field elements.
// end::exercise6[]
*/

// tag::answer6[]
/// Returns the Exercise 6 answer in $F_{31}$.
pub fn answer_6() -> Result<u64, FieldElementError> {
    let prime = 31;

    Ok(FieldElement::new(24, prime)?
        .checked_mul(&FieldElement::new(19, prime)?)?
        .value()
        .to_u64()
        .expect("Chapter 1 exercise values fit in u64"))
}
// end::answer6[]

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

// tag::answer7[]
/// Returns the Exercise 7 results for $p = 7, 11, 17, 31$.
pub fn answer_7() -> Result<Vec<Vec<u64>>, FieldElementError> {
    [7, 11, 17, 31]
        .into_iter()
        .map(|prime| {
            (1..prime)
                .map(|value| {
                    FieldElement::new(value, prime).map(|element| {
                        element
                            .pow((prime - 1) as i128)
                            .value()
                            .to_u64()
                            .expect("Chapter 1 exercise values fit in u64")
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect()
}
// end::answer7[]

/*
// tag::exercise8[]
==== Exercise 8

Solve the following equations in __F__~31~:

* 3 / 24
* 17^-3^
* 4^-4^ * 11
// end::exercise8[]
*/

// tag::answer8[]
/// Returns the three answers for Exercise 8 in $F_{31}$.
pub fn answer_8() -> Result<[u64; 3], FieldElementError> {
    let prime = 31;
    let first = FieldElement::new(3, prime)?
        .checked_div(&FieldElement::new(24, prime)?)?
        .value()
        .to_u64()
        .expect("Chapter 1 exercise values fit in u64");
    let second = FieldElement::new(17, prime)?
        .pow(-3)
        .value()
        .to_u64()
        .expect("Chapter 1 exercise values fit in u64");
    let third = FieldElement::new(4, prime)?
        .pow(-4)
        .checked_mul(&FieldElement::new(11, prime)?)?
        .value()
        .to_u64()
        .expect("Chapter 1 exercise values fit in u64");

    Ok([first, second, third])
}
// end::answer8[]

/*
// tag::exercise9[]
==== Exercise 9

Write the corresponding `checked_div` method that defines the division of two field elements.
// end::exercise9[]
*/

// tag::answer9[]
/// Returns the Exercise 9 answer in $F_{31}$.
pub fn answer_9() -> Result<u64, FieldElementError> {
    let prime = 31;

    Ok(FieldElement::new(3, prime)?
        .checked_div(&FieldElement::new(24, prime)?)?
        .value()
        .to_u64()
        .expect("Chapter 1 exercise values fit in u64"))
}
// end::answer9[]
