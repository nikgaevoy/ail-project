#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DecideFirstVariable {}

use crate::cdcl::propagation::*;

impl DecisionHeuristic for DecideFirstVariable {
    fn from_formula(_n: usize, _formula: &Formula) -> Self {
        Self::default()
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

    fn decide_literal(&mut self, _formula: &Formula, trail: &Trail) -> Option<Literal> {
        trail
            .assignment
            .iter()
            .enumerate()
            .find_map(|(varialbe, state)| {
                if state.is_unset() {
                    Some(varialbe as Literal)
                } else {
                    None
                }
            })
    }
}
