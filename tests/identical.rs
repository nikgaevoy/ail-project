use rand::{thread_rng, Rng};

use ail_project::cdcl::decision::DecideFirstVariable;
use ail_project::cdcl::first_uip::FirstUIP;
use ail_project::cdcl::mincut::CutFirstUIP;
use ail_project::*;
use cdcl::*;

#[test]
fn identical_to_simple() {
    let n: usize = 30;

    let mut formula: Formula = vec![];

    let mut rng = thread_rng();

    for test in 0..1e5 as usize {
        let bnd = n as Literal;

        formula.push((0..3).map(|_| rng.gen_range(-bnd..bnd)).collect());

        let mut new_formula = formula.clone();
        let mut old_formula = formula.clone();

        let new = cdcl::cdcl_solve::<DecideFirstVariable, FirstUIP>(n, &mut new_formula);
        let old = simple_cdcl::cdcl_solve(&mut old_formula);

        assert_eq!(new_formula, old_formula);
        assert_eq!(new.is_some(), old.is_some());

        match new {
            None => {
                println!(
                    "ok: {}\tformula size: {}\tlearned: {}",
                    test,
                    formula.len(),
                    new_formula.len() - formula.len()
                );
                formula.clear();
            }
            Some(mut assignment) => {
                assert!(is_satisfying(&formula, &assignment));
                let old_assignment = old.unwrap();
                assignment.truncate(old_assignment.len());
                assert_eq!(assignment, old_assignment);
            }
        }
    }
}

// #[test] // does not work due to details of implementation
#[allow(dead_code)]
fn identical_to_cut() {
    let n: usize = 30;

    let mut formula: Formula = vec![];

    let mut rng = thread_rng();

    for test in 0..1e5 as usize {
        let bnd = n as Literal;

        formula.push((0..3).map(|_| rng.gen_range(-bnd..bnd)).collect());

        let mut new_formula = formula.clone();
        let mut old_formula = formula.clone();

        let new = cdcl_solve::<DecideFirstVariable, CutFirstUIP>(n, &mut new_formula);
        let old = cdcl_solve::<DecideFirstVariable, FirstUIP>(n, &mut old_formula);

        assert_eq!(new_formula, old_formula);
        assert_eq!(new, old);

        match new {
            None => {
                println!(
                    "ok: {}\tformula size: {}\tlearned: {}",
                    test,
                    formula.len(),
                    new_formula.len() - formula.len()
                );
                formula.clear();
            }
            Some(assignment) => {
                assert!(is_satisfying(&formula, &assignment));
            }
        }
    }
}
