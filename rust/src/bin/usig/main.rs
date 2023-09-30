use tracing::*;

use nom::AsChar;
use tracing_subscriber::field::debug;
use usig::*;

#[instrument]
fn main() {
    tracing_subscriber::fmt::init();
    let mut graph = GraphMap::<_, _, Undirected>::with_capacity(10, 10);
    graph.add_edge(1, 2, 1.225);
    graph.add_edge(1, 3, 1.0);
    graph.add_edge(2, 5, 2.236);
    graph.add_edge(3, 4, 1.0);
    debug!(graph = ?graph);
    debug!("Contains 1-2 edge ? {}", graph.contains_edge(1, 2));
    debug!(
        "Node 1 neighbors: {:?}",
        graph.neighbors(1).collect::<Vec<_>>()
    );
    debug!("bfs result: {:?}", graph.bfs(1, 5));
    debug!("dfs result: {:?}", graph.dfs(1, 5));
}
