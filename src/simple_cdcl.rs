use std::mem;

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
enum VariableState {
    #[default]
    Unset,
    False(usize),
    True(usize),
}

impl VariableState {
    fn from_bool(value: bool, level: usize) -> Self {
        if value {
            True(level)
        } else {
            False(level)
        }
    }

    fn decision_level(&self) -> usize {
        match self {
            Unset => {
                panic!("Decision level of unset variable")
            }
            False(l) => *l,
            True(l) => *l,
        }
    }

    fn bool_value(&self) -> bool {
        match self {
            Unset => {
                panic!("Boolean value of unset variable")
            }
            False(_) => false,
            True(_) => true,
        }
    }

    fn is_true(&self) -> bool {
        match self {
            True(_) => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    fn is_false(&self) -> bool {
        match self {
            False(_) => true,
            _ => false,
        }
    }

    fn is_unset(&self) -> bool {
        match self {
            Unset => true,
            _ => false,
        }
    }
}

type Reason = Option<usize>;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
enum ClauseType {
    #[default]
    Unwatched,
    Satisfied,
    Falsified,
    Unit(Literal),
    Watched(Literal, Literal),
}

impl ClauseType {
    fn unwrap_watched(&self) -> (Literal, Literal) {
        match self {
            Watched(a, b) => (*a, *b),
            _ => {
                panic!("Unwrap called in non-watched clause type");
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Trail {
    assignment: Vec<VariableState>,
    levels: Vec<Vec<(Variable, Reason)>>,
    clause_types: Vec<ClauseType>,
    watches: Vec<[Vec<usize>; 2]>,
}

impl Trail {
    fn new(n: usize, m: usize) -> Self {
        Self {
            assignment: vec![Default::default(); n],
            levels: vec![Default::default()],
            clause_types: vec![Default::default(); m],
            watches: vec![Default::default(); n],
        }
    }

    fn assign_variable(&mut self, variable: Variable, value: bool, level: usize) {
        debug_assert!(self.assignment[variable].is_unset());

        self.assignment[variable] = VariableState::from_bool(value, level);
    }

    fn decide_literal(&mut self, variable: Variable, value: bool) {
        self.assign_variable(variable, value, self.levels.len());
        self.levels.push(vec![(variable, None)]);
    }

    fn propagate_variable(&mut self, variable: Variable, value: bool, reason_id: usize) {
        self.assign_variable(variable, value, self.levels.len() - 1);
        self.levels
            .last_mut()
            .unwrap()
            .push((variable, Some(reason_id)));
    }

    fn propagate_literal(&mut self, literal: Literal, reason_id: usize) {
        self.propagate_variable(variable_name(literal), literal >= 0, reason_id)
    }

    fn add_watch(&mut self, literal: Literal, clause_id: usize) {
        self.watches[variable_name(literal)][(literal >= 0) as usize].push(clause_id);
    }

    fn to_literal(&self, variable: Variable) -> Literal {
        let literal = variable as Literal;

        if self.assignment[variable].bool_value() {
            literal
        } else {
            !literal
        }
    }
}

pub fn cdcl_solve(formula: &mut Formula) -> Option<Vec<bool>> {
    let mut sat = CDCL::new(formula);

    if sat.solve() {
        Some(
            sat.trail
                .assignment
                .iter()
                .map(|state| state.is_true())
                .collect(),
        )
    } else {
        None
    }
}

#[derive(Debug)]
struct CDCL<'a> {
    trail: Trail,
    formula: &'a mut Formula,
    conflict_assignment: Vec<bool>,
}

impl<'a> CDCL<'a> {
    fn new(formula: &'a mut Formula) -> CDCL<'a> {
        let n = formula
            .iter()
            .flat_map(|clause| {
                clause
                    .iter()
                    .copied()
                    .map(|literal| variable_name(literal) + 1)
            })
            .max()
            .unwrap_or(0);

        CDCL::<'a> {
            trail: Trail::new(n, formula.len()),
            formula,
            conflict_assignment: vec![false; n],
        }
    }

    fn add_clause(&mut self, clause: Clause, clause_type: ClauseType) {
        self.formula.push(clause);
        self.trail.clause_types.push(clause_type);
    }

    fn get_clause_type(&self, clause: &Clause, unit: Option<Literal>) -> ClauseType {
        let mut ans = Falsified;

        if let Some(literal) = unit {
            let unit_type = self.trail.assignment[variable_name(literal)];

            if unit_type.is_unset() {
                ans = Unit(literal)
            } else if unit_type.bool_value() == (literal >= 0) {
                return Satisfied;
            }
        }

        for literal in clause.iter().copied() {
            match self.trail.assignment[variable_name(literal)] {
                Unset => match ans {
                    Falsified => {
                        ans = Unit(literal);
                    }
                    Unit(known) => {
                        if literal != known {
                            ans = Watched(known, literal)
                        }
                    }
                    _ => {}
                },
                False(_) => {
                    if literal < 0 {
                        return Satisfied;
                    }
                }
                True(_) => {
                    if literal >= 0 {
                        return Satisfied;
                    }
                }
            }
        }

        ans
    }

    fn analyze_conflict(&mut self, mut conflict: Clause) -> (Clause, Literal) {
        for variable in conflict.iter().map(|literal| variable_name(*literal)) {
            self.conflict_assignment[variable] = true;
        }

        let mut last = self.trail.levels.last().unwrap().iter().rev().copied();

        loop {
            let (uip, reason) = last.next().unwrap();

            if self.conflict_assignment[uip] {
                if last
                    .clone()
                    .any(|(variable, _)| self.conflict_assignment[variable])
                {
                    let clause = &self.formula[reason.unwrap()];
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

                    return (conflict, !self.trail.to_literal(uip));
                }
            }
        }
    }

    fn preprocess_clauses(&mut self) -> bool {
        for (index, clause) in self.formula.iter().enumerate() {
            self.trail.clause_types[index] = self.get_clause_type(clause, None);

            match self.trail.clause_types[index] {
                Unwatched => {
                    unreachable!()
                }
                Satisfied => {}
                Falsified => {
                    return false;
                }
                Unit(literal) => self.trail.propagate_literal(literal, index),
                Watched(a, b) => {
                    self.trail.add_watch(a, index);
                    self.trail.add_watch(b, index);
                }
            }
        }

        true
    }

    fn backtrack_and_add_uip_clause(&mut self, clause: Clause, uip: Literal) -> usize {
        let second_deepest = clause
            .iter()
            .copied()
            .filter(|&literal| literal != uip)
            .max_by_key(|&literal| self.trail.assignment[variable_name(literal)].decision_level());

        let back_level = second_deepest.map_or(0, |back| {
            self.trail.assignment[variable_name(back)].decision_level()
        });

        debug_assert!(back_level + 1 < self.trail.levels.len());

        self.trail
            .levels
            .drain(back_level + 1..)
            .flatten()
            .for_each(|(variable, _)| self.trail.assignment[variable] = Unset);

        let new_clause_id = self.formula.len();

        if let Some(literal) = second_deepest {
            self.add_clause(clause, Watched(literal, uip));
            self.trail.add_watch(literal, new_clause_id);
            self.trail.add_watch(uip, new_clause_id);
        } else {
            self.add_clause(clause, Unit(uip));
        }

        new_clause_id
    }

    fn process_unit_clauses(&mut self) -> bool {
        let mut variable_index = 0;

        'unit_variables: while variable_index < self.trail.levels.last().unwrap().len() {
            let variable = self.trail.levels.last().unwrap()[variable_index].0;
            variable_index += 1;

            let value = self.trail.assignment[variable].bool_value();
            let falsified_literal = !self.trail.to_literal(variable);

            let mut watch_index = 0;

            while watch_index < self.trail.watches[variable][(!value) as usize].len() {
                let clause_id = self.trail.watches[variable][(!value) as usize][watch_index];

                let (a, b) = self.trail.clause_types[clause_id].unwrap_watched();

                if a != falsified_literal && b != falsified_literal {
                    self.trail.watches[variable][(!value) as usize].swap_remove(watch_index);

                    continue;
                } else {
                    watch_index += 1;
                }

                let other_literal = a ^ b ^ falsified_literal;

                match self.get_clause_type(&self.formula[clause_id], Some(other_literal)) {
                    Unwatched => {
                        unreachable!()
                    }
                    Satisfied => {}
                    Falsified => {
                        if self.trail.levels.len() == 1 {
                            return false;
                        }

                        let (conflict, uip) =
                            self.analyze_conflict(self.formula[clause_id].clone());

                        let new_clause_id = self.backtrack_and_add_uip_clause(conflict, uip);

                        variable_index = self.trail.levels.last().unwrap().len();
                        self.trail.propagate_literal(uip, new_clause_id);

                        continue 'unit_variables;
                    }
                    Unit(new_literal) => self.trail.propagate_literal(new_literal, clause_id),
                    Watched(a, b) => {
                        self.trail.clause_types[clause_id] = Watched(a, b);
                        if a != falsified_literal {
                            self.trail.add_watch(a, clause_id);
                        }
                        self.trail.add_watch(b, clause_id);
                    }
                }
            }
        }

        true
    }

    fn solve(&mut self) -> bool {
        if !self.preprocess_clauses() {
            return false;
        }

        'decision: loop {
            if !self.process_unit_clauses() {
                return false;
            }

            for (variable, state) in self.trail.assignment.iter_mut().enumerate() {
                if *state == Unset {
                    self.trail.decide_literal(variable, true);

                    continue 'decision;
                }
            }

            return true;
        }
    }
}
