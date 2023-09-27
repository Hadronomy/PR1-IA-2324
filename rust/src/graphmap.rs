//! TODO

use std::{hash::Hash, marker::PhantomData};

use indexmap::IndexMap;

pub type Directed = ();
pub type Undirected = ();

/// A graph data structure.
/// This is a general purpose graph data structure that can be used to represent
/// both directed and undirected graphs.
#[derive(Clone, Debug)]
pub struct GraphMap<TNode, TEdge, Ty = Undirected> {
    nodes: IndexMap<TNode, Vec<(TNode, CompactDirection)>>,
    edges: IndexMap<(TNode, TNode), TEdge>,
    ty: PhantomData<Ty>,
}

impl<TNode, TEdge, Ty> GraphMap<TNode, TEdge, Ty>
where
    TNode: Eq + Hash + Copy + Ord,
{
    /// Creates a new [`GraphMap<TNode, TEdge, Ty>`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new [`GraphMap<TNode, TEdge, Ty>`] with the specified capacity.
    pub fn with_capacity(nodes: usize, edges: usize) -> Self {
        GraphMap {
            nodes: IndexMap::with_capacity(nodes.next_power_of_two()),
            edges: IndexMap::with_capacity(edges.next_power_of_two()),
            ty: PhantomData,
        }
    }

    /// Returns the capacity of the graph.
    pub fn capacity(&self) -> (usize, usize) {
        (self.nodes.capacity(), self.edges.capacity())
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
    }

    /// Adds a node to the graph.
    pub fn add_node(&mut self, node: TNode) -> TNode {
        self.nodes.entry(node).or_insert(Vec::new());
        node
    }

    /// Removes a node from the graph.
    pub fn remove_node(&mut self, node: TNode) -> bool {
        let links = match self.nodes.swap_remove(&node) {
            Some(links) => links,
            None => return false,
        };
        for (succ, dir) in links {
            let edge = if dir == CompactDirection::Outgoing {
                todo!("remove edge");
                // (node, succ)
            } else {
                todo!("remove edge");
                // (succ, node)
            };
        }
        true
    }

    pub fn contains_node(&self, node: TNode) -> bool {
        self.nodes.contains_key(&node)
    }

    pub fn add_edge(&mut self, from: TNode, to: TNode, weight: TEdge) -> Option<TEdge> {
        todo!();
    }

    pub fn remove_single_edge(&mut self, from: &TNode, to: &TNode, dir: CompactDirection) -> bool {
        todo!();
    }

    pub fn remove_edge(&mut self, from: &TNode, to: &TNode) -> Option<TEdge> {
        todo!();
    }
}

impl<TNode, TEdge, Ty> Default for GraphMap<TNode, TEdge, Ty>
where
    TNode: Eq + Hash + Copy + Ord,
{
    /// Creates an empty [`GraphMap<TNode, TEdge, Ty>`].
    fn default() -> Self {
        Self {
            nodes: IndexMap::new(),
            edges: IndexMap::new(),
            ty: PhantomData::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CompactDirection {
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
