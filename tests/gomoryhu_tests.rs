extern crate network_flow_project;
use network_flow_project::{GomoryHuTree};

#[test]
fn test_add_edge() {
    let mut ght = GomoryHuTree::new(4);
    ght.add_edge(0, 1, 10);
    ght.add_edge(1, 2, 5);
    ght.add_edge(2, 3, 15);

    assert_eq!(ght.graph[0][0].to, 1);
    assert_eq!(ght.graph[0][0].cap, 10);
    assert_eq!(ght.graph[1][0].to, 0);
    assert_eq!(ght.graph[1][0].cap, 0);

    assert_eq!(ght.graph[1][1].to, 2);
    assert_eq!(ght.graph[1][1].cap, 5);
    assert_eq!(ght.graph[2][0].to, 1);
    assert_eq!(ght.graph[2][0].cap, 0);

    assert_eq!(ght.graph[2][1].to, 3);
    assert_eq!(ght.graph[2][1].cap, 15);
    assert_eq!(ght.graph[3][0].to, 2);
    assert_eq!(ght.graph[3][0].cap, 0);
}

#[test]
fn test_tree_structure() {
    let mut ght = GomoryHuTree::new(4);
    ght.add_edge(0, 1, 10);
    ght.add_edge(1, 2, 5);
    ght.add_edge(2, 3, 15);

    ght.build_tree();
    let tree = ght.get_tree();

    assert_eq!(tree.len(), 4); // Tree should have n edges
    assert!(tree.iter().any(|e| e.u == 1 && e.v == 0));
    assert!(tree.iter().any(|e| e.u == 2 && e.v == 1));
    assert!(tree.iter().any(|e| e.u == 3 && e.v == 2));
}