use super::WeightHeuristic;
use crate::cdcl::Formula;

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct MinCutFunction {}

impl WeightHeuristic<usize> for MinCutFunction {
    fn from_formula(_n: usize, _formula: &Formula) -> Self {
        Self::default()
    }

    fn source_excess(&self) -> usize {
        usize::MAX
    }

    fn gen_vertex_weight(&self, level: usize, conflict_level: usize, _is_decision: bool) -> usize {
        if level == conflict_level {
            usize::MAX / 2
        } else {
            1
        }
    }
}

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct FirstUIPFunction {}

impl WeightHeuristic<usize> for FirstUIPFunction {
    fn from_formula(_n: usize, _formula: &Formula) -> Self {
        Self::default()
    }

    fn source_excess(&self) -> usize {
        usize::MAX
    }

    fn gen_vertex_weight(&self, level: usize, conflict_level: usize, _is_decision: bool) -> usize {
        if level == conflict_level {
            usize::MAX / 2
        } else {
            0
        }
    }
}
