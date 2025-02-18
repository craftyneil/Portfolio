use num::Num;

pub enum SequenceType<T>
where
    T: Num + Copy,
{
    Arithmetic { reason: T },
    Geometric { reason: T },
}

impl<T: Num + Copy> SequenceType<T> {
    pub fn new_arithemtic(reason: T) -> Self {
        Self::Arithmetic { reason }
    }

    pub fn new_geometric(reason: T) -> Self {
        Self::Geometric { reason }
    }

    pub fn from_fn(formula: impl Fn(T) -> T) -> Option<Self> {
        // this stop the programm from computing the values in each conditions
        let f_0 = formula(T::zero());
        let f_1 = formula(T::one());
        let f_2 = formula(T::one() + T::one());

        let is_arithmetic = f_1 - f_0 == f_2 - f_1;
        if is_arithmetic {
            return Some(Self::new_arithemtic(f_1 - f_0));
        }

        let is_geometric = f_1 / f_0 == f_2 / f_1;
        if is_geometric {
            return Some(Self::new_geometric(f_1 / f_0));
        }

        // if `None`, then the sequence provided is neither arithmetic, nor geometric
        None
    }

    // pub fn reason(&self) -> T {
    //     use SequenceType::*;
    //     match self {
    //         Arithmetic { reason } => *reason,
    //         Geometric { reason } => *reason,
    //     }
    // }
}

enum SequenceVariant<T>
where
    T: Num + Copy,
{
    Explicit(SequenceType<T>),
    Recurence(SequenceType<T>),
}
