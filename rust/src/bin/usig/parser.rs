use nom::{
    character::complete::{digit1, line_ending},
    combinator::{map, map_res, opt, recognize},
    multi::many_m_n,
    number::complete::double,
    sequence::delimited,
    IResult,
};
use tracing::{debug, trace};
use usig::*;

fn usize(input: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), str::parse)(input)
}

pub fn parse_graph(input: &str) -> IResult<&str, GraphMap<usize, f64, Undirected>> {
    let (input, nodes) = usize(input).expect("Failed to parse nodes");
    let expected_size = ((nodes as f64 / 2.0) * (nodes as f64 - 1.0)) as usize;
    debug!(?nodes, ?expected_size);
    map(
        many_m_n(
            expected_size,
            expected_size,
            delimited(opt(line_ending), double, opt(line_ending)),
        ),
        move |edges| {
            let mut iter = edges.iter();
            let mut graph = GraphMap::<usize, f64, Undirected>::with_capacity(nodes, edges.len());
            debug!("Number of parsed edges: {}", edges.len());
            for i in 1..nodes {
                for j in i + 1..=nodes {
                    trace!("{} - {}", i, j);
                    let weight = iter.next().expect("failed to get next edge weight");
                    if (weight - -1.0).abs() < f64::EPSILON {
                        continue;
                    }
                    graph.add_edge(i, j, *weight);
                }
            }
            graph
        },
    )(input)
}
