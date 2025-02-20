//! This code was a project that helped me to learn rust's generics.
//! The goal was to make a code that help us create sequence (arithmetic sequence and/or geometric sequence)
//! and to also help the user calculate a term of the list or calculate the sum of terms that are in a range.
//!
//! # Example
//!
//! ```rust
//! let my_sequence = SequenceVariant::Explicit(SequenceType::new_arithmetic(10));
//!
//! assert_eq!(my_sequence.reason(), 10);
//!
//! assert_eq!(my_sequence.sum_from_range());
//! ```

#![deny(rustdoc::invalid_rust_codeblocks)]
#![feature(let_chains)]

use std::ops::{Deref, RangeInclusive};

use num::{pow::Pow, traits::NumAssignOps, FromPrimitive, Num, ToPrimitive, Unsigned};

#[doc(hidden)]
fn main() {
    let example_of_sequence = SequenceVariant::Explicit {
        formula: SequenceType::new_arithemtic(3),
        initial_term: 0,
    };
}

/// Enum that represent all the possible type of a Sequence
///
/// ## Note
///
/// I restreined myself to the arithmetic and geometric sequence.
/// There is actually more type of sequence but the arithmetic and geometric one's are the easiest to use
pub enum SequenceType<T>
where
    T: Num + Copy,
{
    /// Variant that represent an Arithmetic Sequence
    Arithmetic {
        /// Represent the reason of the arithmetic sequence. Sometimes this reason is called `r`
        reason: T,
    },

    /// Variant that represent an Geometric Sequence
    Geometric {
        /// Represent the reason of the geometric sequence. Sometimes this reason is called `q`
        reason: T,
    },
}

impl<T: Num + Copy> SequenceType<T> {
    /// create a new arithmetic sequence from the reason `r`
    pub fn new_arithemtic(reason: T) -> Self {
        Self::Arithmetic { reason }
    }

    /// create a new geometric sequence from the reason `q`
    pub fn new_geometric(reason: T) -> Self {
        Self::Geometric { reason }
    }

    /// this function create a new sequence base on a function or a closure that takes a number and return the same type of number
    /// The function will be used to calculate if it's a function of an [`SequenceType::Arithmetic`] sequence or a [`SequenceType::Geometric`] sequence
    /// If the function belongs to neither of the proposed types, `from_fn` will return [`None`]
    pub fn from_fn(formula: impl Fn(T) -> T) -> Option<Self> {
        // this stop the programm from computing the values in each conditions
        let f_0 = formula(T::zero());
        let f_1 = formula(T::one());
        let f_2 = formula(T::one() + T::one());

        // The formula for calculating the reason r of a arithmetic sequence is: U(n+1) - U(n)
        // if the reason if the same for n=0 and n=1, then its an arithmetic sequence
        let is_arithmetic = f_1 - f_0 == f_2 - f_1;
        if is_arithmetic {
            return Some(Self::new_arithemtic(f_1 - f_0));
        }

        // The formula for calculating the reason q of a geometric sequence is: U(n+1)/U(n)
        // if the reason if the same for n=0 and n=1, then its an geometric sequence
        let is_geometric = f_1 / f_0 == f_2 / f_1;
        if is_geometric {
            return Some(Self::new_geometric(f_1 / f_0));
        }

        // if `None`, then the sequence provided is neither arithmetic, nor geometric
        None
    }

    /// return the reason of the current sequence
    pub fn reason(&self) -> T {
        use SequenceType::*;
        match self {
            Arithmetic { reason } => *reason,
            Geometric { reason } => *reason,
        }
    }
}

pub enum SequenceVariant<T>
where
    T: Num + Copy,
{
    Explicit {
        formula: SequenceType<T>,
        initial_term: T,
    },
    Recurence {
        formula: SequenceType<T>,
        initial_term: T,
    },
}

impl<T> Deref for SequenceVariant<T>
where
    T: Num + Copy,
{
    type Target = SequenceType<T>;

    /// this `impl` help us extract the formula of the sequence
    fn deref(&self) -> &Self::Target {
        match self {
            SequenceVariant::Explicit { formula, .. } => formula,
            SequenceVariant::Recurence { formula, .. } => formula,
        }
    }
}

