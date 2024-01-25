extern crate network_flow_project;
use network_flow_project::{Dinic, Graph};

#[test]
fn test_basic() {
    let mut graph = Graph::new(4);
    graph.add_edge(0, 1, 10);
    graph.add_edge(1, 3, 5);
    graph.add_edge(0, 2, 10);
    graph.add_edge(2, 3, 10);

    let mut dinic = Dinic::from_graph(graph);
    assert_eq!(dinic.max_flow(0, 3), 15);
}

#[test]
fn test_no_path() {
    let mut graph = Graph::new(4);
    graph.add_edge(0, 1, 10);
    graph.add_edge(1, 2, 0); // no capacity
    graph.add_edge(2, 3, 10);

    let mut dinic = Dinic::from_graph(graph);
    assert_eq!(dinic.max_flow(0, 3), 0);
}

#[test]
fn test_multiple_paths() {
    let mut graph = Graph::new(6);
    graph.add_edge(0, 1, 16);
    graph.add_edge(0, 2, 13);
    graph.add_edge(1, 2, 10);
    graph.add_edge(1, 3, 12);
    graph.add_edge(2, 1, 4);
    graph.add_edge(2, 4, 14);
    graph.add_edge(3, 2, 9);
    graph.add_edge(3, 5, 20);
    graph.add_edge(4, 3, 7);
    graph.add_edge(4, 5, 4);

    let mut dinic = Dinic::from_graph(graph);
    assert_eq!(dinic.max_flow(0, 5), 23);
}

#[test]
fn test_disconnected_graph() {
    let mut graph = Graph::new(4);
    graph.add_edge(0, 1, 10);
    // No edge between 1 and 2 or 2 and 3

    let mut dinic = Dinic::from_graph(graph);
    assert_eq!(dinic.max_flow(0, 3), 0);
}

#[test]
fn test_large_capacity() {
    let mut graph = Graph::new(3);
    graph.add_edge(0, 1, i64::MAX);
    graph.add_edge(1, 2, i64::MAX);

    let mut dinic = Dinic::from_graph(graph);
    assert_eq!(dinic.max_flow(0, 2), i64::MAX);
}

#[test]
fn test_big_graph() {
    let graph_size = 5000;
    let mut graph = Graph::new(graph_size);
    let max_capacity: i64 = 1000000;

    for i in 0..graph_size-1 {
        graph.add_edge(i, i + 1, max_capacity);
    }

    for i in 0..graph_size-2 {
        graph.add_edge(i, i + 2, max_capacity / 2);
    }

    let mut dinic = Dinic::from_graph(graph);
    assert_eq!(dinic.max_flow(0, graph_size-1), max_capacity + max_capacity / 2);
}
