pub mod functions;
pub mod heuristic;

use heuristic::*;
use num::BigUint;
use std::usize;

pub type CutMinimal = MinCutConflict<usize, functions::MinCutFunction>;
pub type CutFirstUIP = MinCutConflict<usize, functions::FirstUIPFunction>;
pub type CutSecondUIP = MinCutConflict<usize, functions::SecondUIPFunction>;
pub type CutThirdUIP = MinCutConflict<usize, functions::ThirdUIPFunction>;
pub type CutAllUIP = MinCutConflict<BigUint, functions::AllUIPFunction>;
pub type CutSatAllUIP = MinCutConflict<usize, functions::SaturatingAllUIPFunction>;
pub type CutRelSat = MinCutConflict<usize, functions::RelSatFunction>;
