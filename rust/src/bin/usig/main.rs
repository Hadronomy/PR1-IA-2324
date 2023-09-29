use tracing::*;

use tracing_subscriber::field::debug;
use usig::*;
use nom::AsChar;

#[instrument]
fn main() {
    tracing_subscriber::fmt::init();
    let mut graph = GraphMap::<_, _, Undirected>::with_capacity(10, 10);
    graph.add_edge(1, 2, 1);
    graph.add_edge(1, 3, 1);
    graph.add_edge(2, 3, 1);
    graph.add_edge(2, 4, 1);
    graph.add_edge(3, 4, 1);
    graph.add_edge(4, 5, 1);
    debug!(graph = ?graph);
    debug!("Contains 1-2 edge ? {}", graph.contains_edge(1, 2));
    debug!(
        "Node 1 neighbors: {:?}",
        graph.neighbors(1).collect::<Vec<_>>()
    );
    debug!(
        "bfs result: {:?}",
        graph.bfs(1, 5).iter().collect::<Vec<_>>()
    );
    debug!(
        "dfs result: {:?}",
        graph.dfs(1, 5).iter().collect::<Vec<_>>()
    )
}