impl<T> SequenceVariant<T>
where
    T: Num + FromPrimitive + ToPrimitive + Pow<T, Output = T> + NumAssignOps + Copy,
{
    /// returns the initial term of the sequence of [`self`]
    pub fn initial_term(&self) -> T {
        match self {
            SequenceVariant::Explicit { initial_term, .. } => *initial_term,
            SequenceVariant::Recurence { initial_term, .. } => *initial_term,
        }
    }
    /// Calculate the nth term of a Sequence based on the variant [`SequenceVariant`] and the type [`SequenceType`]
    ///
    /// # Example
    ///
    /// ```rust
    /// let example_of_sequence = SequenceVariant::Explicit {
    ///     formula: SequenceType::new_arithemtic(3),
    ///     initial_term: 0,
    /// };
    ///
    /// assert_eq!(6, example_of_sequence.nth_term(2));
    /// ```
    pub fn nth_term<U: Unsigned + ToPrimitive>(&self, n: U) -> T {
        use SequenceVariant::*;
        match self {
            Explicit {
                formula: sequence_type,
                initial_term,
            } => match sequence_type {
                SequenceType::Arithmetic { reason } => {
                    *initial_term + from_unsigned_to_num::<U, T>(n) * (*reason)
                }
                SequenceType::Geometric { reason } => {
                    *initial_term * (*reason).pow(from_unsigned_to_num::<U, T>(n))
                }
            },
            Recurence {
                formula: sequence_type,
                initial_term,
            } => match sequence_type {
                SequenceType::Arithmetic { reason } => {
                    let mut result = *initial_term;
                    // we add the reason n times with a loop because it's a sequence defined by recurence
                    for _ in 0..=n.to_u128().unwrap() {
                        result += *reason;
                    }
                    result
                }
                SequenceType::Geometric { reason } => {
                    let mut result = *initial_term;
                    // we add the reason n times with a loop because it's a sequence defined by recurence
                    for _ in 0..=n.to_u128().unwrap() {
                        result *= *reason;
                    }
                    result
                }
            },
        }
    }

    /// This function make the sum of all elements of the range passed into the function `self.nth_term`
    pub fn sum_from_range<U: Unsigned + ToPrimitive + Copy>(&self, range: RangeInclusive<U>) -> T {
        use std::ops::Bound::Included;
        use std::ops::RangeBounds as _;

        if let Included(&start_of_range) = range.start_bound()
            && let Included(&end_of_range) = range.end_bound()
        {
            // the +1 is there because whe use an RangeInclusive
            let range_len = from_unsigned_to_num::<U, T>(end_of_range - start_of_range) + T::one();
            match **self {
                SequenceType::Arithmetic { .. } => {
                    // represent the number 2 with generics
                    let two = T::one() + T::one();

                    // compute the formula for an arithmetic progression
                    // see more at https://en.wikipedia.org/wiki/Arithmetic_progression
                    let sum = range_len
                        * (self.nth_term(end_of_range) + self.nth_term(start_of_range))
                        / two;

                    return sum;
                }
                SequenceType::Geometric { .. } => {
                    // compute the formula for a geometric serie
                    // see more at https://en.wikipedia.org/wiki/Geometric_series
                    let first_sum = self.initial_term()
                        * ((T::one()
                            - self
                                .reason()
                                .pow(from_unsigned_to_num::<U, T>(end_of_range)))
                            / (T::one() - self.reason()));

                    let second_sum = self.initial_term()
                        * ((T::one()
                            - self
                                .reason()
                                .pow(from_unsigned_to_num::<U, T>(start_of_range)))
                            / (T::one() - self.reason()));

                    return first_sum - second_sum;
                }
            }
        };
        // [`range.start_bound()`] and [`range.end_bound()`] will always return the [`Included`] variant
        // so the previous if let should always be true
        unreachable!()
    }
}

/// This helper function convert any unsigned integer into any number (integers, unsigned integers, floats) like this: <br>
/// [`T1`] -> [`u128`] -> [`T2`]
///
/// The type [`u128`] is used to prevent wrapping if the user used an `unsigned_integer` that is extremely big,
/// or just of type [`u128`]
#[doc(hidden)]
fn from_unsigned_to_num<T1, T2>(unsigned_num: T1) -> T2
where
    T2: Num + FromPrimitive,    // FromPrimitive is needed just for the conversion
    T1: Unsigned + ToPrimitive, // same for ToPrimitive
{
    FromPrimitive::from_u128(unsigned_num.to_u128().unwrap()).unwrap() // can't fail so I used unwrap
}
