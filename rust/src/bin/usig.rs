use tracing::*;

use usig::*;

#[instrument]
fn main() {
    tracing_subscriber::fmt::init();
    let mut graph = GraphMap::<u8, i32, Undirected>::with_capacity(10, 10);
    graph.add_node(1);
    debug!(graph = ?graph);
}
