use std::mem;

/// Question 1
#[derive(Clone, Default)]
pub enum Formule<T> {
    Impl(Box<Self>, Box<Self>),
    And(Box<Self>, Box<Self>),
    Or(Box<Self>, Box<Self>),
    Not(Box<Self>),
    Var(T),
    Top,
    #[default]
    Bottom,
}

use Formule::*;
use num::cast::AsPrimitive;

impl<T> Formule<T> {
    pub fn imply(left: Self, right: Self) -> Self {
        Self::Impl(Box::new(left), Box::new(right))
    }

    pub fn and(left: Self, right: Self) -> Self {
        Self::And(Box::new(left), Box::new(right))
    }

    pub fn or(left: Self, right: Self) -> Self {
        Self::Or(Box::new(left), Box::new(right))
    }

    pub fn not(inner: Self) -> Self {
        Self::Not(Box::new(inner))
    }
}

// region: implementation of the Bit opperation trait to simplify the creation of a formula

use core::ops;

impl<T> ops::BitAnd<bool> for Formule<T> {
    type Output = Self;
    fn bitand(self, rhs: bool) -> Self::Output {
        if rhs {
            Self::and(self, Self::Top)
        } else {
            Self::and(self, Self::Bottom)
        }
    }
}

impl<T> ops::BitAnd<Self> for Formule<T> {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self::and(self, rhs)
    }
}

impl<T> ops::BitOr<bool> for Formule<T> {
    type Output = Self;
    fn bitor(self, rhs: bool) -> Self::Output {
        if rhs {
            Self::or(self, Self::Top)
        } else {
            Self::or(self, Self::Bottom)
        }
    }
}

impl<T> ops::BitOr<Self> for Formule<T> {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self::or(self, rhs)
    }
}

impl<T> ops::Not for Formule<T> {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self::not(self)
    }
}

// endregion

impl<T: num::Integer + Copy> Formule<T> {
    /// Question 3
    pub fn size(&self) -> u128 {
        match self {
            Var(_) | Top | Bottom => 1,
            Impl(left_formula, right_formula) => 1 + left_formula.size() + right_formula.size(),
            And(left_formula, right_formula) => 1 + left_formula.size() + right_formula.size(),
            Or(left_formula, right_formula) => 1 + left_formula.size() + right_formula.size(),
            Not(formula) => 1 + formula.size(),
        }
    }

    /// Question 4
    pub fn max_var_indice(&self) -> Option<T> {
        use std::cmp::max;
        match self {
            Var(var) => Some(*var),
            Impl(left_formula, right_formula)
            | And(left_formula, right_formula)
            | Or(left_formula, right_formula) => max(
                left_formula.max_var_indice(),
                right_formula.max_var_indice(),
            ),
            Not(formula) => formula.max_var_indice(),
            Top => None,
            Bottom => None,
        }
    }

    pub fn deleate_impl(&mut self) {
        if let (Impl(left_formula, right_formula)) = self {
            *self = Or(
                Box::new(Not(Box::new(mem::take(left_formula)))),
                Box::new(mem::take(right_formula)),
            )
        }
        match self {
            And(left_formula, right_formula) => {
                left_formula.deleate_impl();
                right_formula.deleate_impl();
            }
            Or(left_formula, right_formula) => {
                left_formula.deleate_impl();
                right_formula.deleate_impl();
            }
            Not(formula) => {
                formula.deleate_impl();
            }
            _ => (),
        }
    }

    pub fn truth_value(&mut self, valuation: [bool; 4]) -> bool
    where
        T: AsPrimitive<usize>,
    {
        match self {
            Impl(left_formula, right_formula) => {
                unimplemented!("I don't know how to use the impl in this function")
            }
            And(left_formula, right_formula) => {
                left_formula.truth_value(valuation) && right_formula.truth_value(valuation)
            }
            Or(left_formula, right_formula) => {
                left_formula.truth_value(valuation) && right_formula.truth_value(valuation)
            }
            Not(formula) => !formula.truth_value(valuation),
            Var(index) => valuation[index.as_()],
            Top => true,
            Bottom => false,
        }
    }

    pub fn repel_negs(&mut self) {
        match self {
            Impl(left_formula, right_formula)
            | And(left_formula, right_formula)
            | Or(left_formula, right_formula) => {
                left_formula.repel_negs();
                right_formula.repel_negs();
            }
            Not(formula) => match &mut **formula {
                Impl(left_formula, right_formula) => {
                    *self = Self::imply(
                        Self::Not(mem::take(left_formula)),
                        Self::Not(mem::take(right_formula)),
                    );
                    self.repel_negs();
                }
                And(left_formula, right_formula) => {
                    *self = Self::or(
                        Self::Not(mem::take(left_formula)),
                        Self::Not(mem::take(right_formula)),
                    );
                    self.repel_negs();
                }
                Or(left_formula, right_formula) => {
                    *self = Self::and(
                        Self::Not(mem::take(left_formula)),
                        Self::Not(mem::take(right_formula)),
                    );
                    self.repel_negs();
                }
                Not(formula) => {
                    *self = mem::take(formula);
                    self.repel_negs();
                }
                _ => (),
            },
            _ => (),
        }
    }

    pub fn complexity(&self) -> u128 {
        self.size()
    }
}

impl<T: num::Integer + Copy> PartialEq for Formule<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Var(_), Var(_)) => true,
            (Impl(left_formula, right_formula), Impl(autre_f1, autre_f2))
            | (And(left_formula, right_formula), And(autre_f1, autre_f2))
            | (Or(left_formula, right_formula), Or(autre_f1, autre_f2)) => {
                left_formula == autre_f1 && right_formula == autre_f2
            }
            (Not(formula), Not(autre_f)) => formula == autre_f,
            (Top, Top) => true,
            (Bottom, Bottom) => true,
            _ => false,
        }
    }
}

/// Aussi Question 1
#[derive(Clone, Copy)]
pub struct Valuation([bool; 4]);

impl From<[bool; 4]> for Valuation {
    fn from(value: [bool; 4]) -> Self {
        Valuation(value)
    }
}

#[derive(Clone, Copy)]
pub struct IncrementError;

impl Valuation {
    pub fn increment(&mut self) -> Result<(), IncrementError> {
        fn increment_helper(valuation: &mut [bool; 4], index: usize) -> Result<(), IncrementError> {
            if index >= valuation.len() {
                return Err(IncrementError);
            }
            if valuation[index] {
                valuation[index] = false;
                increment_helper(valuation, index + 1);
            } else {
                valuation[index] = true;
            }
            Ok(())
        }
        let valuation_len = self.0.len();
        increment_helper(&mut self.0, valuation_len)
    }
}
