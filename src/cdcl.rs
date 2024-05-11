pub mod decision;
pub mod first_uip;
pub mod propagation;

use crate::cdcl::propagation::{ConflictAnalysis, DecisionHeuristic};
#[allow(unused_imports)]
pub use propagation::{variable_name, Clause, Formula, Literal, Variable, CDCL};
use std::io::BufRead;

pub fn is_satisfying(formula: &Formula, assignment: &Vec<bool>) -> bool {
    formula.iter().all(|clause| {
        clause
            .iter()
            .copied()
            .any(|literal| assignment[variable_name(literal)] == (literal >= 0))
    })
}

pub fn read_dimacs<R: BufRead>(reader: &mut R) -> (usize, Formula) {
    let [n, m] = loop {
        let mut input = String::new();

        assert_ne!(reader.read_line(&mut input).expect("Failed read"), 0);

        let line = input.trim();

        if line.starts_with("p cnf") {
            let tmp: [usize; 2] = line
                .trim_start_matches("p cnf")
                .trim()
                .split(" ")
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            break tmp;
        } else {
            assert!(line.starts_with("c"));
        }
    };

    let mut ans = Formula::with_capacity(m);

    let mut input = String::new();

    while reader.read_line(&mut input).expect("Failed read") > 0 {
        let line = input.trim();

        if line.starts_with("c") {
            continue;
        } else {
            let mut clause: Vec<_> = line
                .split(" ")
                .map(|s| s.parse::<isize>().unwrap())
                .collect();

            assert_eq!(clause.pop(), Some(0));

            for literal in &mut clause {
                if *literal > 0 {
                    *literal -= 1;
                }
            }

            ans.push(clause);
        }

        input.clear();
    }

    (n, ans)
}

pub fn cdcl_solve<D: DecisionHeuristic, C: ConflictAnalysis>(
    n: usize,
    formula: &mut Formula,
) -> Option<Vec<bool>> {
    let d = D::from_formula(n, &formula);
    let c = C::from_formula(n, &formula);

    let mut cdcl = CDCL::new(n, formula, d, c);

    if cdcl.solve() {
        Some(cdcl.get_assignment())
    } else {
        None
    }
}
