//! TODO

use std::{collections::HashMap, marker::PhantomData};

pub type Directed = ();
pub type Undirected = ();

#[derive(Clone, Debug)]
pub struct GraphMap<N, E, Ty = Undirected> {
    nodes: HashMap<N, Vec<(N, CompactDirection)>>,
    edges: HashMap<(N, N), E>,
    ty: PhantomData<Ty>,
}

#[derive(Clone, Debug)]
enum CompactDirection {
    Outgoing,
    Incoming,
}

impl CompactDirection {
    /// Return the opposite `CompactDirection`.
    #[inline]
    pub fn opposite(self) -> CompactDirection {
        match self {
            CompactDirection::Outgoing => CompactDirection::Incoming,
            CompactDirection::Incoming => CompactDirection::Outgoing,
        }
    }
}

impl<N, E, Ty> GraphMap<N, E, Ty>
where
    N: Eq + std::hash::Hash + Copy + Ord,
{
    pub fn new() -> Self {
        todo!();
    }

    pub fn with_capacity(nodes: usize, edges: usize) -> Self {
        GraphMap {
            nodes: HashMap::with_capacity(nodes),
            edges: HashMap::with_capacity(edges),
            ty: PhantomData,
        }
    }

    pub fn capacity(&self) -> (usize, usize) {
        (self.nodes.capacity(), self.edges.capacity())
    }

    pub fn add_node(&mut self, node: N) -> N {
        self.nodes.entry(node).or_insert(Vec::new());
        node
    }
}
