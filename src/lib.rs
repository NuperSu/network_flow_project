use std::cmp;
use std::collections::VecDeque;

#[derive(Clone)]
pub struct Edge {
    to: usize,
    rev: usize,
    cap: i64,
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
