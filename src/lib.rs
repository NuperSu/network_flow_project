use std::cmp;
use std::collections::VecDeque;

pub struct Edge {
    to: usize,
    rev: usize,
    cap: i64,
}

pub struct Graph {
    graph: Vec<Vec<Edge>>,
}

pub struct Dinic {
    graph: Graph,
    level: Vec<i32>,
    iter: Vec<usize>,
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