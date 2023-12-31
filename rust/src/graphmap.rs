//! Based of petgraph implementation.
//!
//! See https://github.com/petgraph/petgraph/blob/master/src/graph_impl/mod.rs

use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt,
    hash::Hash,
    iter::Cloned,
    marker::PhantomData,
    slice::Iter,
};

use indexmap::{map::Keys, IndexMap};
use tracing::info;

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

pub trait NodeTrait: Eq + Hash + Copy + Ord + PartialEq {}
impl<TNode> NodeTrait for TNode where TNode: Eq + Hash + Copy + Ord + PartialEq {}

impl<TNode, TEdge, Ty> GraphMap<TNode, TEdge, Ty>
where
    TNode: NodeTrait + std::fmt::Debug,
    TEdge: Default + Copy + std::ops::Add<Output = TEdge> + std::fmt::Debug,
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

    pub fn get_edge(&self, from: TNode, to: TNode) -> Option<&TEdge> {
        // TODO: Check for directionality
        if let value @ Some(_) = self.edges.get(&(from, to)) {
            value
        } else {
            self.edges.get(&(to, from))
        }
    }

    pub fn nodes(&self) -> Nodes<TNode> {
        Nodes {
            iter: self.nodes.keys().cloned(),
        }
    }

    /// Returns the neighbors of a given node in the graph.
    ///
    /// # Arguments
    ///
    /// * `node` - A node in the graph.
    ///
    /// # Returns
    ///
    /// A `Neighbors` struct containing the neighbors of the given node and their edge weights.
    pub fn neighbors(&self, node: TNode) -> Neighbors<TNode, Ty> {
        Neighbors {
            iter: self.nodes[&node].iter(),
            ty: PhantomData,
        }
    }

    pub fn path_cost(&self, tree: HashMap<TNode, TNode>, goal: TNode) -> TEdge {
        let mut acc = TEdge::default();
        let mut next: Option<TNode> = Some(goal);
        while let Some(node) = next {
            if let Some(parent) = tree.get(&node) {
                acc = acc + *self.get_edge(*parent, node).unwrap();
                next = Some(*parent);
            } else {
                break;
            }
        }
        acc
    }

    fn min_max_node(mut nodes: Vec<TNode>) -> (TNode, TNode) {
        nodes.sort();
        let min = nodes[0];
        let max = nodes[nodes.len() - 1];
        (min, max)
    }

    /// Performs a breadth-first search on the graph, starting from the given start node and
    /// searching for the given goal node.
    ///
    /// # Arguments
    ///
    /// * `start` - The starting node for the search.
    /// * `goal` - The goal node to search for.
    ///
    /// # Returns
    ///
    /// A [`GraphSearchReport`] struct containing information about the search, including the path
    /// from the start node to the goal node (if one was found), the number of nodes visited, and
    /// the number of edges traversed.
    pub fn bfs(&self, start: TNode, goal: TNode) -> GraphSearchReport<TNode, TEdge> {
        let mut parents = HashMap::new();
        let mut queue = vec![start];
        let mut seen = HashSet::new();
        let mut generated = Vec::new();
        let mut expanded = Vec::new();
        let mut acc = TEdge::default();
        seen.insert(start);
        generated.push(start);
        while !queue.is_empty() {
            let node = {
                let mut min = None;
                let mut max = None;
                for (i, node) in queue.iter().enumerate() {
                    if min.is_none() || node < queue.get(min.unwrap()).unwrap() {
                        min = Some(i);
                    }
                    if max.is_none() || node > queue.get(max.unwrap()).unwrap() {
                        max = Some(i);
                    }
                }
                // choose min or max randomly
                let chosen_index = if rand::random() {
                    min.unwrap()
                } else {
                    max.unwrap()
                };
                let copy = *queue.get(chosen_index).unwrap();
                queue.swap_remove(chosen_index);
                copy
            };
            expanded.push(node);
            if node == goal {
                break;
            }
            for neighbor in self.neighbors(node) {
                generated.push(neighbor);
                if seen.insert(neighbor) {
                    acc = acc + *self.get_edge(node, neighbor).unwrap();
                    parents.insert(neighbor, node);
                    queue.push(neighbor);
                }
            }
        }
        if parents.get(&goal).is_none() {
            return GraphSearchReport {
                path: None,
                distance: None,
                generated_nodes: generated,
                expanded_nodes: expanded,
            };
        }
        GraphSearchReport {
            path: Some(parents.clone()),
            distance: Some(self.path_cost(parents, goal)),
            generated_nodes: generated,
            expanded_nodes: expanded,
        }
    }

    pub fn dfs(&self, start: TNode, goal: TNode) -> GraphSearchReport<TNode, TEdge> {
        let mut parents = HashMap::new();
        let mut stack = vec![start];
        let mut seen = HashSet::new();
        let mut generated = Vec::new();
        let mut expanded = Vec::new();
        seen.insert(start);
        generated.push(start);
        while let Some(node) = stack.pop() {
            expanded.push(node);
            if node == goal {
                break;
            }
            for neighbor in self.neighbors(node) {
                generated.push(neighbor);
                if seen.insert(neighbor) {
                    parents.insert(neighbor, node);
                    stack.push(neighbor);
                }
            }
        }
        if parents.get(&goal).is_none() {
            return GraphSearchReport {
                path: None,
                distance: None,
                generated_nodes: generated,
                expanded_nodes: expanded,
            };
        }
        GraphSearchReport {
            path: Some(parents.clone()),
            distance: Some(self.path_cost(parents, goal)),
            generated_nodes: generated,
            expanded_nodes: expanded,
        }
    }
}

impl<TNode, TEdge, Ty> Default for GraphMap<TNode, TEdge, Ty>
where
    TNode: NodeTrait,
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
#[derive(Clone, Debug)]
pub struct GraphSearchReport<TNode, TEdge> {
    pub path: Option<HashMap<TNode, TNode>>,
    pub distance: Option<TEdge>,
    pub generated_nodes: Vec<TNode>,
    pub expanded_nodes: Vec<TNode>,
}

impl<TNode, TEdge> GraphSearchReport<TNode, TEdge> {
    pub fn str_path(&self, start: TNode, goal: TNode) -> String
    where
        TNode: NodeTrait + std::fmt::Display,
    {
        let mut acc = String::new();
        let path = match &self.path {
            Some(path) => path,
            None => return acc,
        };
        let mut next: Option<TNode> = Some(goal);
        while let Some(node) = next {
            if let Some(parent) = path.get(&node) {
                match acc.len() {
                    0 => acc = format!("{}", node),
                    _ => acc = format!("{} -> {}", node, acc),
                }
                next = Some(*parent);
            } else {
                break;
            }
        }
        format!("{} -> {}", start, acc)
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
