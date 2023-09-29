pub mod graph;
pub mod graphmap;

#[macro_use]
pub mod macros;

pub use crate::graph::*;
pub use crate::graphmap::*;
pub use crate::macros::*;

pub use crate::Direction::{Incoming, Outgoing};

#[derive(Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(usize)]
pub enum Direction {
    Outgoing = 0,
    Incoming = 1,
}

copyclone!(Direction);

impl Direction {
    #[inline]
    pub fn opposite(self) -> Direction {
        match self {
            Direction::Outgoing => Direction::Incoming,
            Direction::Incoming => Direction::Outgoing,
        }
    }

    #[inline]
    pub fn index(self) -> usize {
        self as usize
    }
}

pub use crate::Direction as EdgeDirection;

#[derive(Copy, Debug)]
pub enum Directed {}
copyclone!(Directed);

#[derive(Copy, Debug)]
pub enum Undirected {}
copyclone!(Undirected);

pub trait EdgeType {
    fn is_directed() -> bool;
}

impl EdgeType for Directed {
    #[inline]
    fn is_directed() -> bool {
        true
    }
}

impl EdgeType for Undirected {
    #[inline]
    fn is_directed() -> bool {
        false
    }
}
