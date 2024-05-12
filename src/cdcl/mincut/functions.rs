use num::{BigUint, One, Zero};

use crate::cdcl::Formula;

use super::WeightHeuristic;

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct MinCutFunction {}

impl WeightHeuristic<usize> for MinCutFunction {
    fn from_formula(_n: usize, _formula: &Formula) -> Self {
        Self::default()
    }

    fn source_excess(&self, _conflict_level: usize) -> usize {
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

    fn source_excess(&self, _conflict_level: usize) -> usize {
        usize::MAX
    }

    fn gen_vertex_weight(&self, level: usize, conflict_level: usize, _is_decision: bool) -> usize {
        if level == 0 {
            return 0;
        }

        if level == conflict_level {
            usize::MAX / 2
        } else {
            0
        }
    }
}

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct SecondUIPFunction {}

impl WeightHeuristic<usize> for SecondUIPFunction {
    fn from_formula(_n: usize, _formula: &Formula) -> Self {
        Self::default()
    }

    fn source_excess(&self, _conflict_level: usize) -> usize {
        usize::MAX
    }

    fn gen_vertex_weight(&self, level: usize, conflict_level: usize, _is_decision: bool) -> usize {
        if level == 0 {
            return 0;
        }

        let diff = conflict_level - level;

        if diff < 2 {
            (usize::MAX / 2) >> diff
        } else {
            0
        }
    }
}

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ThirdUIPFunction {}

impl WeightHeuristic<usize> for ThirdUIPFunction {
    fn from_formula(_n: usize, _formula: &Formula) -> Self {
        Self::default()
    }

    fn source_excess(&self, _conflict_level: usize) -> usize {
        usize::MAX
    }

    fn gen_vertex_weight(&self, level: usize, conflict_level: usize, _is_decision: bool) -> usize {
        if level == 0 {
            return 0;
        }

        let diff = conflict_level - level;

        if diff < 3 {
            (usize::MAX / 2) >> diff
        } else {
            0
        }
    }
}

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct SaturatingAllUIPFunction {}

impl WeightHeuristic<usize> for SaturatingAllUIPFunction {
    fn from_formula(_n: usize, _formula: &Formula) -> Self {
        Self::default()
    }

    fn source_excess(&self, _conflict_level: usize) -> usize {
        usize::MAX
    }

    fn gen_vertex_weight(&self, level: usize, conflict_level: usize, _is_decision: bool) -> usize {
        if level == 0 {
            return 0;
        }

        (usize::MAX / 4) >> (conflict_level - level).min((usize::BITS - 1) as usize)
    }
}

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct AllUIPFunction {}

impl WeightHeuristic<BigUint> for AllUIPFunction {
    fn from_formula(_n: usize, _formula: &Formula) -> Self {
        Self::default()
    }

    fn source_excess(&self, conflict_level: usize) -> BigUint {
        BigUint::from(4u32) << conflict_level
    }

    fn gen_vertex_weight(
        &self,
        level: usize,
        _conflict_level: usize,
        _is_decision: bool,
    ) -> BigUint {
        if level == 0 {
            return BigUint::zero();
        }

        BigUint::one() << level
    }
}

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct RelSatFunction {}

impl WeightHeuristic<usize> for RelSatFunction {
    fn from_formula(_n: usize, _formula: &Formula) -> Self {
        Self::default()
    }

    fn source_excess(&self, _conflict_level: usize) -> usize {
        usize::MAX
    }

    fn gen_vertex_weight(&self, level: usize, conflict_level: usize, is_decision: bool) -> usize {
        if level == 0 {
            return 0;
        }

        if (level == conflict_level) && !is_decision {
            usize::MAX
        } else {
            0
        }
    }
}
