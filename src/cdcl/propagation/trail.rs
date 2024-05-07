use ClauseType::*;
use VariableState::*;

pub type Variable = usize;
pub type Literal = isize;
pub type Clause = Vec<Literal>;
pub type Formula = Vec<Clause>;

pub fn variable_name(literal: Literal) -> Variable {
    literal.max(!literal) as Variable
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub enum VariableState {
    #[default]
    Unset,
    False(usize),
    True(usize),
}

impl VariableState {
    pub fn from_bool(value: bool, level: usize) -> Self {
        if value {
            True(level)
        } else {
            False(level)
        }
    }

    pub fn decision_level(&self) -> usize {
        match self {
            Unset => {
                panic!("Decision level of unset variable")
            }
            False(l) => *l,
            True(l) => *l,
        }
    }

    pub fn bool_value(&self) -> bool {
        match self {
            Unset => {
                panic!("Boolean value of unset variable")
            }
            False(_) => false,
            True(_) => true,
        }
    }

    pub fn is_true(&self) -> bool {
        match self {
            True(_) => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    pub fn is_false(&self) -> bool {
        match self {
            False(_) => true,
            _ => false,
        }
    }

    pub fn is_unset(&self) -> bool {
        match self {
            Unset => true,
            _ => false,
        }
    }
}

pub type Reason = Option<usize>;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ClauseType {
    #[default]
    Unwatched,
    Satisfied,
    Falsified,
    Unit(Literal),
    Watched(Literal, Literal),
}

impl ClauseType {
    pub fn unwrap_watched(&self) -> (Literal, Literal) {
        match self {
            Watched(a, b) => (*a, *b),
            _ => {
                panic!("Unwrap called in non-watched clause type");
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Trail {
    pub assignment: Vec<VariableState>,
    pub levels: Vec<Vec<(Variable, Reason)>>,
    pub clause_types: Vec<ClauseType>,
    pub watches: Vec<[Vec<usize>; 2]>,
}

impl Trail {
    pub fn new(n: usize, m: usize) -> Self {
        Self {
            assignment: vec![Default::default(); n],
            levels: vec![Default::default()],
            clause_types: vec![Default::default(); m],
            watches: vec![Default::default(); n],
        }
    }

    pub fn assign_variable(&mut self, variable: Variable, value: bool, level: usize) {
        debug_assert!(self.assignment[variable].is_unset());

        self.assignment[variable] = VariableState::from_bool(value, level);
    }

    pub fn decide_variable(&mut self, variable: Variable, value: bool) {
        self.assign_variable(variable, value, self.levels.len());
        self.levels.push(vec![(variable, None)]);
    }

    pub fn decide_literal(&mut self, literal: Literal) {
        self.decide_variable(variable_name(literal), literal >= 0);
    }

    pub fn propagate_variable(&mut self, variable: Variable, value: bool, reason_id: usize) {
        self.assign_variable(variable, value, self.levels.len() - 1);
        self.levels
            .last_mut()
            .unwrap()
            .push((variable, Some(reason_id)));
    }

    pub fn propagate_literal(&mut self, literal: Literal, reason_id: usize) {
        self.propagate_variable(variable_name(literal), literal >= 0, reason_id)
    }

    pub fn add_watch(&mut self, literal: Literal, clause_id: usize) {
        self.watches[variable_name(literal)][(literal >= 0) as usize].push(clause_id);
    }

    pub fn to_literal(&self, variable: Variable) -> Literal {
        let literal = variable as Literal;

        if self.assignment[variable].bool_value() {
            literal
        } else {
            !literal
        }
    }
}
