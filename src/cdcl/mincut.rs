pub mod heuristic;
pub mod functions;

use std::usize;
use heuristic::*;

pub type CutFirstUIP = MinCutConflict<usize, functions::FirstUIPFunction>;
pub type CutMinimal = MinCutConflict<usize, functions::MinCutFunction>;
