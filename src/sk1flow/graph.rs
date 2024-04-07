pub trait Edge {
    fn to(&self) -> usize;
}

pub trait WeightedEdge<T>: Edge {
    fn weight(&self) -> &T;
    fn weight_mut(&mut self) -> &mut T;
}

impl<E: Edge, W> Edge for (E, W) {
    fn to(&self) -> usize {
        self.0.to()
    }
}

impl<E: Edge, W> WeightedEdge<W> for (E, W) {
    fn weight(&self) -> &W {
        &self.1
    }

    fn weight_mut(&mut self) -> &mut W {
        &mut self.1
    }
}

macro_rules! edge_impl {
        ($t:ty) => {
            impl Edge for $t {
                fn to(&self) -> usize {
                    *self as usize
                }
            }
        };
    }

edge_impl!(usize);
edge_impl!(isize);
edge_impl!(u8);
edge_impl!(i8);
edge_impl!(u16);
edge_impl!(i16);
edge_impl!(u32);
edge_impl!(i32);
edge_impl!(u64);
edge_impl!(i64);
edge_impl!(u128);
edge_impl!(i128);

fn make_rooted_dfs(tree: &mut Vec<Vec<impl Edge>>, v: usize, p: usize) {
    tree[v].retain(|val| val.to() != p);

    for i in 0..tree[v].len() {
        make_rooted_dfs(tree, tree[v][i].to(), v);
    }
}

#[allow(dead_code)]
pub fn make_rooted(tree: &mut Vec<Vec<impl Edge>>, root: usize) {
    make_rooted_dfs(tree, root, usize::MAX);
}
