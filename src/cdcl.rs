pub mod decision;
pub mod first_uip;
pub mod propagation;

#[allow(unused_imports)]
pub use propagation::{variable_name, Clause, Formula, Literal, Variable, CDCL};

pub fn is_satisfying(formula: &Formula, assignment: &Vec<bool>) -> bool {
    formula.iter().all(|clause| {
        clause
            .iter()
            .copied()
            .any(|literal| assignment[variable_name(literal)] == (literal >= 0))
    })
}
