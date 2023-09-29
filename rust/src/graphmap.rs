//! TODO

use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt,
    hash::Hash,
    iter::Cloned,
    marker::PhantomData,
    slice::Iter,
};

use indexmap::{map::Keys, IndexMap};

use crate::{iterator_wrap, Direction, EdgeType, Incoming, Outgoing, Undirected};

/// A graph data structure.
/// This is a general purpose graph data structure that can be used to represent
/// both directed and undirected graphs.
#[derive(Clone)]
pub struct GraphMap<TNode, TEdge, Ty = Undirected> {
    nodes: IndexMap<TNode, Vec<(TNode, CompactDirection)>>,
    edges: IndexMap<(TNode, TNode), TEdge>,
    ty: PhantomData<Ty>,
}

impl<N: Eq + Hash + fmt::Debug, E: fmt::Debug, Ty: EdgeType> fmt::Debug for GraphMap<N, E, Ty> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.nodes.fmt(f)
    }
}

pub trait NodeTrait: Eq + Hash + Copy + Ord {}
impl<TNode> NodeTrait for TNode where TNode: Eq + Hash + Copy + Ord {}

impl<TNode, TEdge, Ty> GraphMap<TNode, TEdge, Ty>
where
    TNode: NodeTrait,
    Ty: EdgeType,
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
        if let old @ Some(_) = self.edges.insert((from, to), weight) {
            old
        } else {
            self.nodes
                .entry(from)
                .or_insert_with(|| Vec::with_capacity(1))
                .push((to, CompactDirection::Outgoing));
            if from != to {
                self.nodes
                    .entry(to)
                    .or_insert_with(|| Vec::with_capacity(1))
                    .push((from, CompactDirection::Incoming));
            }
            None
        }
    }

    pub fn remove_single_edge(&mut self, from: &TNode, to: &TNode, dir: CompactDirection) -> bool {
        todo!();
    }

    pub fn remove_edge(&mut self, from: &TNode, to: &TNode) -> Option<TEdge> {
        todo!();
    }

    pub fn contains_edge(&self, from: TNode, to: TNode) -> bool {
        self.edges.contains_key(&(from, to))
    }

    pub fn nodes(&self) -> Nodes<TNode> {
        Nodes {
            iter: self.nodes.keys().cloned(),
        }
    }

    pub fn neighbors(&self, node: TNode) -> Neighbors<TNode, Ty> {
        Neighbors {
            iter: self.nodes[&node].iter(),
            ty: PhantomData,
        }
    }

    pub fn bfs(&self, start: TNode, goal: TNode) -> HashMap<TNode, TNode> {
        let mut parents = HashMap::new();
        let mut queue = VecDeque::from(vec![start]);
        let mut seen = HashSet::new();
        seen.insert(start);
        while let Some(node) = queue.pop_front() {
            if node == goal {
                break;
            }
            for neighbor in self.neighbors(node) {
                if seen.insert(neighbor) {
                    parents.insert(neighbor, node);
                    queue.push_back(neighbor);
                }
            }
        }
        parents
    }

    pub fn dfs(&self, start: TNode, goal: TNode) -> HashMap<TNode, TNode> {
        let mut parents = HashMap::new();
        let mut stack = vec![start];
        let mut seen = HashSet::new();
        seen.insert(start);
        while let Some(node) = stack.pop() {
            if node == goal {
                break;
            }
            for neighbor in self.neighbors(node) {
                if seen.insert(neighbor) {
                    parents.insert(neighbor, node);
                    stack.push(neighbor);
                }
            }
        }
        parents
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
            ty: PhantomData,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

impl From<CompactDirection> for Direction {
    fn from(dir: CompactDirection) -> Self {
        match dir {
            CompactDirection::Outgoing => Outgoing,
            CompactDirection::Incoming => Incoming,
        }
    }
}

impl From<Direction> for CompactDirection {
    fn from(dir: Direction) -> Self {
        match dir {
            Outgoing => CompactDirection::Outgoing,
            Incoming => CompactDirection::Incoming,
        }
    }
}

impl PartialEq<Direction> for CompactDirection {
    fn eq(&self, rhs: &Direction) -> bool {
        (*self as usize) == (*rhs as usize)
    }
}

iterator_wrap!(
    impl (Iterator DoubleEndedIterator ExactSizeIterator) for
    #[derive(Clone, Debug)]
    struct Nodes<'a, TNode> where { TNode: 'a + NodeTrait }
    item: TNode,
    iter: Cloned<Keys<'a, TNode, Vec<(TNode, CompactDirection)>>>,
);

#[derive(Clone, Debug)]
pub struct Neighbors<'a, TNode, Ty = Undirected>
where
    TNode: 'a,
    Ty: EdgeType,
{
    iter: Iter<'a, (TNode, CompactDirection)>,
    ty: PhantomData<Ty>,
}

impl<'a, TNode, Ty> Iterator for Neighbors<'a, TNode, Ty>
where
    TNode: NodeTrait,
    Ty: EdgeType,
{
    type Item = TNode;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if Ty::is_directed() {
            (&mut self.iter)
                .filter_map(|&(n, dir)| if dir == Outgoing { Some(n) } else { None })
                .next()
        } else {
            self.iter.next().map(|&(n, _)| n)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.iter.size_hint();
        if Ty::is_directed() {
            (0, upper)
        } else {
            (lower, upper)
        }
    }
}
