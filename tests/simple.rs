use ail_project::*;

use cdcl::*;
use rand::{thread_rng, Rng};

#[test]
fn simple() {
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

        let old = simple_cdcl::cdcl_solve(&mut formula.clone()).is_some();

        assert_eq!(new, old);

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
}
