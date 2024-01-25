use network_flow_project::{Dinic, Graph};
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let v_e: Vec<usize> = input
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();
    let (v, e) = (v_e[0], v_e[1]);

    let mut graph = Graph::new(v);

    for _ in 0..e {
        let mut edge_input = String::new();
        io::stdin().read_line(&mut edge_input).unwrap();
        let edge_data: Vec<i64> = edge_input
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        graph.add_edge(edge_data[0] as usize, edge_data[1] as usize, edge_data[2]);
    }

    let mut dinic = Dinic::from_graph(graph);

    let mut source_sink_input = String::new();
    io::stdin().read_line(&mut source_sink_input).unwrap();
    let source_sink: Vec<usize> = source_sink_input
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();
    let (source, sink) = (source_sink[0], source_sink[1]);

    println!("Maximum flow: {}", dinic.max_flow(source, sink));
}
