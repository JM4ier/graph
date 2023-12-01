use std::{
    collections::VecDeque,
    ops::{Index, IndexMut},
};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct NodeRef {
    idx: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct EdgeRef {
    idx: usize,
}

struct AdjEntry {
    node: NodeRef,
    edge: EdgeRef,
}

struct IncEntry {
    from: NodeRef,
    to: NodeRef,
}

pub struct Graph<N, E> {
    nodes: Vec<N>,
    edges: Vec<E>,
    adjacency: Vec<Vec<AdjEntry>>,
    incidence: Vec<IncEntry>,
}

impl<N, E> Default for Graph<N, E> {
    fn default() -> Self {
        Self {
            nodes: Default::default(),
            edges: Default::default(),
            adjacency: Default::default(),
            incidence: Default::default(),
        }
    }
}

impl<N, E> Graph<N, E> {
    /// Constructs an empty graph
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn node(&mut self, n: N) -> NodeRef {
        let r = NodeRef {
            idx: self.nodes.len(),
        };
        self.nodes.push(n);
        self.adjacency.push(Vec::new());
        r
    }

    pub fn directed_edge(&mut self, from: NodeRef, to: NodeRef, e: E) -> EdgeRef {
        let r = EdgeRef {
            idx: self.edges.len(),
        };
        self.edges.push(e);
        self.adjacency[from.idx].push(AdjEntry { node: to, edge: r });
        self.incidence.push(IncEntry { from, to });
        r
    }
}

impl<N, E: Clone> Graph<N, E> {
    pub fn undirected_edge(&mut self, a: NodeRef, b: NodeRef, e: E) -> [EdgeRef; 2] {
        let r1 = self.directed_edge(a, b, e.clone());
        let r2 = self.directed_edge(b, a, e);
        [r1, r2]
    }
}

impl<N, E> Index<NodeRef> for Graph<N, E> {
    type Output = N;

    fn index(&self, index: NodeRef) -> &Self::Output {
        &self.nodes[index.idx]
    }
}

impl<N, E> IndexMut<NodeRef> for Graph<N, E> {
    fn index_mut(&mut self, index: NodeRef) -> &mut Self::Output {
        &mut self.nodes[index.idx]
    }
}

impl<N, E> Index<EdgeRef> for Graph<N, E> {
    type Output = E;

    fn index(&self, index: EdgeRef) -> &Self::Output {
        &self.edges[index.idx]
    }
}

impl<N, E> IndexMut<EdgeRef> for Graph<N, E> {
    fn index_mut(&mut self, index: EdgeRef) -> &mut Self::Output {
        &mut self.edges[index.idx]
    }
}

impl<N, E> Graph<N, E> {
    /// Most general DFS implementation
    pub fn visit_dfs(
        &self,
        begin: NodeRef,
        mut node_visitor: impl FnMut(NodeRef, &N),
        mut edge_visitor: impl FnMut(EdgeRef, &E),
    ) {
        fn dfs<N, E>(
            graph: &Graph<N, E>,
            visited: &mut [bool],
            node: usize,
            node_visitor: &mut dyn FnMut(NodeRef, &N),
            edge_visitor: &mut dyn FnMut(EdgeRef, &E),
        ) {
            if visited[node] {
                return;
            }

            node_visitor(NodeRef { idx: node }, &graph.nodes[node]);
            visited[node] = true;

            for adj in &graph.adjacency[node] {
                edge_visitor(adj.edge, &graph[adj.edge]);
                dfs(graph, visited, adj.node.idx, node_visitor, edge_visitor);
            }
        }

        let mut visited = vec![false; self.nodes.len()];
        dfs(
            self,
            &mut visited,
            begin.idx,
            &mut node_visitor,
            &mut edge_visitor,
        )
    }

    /// DFS to visit nodes
    pub fn visit_dfs_nodes(&self, begin: NodeRef, node_visitor: impl FnMut(NodeRef, &N)) {
        self.visit_dfs(begin, node_visitor, |_, _| ())
    }

    /// DFS to visit edges
    pub fn visit_dfs_edges(&self, begin: NodeRef, edge_visitor: impl FnMut(EdgeRef, &E)) {
        self.visit_dfs(begin, |_, _| (), edge_visitor)
    }
}

impl<N, E> Graph<N, E> {
    /// Most general BFS implementation
    pub fn visit_bfs(
        &self,
        begin: NodeRef,
        mut node_visitor: impl FnMut(NodeRef, &N),
        mut edge_visitor: impl FnMut(EdgeRef, &E),
    ) {
        // inner function to keep monomorphized assembly small
        fn bfs<N, E>(
            graph: &Graph<N, E>,
            begin: usize,
            node_visitor: &mut dyn FnMut(NodeRef, &N),
            edge_visitor: &mut dyn FnMut(EdgeRef, &E),
        ) {
            let mut visited = vec![false; graph.nodes.len()];
            let mut queue = VecDeque::new();
            queue.push_back(begin);

            while let Some(node) = queue.pop_front() {
                if visited[node] {
                    continue;
                }

                node_visitor(NodeRef { idx: node }, &graph.nodes[node]);
                visited[node] = true;

                for adj in &graph.adjacency[node] {
                    edge_visitor(adj.edge, &graph[adj.edge]);
                    queue.push_back(adj.node.idx);
                }
            }
        }

        bfs(self, begin.idx, &mut node_visitor, &mut edge_visitor)
    }

    /// Visit all the nodes in BFS order
    pub fn visit_bfs_nodes(&self, begin: NodeRef, node_visitor: impl FnMut(NodeRef, &N)) {
        self.visit_bfs(begin, node_visitor, |_, _| ())
    }

    /// Visit all the edges in BFS order
    pub fn visit_bfs_edges(&self, begin: NodeRef, edge_visitor: impl FnMut(EdgeRef, &E)) {
        self.visit_bfs(begin, |_, _| (), edge_visitor)
    }
}

impl<N> Graph<N, i32> {
    pub fn max_flow(&self, from: NodeRef, to: NodeRef) -> i32 {
        let mut i;
        let mut flow_on_path;
    }
    pub fn min_cut(&self, from: NodeRef, to: NodeRef) -> i32 {
        self.max_flow(from, to)
    }
}
