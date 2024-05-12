pub mod trail;

pub use trail::*;
use ClauseType::*;
use VariableState::*;

#[derive(Debug)]
pub struct CDCL<'a, D: DecisionHeuristic, C: ConflictAnalysis> {
    trail: Trail,
    formula: &'a mut Formula,
    decision_heuristic: D,
    conflict_analysis: C,
}

impl<'a, D: DecisionHeuristic, C: ConflictAnalysis> CDCL<'a, D, C> {
    pub fn new(
        n: usize,
        formula: &'a mut Formula,
        decision_heuristic: D,
        conflict_analysis: C,
    ) -> CDCL<'a, D, C> {
        CDCL::<'a> {
            trail: Trail::new(n, formula.len()),
            formula,
            decision_heuristic,
            conflict_analysis,
        }
    }

    pub fn get_assignment(&self) -> Vec<bool> {
        self.trail
            .assignment
            .iter()
            .map(|state| state.is_true())
            .collect()
    }

    fn add_learned_clause(&mut self, clause: Clause, clause_type: ClauseType) {
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

    fn preprocess_clauses(&mut self) -> bool {
        for (index, clause) in self.formula.clone().iter().enumerate() {
            self.trail.clause_types[index] = self.get_clause_type(clause, None);

            match self.trail.clause_types[index] {
                Unwatched => {
                    unreachable!()
                }
                Satisfied => {}
                Falsified => {
                    return false;
                }
                Unit(literal) => self.propagate_literal(literal, index),
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
            self.add_learned_clause(clause, Watched(literal, uip));
            self.trail.add_watch(literal, new_clause_id);
            self.trail.add_watch(uip, new_clause_id);
        } else {
            self.add_learned_clause(clause, Unit(uip));
        }

        self.decision_heuristic.backtrack_and_add_clause(
            &self.formula,
            &self.trail,
            back_level,
            new_clause_id,
        );
        self.conflict_analysis.backtrack_and_add_clause(
            &self.formula,
            &self.trail,
            back_level,
            new_clause_id,
        );

        new_clause_id
    }

    fn propagate_literal(&mut self, literal: Literal, reason_id: usize) {
        self.trail.propagate_literal(literal, reason_id);
        self.decision_heuristic
            .propagate_literal(&self.formula, &self.trail, literal, reason_id);
        self.conflict_analysis
            .propagate_literal(&self.formula, &self.trail, literal, reason_id);
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

                        let conflict = self.conflict_analysis.analyze_conflict(
                            &self.formula,
                            &self.trail,
                            self.formula[clause_id].clone(),
                        );

                        let uip = conflict
                            .iter()
                            .copied()
                            .max_by_key(|literal| {
                                self.trail.assignment[variable_name(*literal)].decision_level()
                            })
                            .unwrap();

                        let new_clause_id = self.backtrack_and_add_uip_clause(conflict, uip);

                        variable_index = self.trail.levels.last().unwrap().len();
                        self.propagate_literal(uip, new_clause_id);

                        continue 'unit_variables;
                    }
                    Unit(new_literal) => self.propagate_literal(new_literal, clause_id),
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

    pub fn solve(&mut self) -> bool {
        if !self.preprocess_clauses() {
            return false;
        }

        loop {
            if !self.process_unit_clauses() {
                return false;
            }

            match self
                .decision_heuristic
                .decide_literal(&self.formula, &self.trail)
            {
                None => {
                    return true;
                }
                Some(literal) => {
                    self.trail.decide_literal(literal);
                    self.conflict_analysis
                        .decide_literal(&self.formula, &self.trail, literal);
                }
            }
        }
    }
}

pub trait DecisionHeuristic {
    fn from_formula(n: usize, formula: &Formula) -> Self;

    fn backtrack_and_add_clause(
        &mut self,
        formula: &Formula,
        trail: &Trail,
        level: usize,
        clause_id: usize,
    );
    fn propagate_literal(
        &mut self,
        formula: &Formula,
        trail: &Trail,
        literal: Literal,
        reason_id: usize,
    );
    fn decide_literal(&mut self, formula: &Formula, trail: &Trail) -> Option<Literal>;
}

pub trait ConflictAnalysis {
    fn from_formula(n: usize, formula: &Formula) -> Self;

    fn analyze_conflict(&mut self, formula: &Formula, trail: &Trail, conflict: Clause) -> Clause;
    fn backtrack_and_add_clause(
        &mut self,
        formula: &Formula,
        trail: &Trail,
        level: usize,
        clause_id: usize,
    );
    fn propagate_literal(
        &mut self,
        formula: &Formula,
        trail: &Trail,
        literal: Literal,
        reason_id: usize,
    );
    fn decide_literal(&mut self, formula: &Formula, trail: &Trail, literal: Literal);
}
