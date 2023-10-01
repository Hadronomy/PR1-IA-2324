mod parser;

use std::{io::Write, path::PathBuf};

use comfy_table::Table;
use tracing::*;

use crate::parser::*;
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    input: PathBuf,
    output: PathBuf,
    starting_node: usize,
    ending_node: usize,
    #[arg(short = 'a', long, value_enum, default_value = "bfs")]
    algorithm: Algorithms,
}

#[derive(ValueEnum, Debug, Clone)]
#[value()]
enum Algorithms {
    #[value()]
    Bfs,
    #[value()]
    Dfs,
}

#[instrument]
fn main() {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    debug!(?args);
    let input = std::fs::read_to_string(args.input).unwrap();
    let (_, graph) = parse_graph(&input).unwrap();
    let result = match args.algorithm {
        Algorithms::Bfs => graph.bfs(args.starting_node, args.ending_node),
        Algorithms::Dfs => graph.dfs(args.starting_node, args.ending_node),
    };
    let output = std::fs::File::create(args.output).unwrap();
    let mut writer = std::io::BufWriter::new(output);
    let mut table = Table::new();
    table.set_header(vec![
        "Node Count (n)",
        "Edge Count (m)",
        "Starting Node (v0)",
        "Ending Node (vd)",
        "Path",
        "Distance",
        "Generated Nodes",
        "Expanded Nodes",
    ]);
    table.add_row(vec![
        graph.node_count().to_string(),
        graph.edge_count().to_string(),
        args.starting_node.to_string(),
        args.ending_node.to_string(),
        result.str_path(args.starting_node, args.ending_node),
        result.distance.to_string(),
        format!("{:?}", result.generated_nodes),
        format!("{:?}", result.expanded_nodes),
    ]);
    writer.write_all(format!("{table}").as_bytes()).unwrap();
    debug!(result = ?result);
}
