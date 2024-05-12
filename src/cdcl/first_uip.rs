use std::mem;

use crate::cdcl::propagation::*;

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct FirstUIP {
    conflict_assignment: Vec<bool>,
}

impl FirstUIP {
    pub fn new(n: usize) -> Self {
        Self {
            conflict_assignment: vec![false; n],
        }
    }
}

impl ConflictAnalysis for FirstUIP {
    fn from_formula(n: usize, _formula: &Formula) -> Self {
        Self::new(n)
    }

    fn analyze_conflict(
        &mut self,
        formula: &Formula,
        trail: &Trail,
        mut conflict: Clause,
    ) -> Clause {
        for variable in conflict.iter().map(|literal| variable_name(*literal)) {
            self.conflict_assignment[variable] = true;
        }

        let mut last = trail.levels.last().unwrap().iter().rev().copied();

        loop {
            let (uip, reason) = last.next().unwrap();

            if self.conflict_assignment[uip] {
                if last
                    .clone()
                    .any(|(variable, _)| self.conflict_assignment[variable])
                {
                    let clause = &formula[reason.unwrap()];
                    conflict.splice(
                        conflict.len()..,
                        clause.iter().copied().filter(|literal| {
                            !mem::replace(
                                &mut self.conflict_assignment[variable_name(*literal)],
                                true,
                            )
                        }),
                    );
                    self.conflict_assignment[uip] = false;
                } else {
                    conflict.retain(|literal| {
                        mem::replace(
                            &mut self.conflict_assignment[variable_name(*literal)],
                            false,
                        )
                    });

                    return conflict;
                }
            }
        }
    }

    fn backtrack_and_add_clause(
        &mut self,
        _formula: &Formula,
        _trail: &Trail,
        _level: usize,
        _clause_id: usize,
    ) {
    }

    fn propagate_literal(
        &mut self,
        _formula: &Formula,
        _trail: &Trail,
        _literal: Literal,
        _reason_id: usize,
    ) {
    }

    fn decide_literal(&mut self, _formula: &Formula, _trail: &Trail, _literal: Literal) {}
}
