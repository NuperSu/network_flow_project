use std::cmp;
use std::collections::VecDeque;

#[derive(Clone)]
pub struct Edge {
    pub to: usize,
    pub rev: usize,
    pub cap: i64,
}

#[derive(Clone)]
pub struct Graph {
    graph: Vec<Vec<Edge>>,
}

pub struct Dinic {
    graph: Graph,
    level: Vec<i32>,
    iter: Vec<usize>,
}

pub struct EdmondsKarp {
    graph: Graph,
    parent: Vec<isize>,
}

#[derive(Clone, Debug)]
pub struct TreeEdge {
    pub u: usize,
    pub v: usize,
    pub w: i64,
}

#[derive(Clone)]
pub struct FlowEdge {
    pub to: usize,
    pub rev: usize,
    pub cap: i64,
}

pub struct GomoryHuTree {
    pub graph: Vec<Vec<FlowEdge>>,
    parent: Vec<usize>,
    weight: Vec<i64>,
    visited: Vec<bool>,
}

impl Graph {
    pub fn new(v: usize) -> Self {
        let graph = (0..v).map(|_| Vec::new()).collect::<Vec<_>>();
        Graph { graph }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cap: i64) {
        let rev_to = self.graph[to].len();
        let rev_from = self.graph[from].len();
        self.graph[from].push(Edge {
            to,
            rev: rev_to,
            cap,
        });
        self.graph[to].push(Edge {
            to: from,
            rev: rev_from,
            cap: 0,
        });
    }

    pub fn clear(&mut self) {
        for edges in self.graph.iter_mut() {
            for edge in edges.iter_mut() {
                edge.cap = 0;
            }
        }
    }
}

impl Dinic {
    pub fn new(v: usize) -> Self {
        let graph = Graph::new(v);
        let level = vec![0; v];
        let iter = vec![0; v];
        Dinic { graph, level, iter }
    }

    pub fn from_graph(graph: Graph) -> Self {
        let v = graph.graph.len();
        let level = vec![0; v];
        let iter = vec![0; v];
        Dinic { graph, level, iter }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cap: i64) {
        self.graph.add_edge(from, to, cap);
    }

    fn bfs(&mut self, s: usize) {
        self.level.fill(-1);
        let mut deque = VecDeque::new();
        self.level[s] = 0;
        deque.push_back(s);
        while let Some(v) = deque.pop_front() {
            for e in &self.graph.graph[v] {
                if e.cap > 0 && self.level[e.to] < 0 {
                    self.level[e.to] = self.level[v] + 1;
                    deque.push_back(e.to);
                }
            }
        }
    }

    fn dfs(&mut self, v: usize, t: usize, f: i64) -> i64 {
        if v == t {
            return f;
        }
        while self.iter[v] < self.graph.graph[v].len() {
            let (to, cap, rev) = {
                let e = &self.graph.graph[v][self.iter[v]];
                (e.to, e.cap, e.rev)
            };

            if cap > 0 && self.level[v] < self.level[to] {
                let d = self.dfs(to, t, cmp::min(f, cap));
                if d > 0 {
                    let e = &mut self.graph.graph[v][self.iter[v]];
                    e.cap -= d;
                    self.graph.graph[to][rev].cap += d;
                    return d;
                }
            }
            self.iter[v] += 1;
        }
        0
    }

    pub fn max_flow(&mut self, s: usize, t: usize) -> i64 {
        let mut flow = 0;
        loop {
            self.bfs(s);
            if self.level[t] < 0 {
                return flow;
            }
            self.iter.fill(0);
            while let Some(fl) = Some(self.dfs(s, t, std::i64::MAX)) {
                if fl == 0 {
                    break;
                }
                flow += fl;
            }
        }
    }
}

impl EdmondsKarp {
    pub fn new(v: usize) -> Self {
        let graph = Graph::new(v);
        let parent = vec![-1; v];
        EdmondsKarp { graph, parent }
    }

    pub fn from_graph(graph: Graph) -> Self {
        let v = graph.graph.len();
        let parent = vec![-1; v];
        EdmondsKarp { graph, parent }
    }

    fn bfs(&mut self, s: usize, t: usize) -> bool {
        self.parent.fill(-1);
        let mut queue = VecDeque::new();
        queue.push_back(s);
        self.parent[s] = -2; // Mark the source vertex

        while let Some(u) = queue.pop_front() {
            for (_, e) in self.graph.graph[u].iter().enumerate() {
                if e.cap > 0 && self.parent[e.to] == -1 {
                    self.parent[e.to] = u as isize; // Store the path
                    if e.to == t {
                        return true; // Reached the sink
                    }
                    queue.push_back(e.to);
                }
            }
        }

        false
    }

