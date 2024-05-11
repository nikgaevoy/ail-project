use ail_project::*;

use cdcl::*;
use rand::{thread_rng, Rng};
use varisat::{CnfFormula, Lit, Solver};

#[test]
fn varisat() -> Result<(), varisat::solver::SolverError> {
    let n: usize = 30;

    let mut formula: Formula = vec![];

    let mut rng = thread_rng();

    for test in 0..1e5 as usize {
        let bnd = n as Literal;

        formula.push((0..3).map(|_| rng.gen_range(-bnd..bnd)).collect());

        let mut incremental = formula.clone();

        let mut alg = CDCL::new(
            n,
            &mut incremental,
            decision::DecideFirstVariable::default(),
            first_uip::FirstUIP::new(n),
        );

        let new = alg.solve();

        let mut old = Solver::new();
        old.add_formula(&CnfFormula::from(formula.iter().map(|clause| {
            clause
                .iter()
                .map(|&lit| Lit::from_index(variable_name(lit), lit >= 0))
                .collect::<Vec<_>>()
        })));

        assert_eq!(new, old.solve()?);

        if !new {
            println!(
                "ok: {}\tformula size: {}\tlearned: {}",
                test,
                formula.len(),
                incremental.len() - formula.len()
            );
            formula.clear();
        } else {
            assert!(is_satisfying(&formula, &alg.get_assignment()));
        }
    }

    Ok(())
}
