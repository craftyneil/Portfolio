#![allow(unused)]
extern crate proc_macro;

use num::{pow::Pow, FromPrimitive, Num, ToPrimitive, Unsigned};
use rust_math::stringify_sequence_closure;
use std::fmt::{self, Debug};

fn main() {
    let my_sequence: Sequence<u32, i32> = Sequence::from_recurence(
        1,
        #[stringify_sequence_closure]
        |n| 5 + n,
    );
    // println!("{my_sequence:#?}");
}

enum SequenceFormula<T, U>
where
    T: Num,
    U: Num,
{
    Arithmetic(Box<dyn Fn(T) -> U>),
    Geometric(Box<dyn Fn(T) -> U>),
    Other(Box<dyn Fn(T) -> U>),
}

impl<T, U> fmt::Debug for SequenceFormula<T, U>
where
    T: Num + fmt::Debug,
    U: Num + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SequenceFormula::Arithmetic(formula) => {
                todo!();
            }
            SequenceFormula::Geometric(_) => todo!(),
            SequenceFormula::Other(_) => todo!(),
        }
    }
}

impl<T: Num, U: Num> SequenceFormula<T, U> {
    fn new(formula: impl Fn(T) -> U + 'static) -> Self {
        if formula(T::one()) - formula(T::zero())
            == formula(T::one() + T::one()) - formula(T::one())
        {
            return Self::Arithmetic(Box::new(formula));
        } else if formula(T::one()) / formula(T::zero())
            == formula(T::one() + T::one()) / formula(T::one())
        {
            return Self::Geometric(Box::new(formula));
        }
        Self::Other(Box::new(formula))
    }

    fn reason(&self) -> Option<U> {
        match &self {
            SequenceFormula::Arithmetic(formula) => {
                let formula = formula.as_ref();
                Some(formula(T::one()) - formula(T::zero()))
            }
            SequenceFormula::Geometric(formula) => {
                let formula = formula.as_ref();
                Some(formula(T::one()) / formula(T::zero()))
            }
            SequenceFormula::Other(_) => None,
        }
    }
}

trait SequenceMethods<T, U> {
    fn nth_term(&self, n: impl Unsigned + ToPrimitive) -> U;
    // fn sum_of_term(&self, terms: RangeInclusive<impl Unsigned>) -> U;
}

#[derive(Debug)]
struct RecurenceSequence<T>
where
    T: Num,
{
    initial_term: T,
    formula: SequenceFormula<T, T>,
}

impl<T: Num> RecurenceSequence<T> {
    fn new(initial_term: T, formula: impl Fn(T) -> T + 'static) -> Self {
        Self {
            initial_term,
            formula: SequenceFormula::new(formula),
        }
    }
}

impl<T: Num + ToPrimitive + Clone> SequenceMethods<T, T> for RecurenceSequence<T> {
    fn nth_term(&self, n: impl Unsigned + ToPrimitive) -> T {
        let mut accumulator = self.initial_term.clone();

        let formula = match &self.formula {
            SequenceFormula::Arithmetic(formula) => formula.as_ref(),
            SequenceFormula::Geometric(formula) => formula.as_ref(),
            SequenceFormula::Other(formula) => formula.as_ref(),
        };

        for _ in 1..=n.to_u128().unwrap() {
            accumulator = formula(accumulator);
        }

        accumulator
    }

    // fn sum_of_term(&self, terms: RangeInclusive<impl Unsigned>) -> T {
    //     let mut accumulator = 0;
    //     for n in terms {
    //         accumulator += self.nth_term(n);
    //     }
    //     accumulator
    // }
}

impl<T: Unsigned + ToPrimitive, U: Num + Clone + FromPrimitive + 'static + Pow<T, Output = U>>
    From<RecurenceSequence<U>> for Option<ExplicitSequence<T, U>>
{
    fn from(value: RecurenceSequence<U>) -> Self {
        match value.formula {
            SequenceFormula::Arithmetic(_) => Some(ExplicitSequence::new(move |n: T| {
                value.initial_term.clone()
                    + value.formula.reason().unwrap()
                        * FromPrimitive::from_u128(n.to_u128().unwrap()).unwrap()
            })),
            SequenceFormula::Geometric(_) => Some(ExplicitSequence::new(move |n: T| {
                value.initial_term.clone() * value.formula.reason().unwrap().pow(n)
            })),
            SequenceFormula::Other(_) => None,
        }
    }
}

#[derive(Debug)]
struct ExplicitSequence<T, U>
where
    T: Unsigned,
    U: Num,
{
    formula: SequenceFormula<T, U>,
}

impl<T: Unsigned, U: Num> ExplicitSequence<T, U> {
    fn new(formula: impl Fn(T) -> U + 'static) -> Self {
        Self {
            formula: SequenceFormula::new(formula),
        }
    }
}

#[derive(Debug)]
struct Sequence<T, U>
where
    T: Unsigned,
    U: Num,
{
    recurence_part: Option<RecurenceSequence<U>>,
    explicit_part: Option<ExplicitSequence<T, U>>,
}

impl<
        T: Unsigned + ToPrimitive,
        U: Num + Clone + Copy + FromPrimitive + Pow<T, Output = U> + 'static,
    > Sequence<T, U>
{
    fn from_recurence(initial_term: U, formula: impl Fn(U) -> U + 'static + Copy) -> Self {
        Self {
            recurence_part: Some(RecurenceSequence::new(initial_term, formula)),
            explicit_part: RecurenceSequence::new(initial_term, formula).into(),
        }
    }

    fn from_explicit(formula: impl Fn(T) -> U + 'static) -> Self {
        Self {
            recurence_part: None,
            explicit_part: Some(ExplicitSequence::new(formula)),
        }
    }

    fn new(
        initial_term: U,
        recurence_formula: impl Fn(U) -> U + 'static,
        explicit_formula: impl Fn(T) -> U + 'static,
    ) -> Self {
        Self {
            recurence_part: Some(RecurenceSequence::new(initial_term, recurence_formula)),
            explicit_part: Some(ExplicitSequence::new(explicit_formula)),
        }
    }
}