    pub fn max_flow(&mut self, s: usize, t: usize) -> i64 {
        let mut flow = 0;

        while self.bfs(s, t) {
            // While there is an augmenting path
            let mut path_flow = i64::MAX;

            // Find the maximum flow through the path found.
            let mut current = t;
            while current != s {
                let prev = self.parent[current] as usize;
                let e = &self.graph.graph[prev]
                    .iter()
                    .find(|e| e.to == current)
                    .unwrap();
                path_flow = cmp::min(path_flow, e.cap);
                current = prev;
            }

            // Update the capacities and the reverse edges along the path
            current = t;
            while current != s {
                let prev = self.parent[current] as usize;

                if let Some(edge) = self.graph.graph[prev].iter_mut().find(|e| e.to == current) {
                    edge.cap -= path_flow;
                }

                let rev_edge_index = self.graph.graph[prev]
                    .iter()
                    .find(|e| e.to == current)
                    .map(|e| e.rev)
                    .unwrap_or(0);

                if let Some(rev_edge) = self.graph.graph[current].get_mut(rev_edge_index) {
                    rev_edge.cap += path_flow;
                }

                current = prev;
            }

            flow += path_flow;
        }

        flow
    }
}

impl GomoryHuTree {
    pub fn new(n: usize) -> Self {
        GomoryHuTree {
            graph: vec![Vec::new(); n],
            parent: vec![0; n],
            weight: vec![0; n],
            visited: vec![false; n],
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize, w: i64) {
        let rev_u = self.graph[v].len();
        let rev_v = self.graph[u].len();
        self.graph[u].push(FlowEdge { to: v, rev: rev_u, cap: w });
        self.graph[v].push(FlowEdge { to: u, rev: rev_v, cap: 0 }); // reverse edge with 0 capacity
    }

    pub fn build_tree(&mut self) {
        for u in 1..self.graph.len() {
            let mut graph = Graph::new(self.graph.len());

            for (from, edges) in self.graph.iter().enumerate() {
                for edge in edges {
                    graph.add_edge(from, edge.to, edge.cap);
                }
            }

            let mut dinic = Dinic::from_graph(graph);

            self.weight[u] = dinic.max_flow(u, self.parent[u]);
            self.visited.iter_mut().for_each(|v| *v = false);
            self.dfs(u, &dinic.graph);

            for v in u + 1..self.graph.len() {
                if self.visited[v] && self.parent[v] == self.parent[u] {
                    self.parent[v] = u;
                }
            }

            if self.parent[self.parent[u]] != usize::MAX && self.visited[self.parent[self.parent[u]]] {
                let pu = self.parent[u];
                self.weight[u] = self.weight[pu];
                self.parent[u] = self.parent[pu];
                self.parent[pu] = u;
            }
        }
    }

    fn dfs(&mut self, u: usize, graph: &Graph) {
        self.visited[u] = true;
        for e in &graph.graph[u] {
            if e.cap > 0 && !self.visited[e.to] {
                self.dfs(e.to, graph);
            }
        }
    }

    pub fn get_tree(&self) -> Vec<TreeEdge> {
        let mut tree = Vec::new();
        for (i, &p) in self.parent.iter().enumerate() {
            if p != usize::MAX {
                tree.push(TreeEdge {
                    u: i,
                    v: p,
                    w: self.weight[i],
                });
            }
        }
        tree
    }

    pub fn from_graph(graph: &Graph) -> Self {
        let n = graph.graph.len();
        let mut gomory_hu_tree = GomoryHuTree {
            graph: vec![Vec::new(); n],
            parent: vec![0; n],
            weight: vec![0; n],
            visited: vec![false; n],
        };

        // Convert the Graph to the format needed for GomoryHuTree.
        for (from, edges) in graph.graph.iter().enumerate() {
            for edge in edges {
                if edge.cap > 0 {
                    // Add edges with positive capacity to the GomoryHuTree graph.
                    gomory_hu_tree.graph[from].push(FlowEdge {
                        to: edge.to,
                        rev: edge.rev,
                        cap: edge.cap,
                    });
                }
            }
        }

        gomory_hu_tree
    }
}
