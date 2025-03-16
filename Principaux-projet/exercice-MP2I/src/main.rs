#![allow(unused)]
#![feature(let_chains)]

mod formule;
use formule::{Formule, Valuation};

fn main() {
    let my_valuation: Valuation = Valuation::from([true, false, true, false]);

    // Question 2
    use Formule::*;
    let exemple: Formule<u32> = Formule::imply(
        Bottom | Var(0),
        (!Var(1) & Var(2) | Var(0)) | (Formule::imply(Top, Var(2) | Var(3))),
    );
}
