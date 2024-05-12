use std::mem;

use num::Integer;

use crate::cdcl::propagation::{ConflictAnalysis, Trail};
use crate::cdcl::{variable_name, Clause, Formula, Literal};
use crate::sk1flow::graph::WeightedEdge;
use crate::sk1flow::SK1Flow;

pub trait MinCutWeight:
    Integer
    + Clone
    + Default
    + for<'a> std::ops::AddAssign<&'a Self>
    + for<'a> std::ops::SubAssign<&'a Self>
{
}

impl<
        W: Integer
            + Clone
            + Default
            + for<'a> std::ops::AddAssign<&'a Self>
            + for<'a> std::ops::SubAssign<&'a Self>,
    > MinCutWeight for W
{
}

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct MinCutConflict<W: MinCutWeight, T: WeightHeuristic<W>> {
    graph: Vec<Vec<(usize, W)>>,
    parents: Vec<Vec<usize>>,
    labels: Vec<Literal>,
    indices: Vec<usize>,
    weight_heuristic: T,
}

impl<W: MinCutWeight, T: WeightHeuristic<W>> MinCutConflict<W, T> {
    const SINK: usize = 1;
    const SOURCE: usize = 0;

    fn add_edge(&mut self, from: usize, to: usize) {
        self.graph[from].push((to, W::zero()));
        self.parents[to].push(from);
    }

    fn add_literal(&mut self, literal: Literal, reason: Option<&Clause>) {
        let in_id = self.graph.len();
        let out_id = in_id + 1;

        self.graph.push(vec![(out_id, W::zero())]);
        self.parents.push(vec![]);
        self.labels.push(literal);
        self.graph.push(vec![]);
        self.parents.push(vec![in_id]);
        self.labels.push(literal);

        self.indices[variable_name(literal)] = in_id;

        match reason {
            None => self.add_edge(Self::SOURCE, in_id),
            Some(clause) => {
                for v in clause
                    .iter()
                    .copied()
                    .filter(|l| *l != literal)
                    .map(|v| variable_name(v))
                {
                    self.add_edge(self.indices[v] + 1, in_id);
                }
            }
        }
    }

    fn pop_redundant_edges(&mut self, v: usize, trail: &Trail) {
        if v >= self.graph.len() {
            return;
        }

        while let Some((u, _w)) = self.graph[v].last() {
            if trail.assignment[variable_name(self.labels[*u])].is_unset() {
                self.graph[v].pop();
            } else {
                break;
            }
        }
    }

    fn pop_redundant_vertex(&mut self, trail: &Trail) {
        for v in self.parents.pop().unwrap() {
            self.pop_redundant_edges(v, trail);
        }
        self.graph.pop();
    }
}

impl<W: MinCutWeight, T: WeightHeuristic<W>> ConflictAnalysis for MinCutConflict<W, T> {
    fn from_formula(n: usize, formula: &Formula) -> Self {
        Self {
            graph: vec![vec![]; 2],
            parents: vec![vec![]; 2],
            labels: vec![0; 2],
            indices: vec![0; n],
            weight_heuristic: T::from_formula(n, formula),
        }
    }

    fn analyze_conflict(&mut self, _formula: &Formula, trail: &Trail, conflict: Clause) -> Clause {
        let total_levels = trail.levels.len();
        let conflict_level = total_levels - 1;

        for v in conflict.iter().map(|literal| variable_name(*literal)) {
            self.add_edge(self.indices[v] + 1, Self::SINK);
        }

        let mut excess = vec![W::zero(); self.graph.len()];
        excess[0] = self.weight_heuristic.source_excess(conflict_level); // infinity, informally

        // vertex weights
        for v in (2..self.graph.len()).step_by(2) {
            let variable = variable_name(self.labels[v]);
            let level = trail.assignment[variable].decision_level();

            debug_assert_eq!(self.graph[v].len(), 1);

            *self.graph[v][0].weight_mut() = self.weight_heuristic.gen_vertex_weight(
                level,
                conflict_level,
                trail.levels[level][0].0 == variable,
            );
        }

        // edges from source to decision variables, all infinite
        for edge in &mut self.graph[Self::SOURCE] {
            *edge.weight_mut() = excess[0].clone();
        }

        for v in (3..self.graph.len()).step_by(2) {
            for edge in &mut self.graph[v] {
                *edge.weight_mut() = excess[0].clone()
            }
        }

        let mut flow = SK1Flow::from_graph(&self.graph, excess);
        flow.flow(Self::SINK);
        let cut = flow.cut(Self::SINK);

        debug_assert!(cut[Self::SINK] && !cut[Self::SOURCE]);

        let mut clause = vec![];

        for v in (2..cut.len()).step_by(2) {
            if cut[v] != cut[v + 1] {
                debug_assert!(cut[v + 1]);

                clause.push(!self.labels[v]);
            }
        }

        debug_assert!(!clause.is_empty());

        clause
    }

    fn backtrack_and_add_clause(
        &mut self,
        _formula: &Formula,
        trail: &Trail,
        _level: usize,
        _clause_id: usize,
    ) {
        for v in mem::replace(&mut self.parents[Self::SINK], Vec::new()) {
            let (to, _w) = self.graph[v].pop().unwrap();

            debug_assert_eq!(to, Self::SINK)
        }

        while self.graph.len() > 2
            && trail.assignment[variable_name(self.labels[self.graph.len() - 1])].is_unset()
        {
            self.pop_redundant_vertex(trail);
        }
        self.labels.truncate(self.graph.len());
        self.parents.truncate(self.graph.len());

        debug_assert_eq!(self.graph.len(), self.parents.len());
        debug_assert_eq!(self.graph.len(), self.labels.len());
    }

    fn propagate_literal(
        &mut self,
        formula: &Formula,
        _trail: &Trail,
        literal: Literal,
        reason_id: usize,
    ) {
        self.add_literal(literal, Some(&formula[reason_id]));
    }

    fn decide_literal(&mut self, _formula: &Formula, _trail: &Trail, literal: Literal) {
        self.add_literal(literal, None);
    }
}

pub trait WeightHeuristic<W: Integer>: Default {
    fn from_formula(n: usize, formula: &Formula) -> Self;
    fn source_excess(&self, conflict_level: usize) -> W;
    fn gen_vertex_weight(&self, level: usize, conflict_level: usize, is_decision: bool) -> W;
}
