mod graph;

use graph::WeightedEdge;
use std::mem;
use std::ops::{AddAssign, SubAssign};

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct SK1Flow<T: Default + Clone + Ord>
    where
            for<'a> T: AddAssign<&'a T> + SubAssign<&'a T>,
{
    edges: Vec<(usize, T)>,
    graph: Vec<Vec<usize>>,
    excess: Vec<T>,
}

impl<T: Default + Clone + Ord> SK1Flow<T>
    where
            for<'a> T: AddAssign<&'a T> + SubAssign<&'a T>,
{
    pub fn new() -> Self {
        Self {
            edges: Vec::new(),
            graph: Vec::new(),
            excess: Vec::new(),
        }
    }

    pub fn from_graph(input_graph: &[Vec<impl WeightedEdge<T>>], excess: Vec<T>) -> Self {
        assert_eq!(input_graph.len(), excess.len());

        let mut ans = Self {
            edges: Vec::new(),
            graph: vec![vec![]; input_graph.len()],
            excess,
        };

        for v in 0..input_graph.len() {
            for edge in &input_graph[v] {
                ans.add_edge(v, edge.to(), edge.weight().clone());
            }
        }

        ans
    }

    pub fn add_vertex(&mut self, excess: T) {
        self.excess.push(excess);
        self.graph.push(vec![]);
    }

    pub fn add_double_edge(
        &mut self,
        from: usize,
        to: usize,
        capacity: T,
        reverse_capacity: T,
    ) {
        self.graph[from].push(self.edges.len());
        self.edges.push((to, capacity));
        self.graph[to].push(self.edges.len());
        self.edges.push((from, reverse_capacity));
    }

    pub fn add_edge(&mut self, from: usize, to: usize, capacity: T) {
        self.add_double_edge(from, to, capacity, T::default());
    }

    fn push_edge(edges: &mut Vec<(usize, T)>, excess: &mut Vec<T>, id: usize) -> T {
        let v = edges[id ^ 1].0;
        let u = edges[id].0;

        let value = (&excess[v]).min(&edges[id].1).clone();

        edges[id].1 -= &value;
        edges[id ^ 1].1 += &value;
        excess[v] -= &value;
        excess[u] += &value;

        value
    }

    pub fn flow(&mut self, sink: usize) -> T {
        loop {
            let mut order = vec![sink];
            let mut layers = vec![u32::MAX; self.excess.len()];
            layers[sink] = 1;
            let mut id = 0;

            while id < order.len() {
                let v = order[id];
                id += 1;

                for &edge_id in &self.graph[v] {
                    if self.edges[edge_id ^ 1].1 != Default::default() {
                        let u = self.edges[edge_id].0;

                        if layers[u] == u32::MAX {
                            layers[u] = layers[v] + 1;
                            order.push(u);
                        }
                    }
                }
            }

            let mut change = false;

            while let Some(v) = order.pop() {
                for &edge_id in &self.graph[v] {
                    if self.excess[v] == Default::default() {
                        break;
                    }

                    if layers[self.edges[edge_id].0] == layers[v] - 1 {
                        change |= Self::push_edge(&mut self.edges, &mut self.excess, edge_id)
                            != Default::default();
                    }
                }
            }

            if !change {
                break;
            }
        }

        mem::replace(&mut self.excess[sink], T::default())
    }

    pub fn edges(&self) -> &Vec<(usize, T)> {
        &self.edges
    }

    pub fn excess(&self) -> &Vec<T> {
        &self.excess
    }
}
