use ail_project::*;

use ail_project::cdcl::decision::DecideFirstVariable;
use ail_project::cdcl::first_uip::FirstUIP;
use ail_project::cdcl::mincut::{CutFirstUIP, CutMinimal};
use ail_project::cdcl::propagation::{ConflictAnalysis, DecisionHeuristic};
use cdcl::*;
use rand::{thread_rng, Rng};
use varisat::{CnfFormula, Lit, Solver};

fn test_random<D: DecisionHeuristic, C: ConflictAnalysis>() {
    let n: usize = 4;

    let mut formula: Formula = vec![];

    let mut rng = thread_rng();

    for test in 0..1e5 as usize {
        let bnd = n as Literal;

        formula.push((0..3).map(|_| rng.gen_range(-bnd..bnd)).collect());

        let mut incremental = formula.clone();

        let new = cdcl_solve::<D, C>(n, &mut incremental);

        let mut old = Solver::new();
        old.add_formula(&CnfFormula::from(formula.iter().map(|clause| {
            clause
                .iter()
                .map(|&lit| Lit::from_index(variable_name(lit), lit >= 0))
                .collect::<Vec<_>>()
        })));

        assert_eq!(new.is_some(), old.solve().unwrap());

        match new {
            None => {
                println!(
                    "ok: {}\tformula size: {}\tlearned: {}",
                    test,
                    formula.len(),
                    incremental.len() - formula.len()
                );
                formula.clear();
            }
            Some(assignment) => {
                assert!(is_satisfying(&formula, &assignment));
            }
        }
    }
}

#[test]
fn basic_correctness() {
    test_random::<DecideFirstVariable, FirstUIP>()
}

#[test]
fn first_uip_correctness() {
    test_random::<DecideFirstVariable, CutFirstUIP>()
}

#[test]
fn mincut_correctness() {
    test_random::<DecideFirstVariable, CutMinimal>()
}
