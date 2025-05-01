fn reverse(graph: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut new = vec![vec![]; graph.len()]; 
    // make a new graph with the same size
    for (node, neighbors) in graph.iter().enumerate() {
        // iterate through the graph, get the node, and the list of neighbors
        // the node is just the index though so that is why it works
        // the neighbors are the list of neighbors 
        for neighbor in neighbors {
            new[*neighbor as usize].push(node as u32);
        }
    }
    new
}

use std::collections::VecDeque;

// Type aliases
type Vertex = usize;
type ListOfEdges = Vec<(Vertex, Vertex)>;
type AdjacencyLists = Vec<Vec<Vertex>>;
type Component = usize;

// Graph Struct
#[derive(Debug)]
struct Graph {
    n: usize,
    outedges: AdjacencyLists,
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph { n, outedges: vec![vec![]; n] }
    }

    fn from_edges(n: usize, edges: &ListOfEdges) -> Self {
        let mut g = Graph::new(n);
        for &(u, v) in edges {
            g.outedges[u].push(v);
        }
        for l in g.outedges.iter_mut() {
            l.sort();
        }
        g
    }

    fn reverse(&self) -> Self {
        let mut reversed = vec![vec![]; self.n];
        for (u, neighbors) in self.outedges.iter().enumerate() {
            for &v in neighbors {
                reversed[v].push(u);
            }
        }
        Graph { n: self.n, outedges: reversed }
    }
}

fn dfs_fill_order(v: Vertex, graph: &Graph, visited: &mut Vec<bool>, stack: &mut Vec<Vertex>) {
    visited[v] = true;
    for &w in &graph.outedges[v] {
        if !visited[w] {
            dfs_fill_order(w, graph, visited, stack);
        }
    }
    stack.push(v);
}

fn dfs_mark_component(v: Vertex, graph: &Graph, component: &mut Vec<Option<Component>>, label: Component) {
    component[v] = Some(label);
    for &w in &graph.outedges[v] {
        if component[w].is_none() {
            dfs_mark_component(w, graph, component, label);
        }
    }
}
pub fn find_strongly_connected_components(graph: &Graph) -> Vec<Vec<Vertex>> {
    let mut visited = vec![false; graph.n];
    let mut stack = Vec::new();

    // Step 1: Fill stack with finish order (postorder)
    for v in 0..graph.n {
        if !visited[v] {
            dfs_fill_order(v, graph, &mut visited, &mut stack);
        }
    }

    // Step 2: Reverse the graph
    let reversed_graph = graph.reverse();

    // Step 3: DFS in reverse order to find components
    let mut component = vec![None; graph.n];
    let mut components: Vec<Vec<Vertex>> = vec![];
    let mut current_label = 0;

    while let Some(v) = stack.pop() {
        if component[v].is_none() {
            current_label += 1;
            let mut current = vec![];
            dfs_collect_component(v, &reversed_graph, &mut component, current_label, &mut current);
            components.push(current);
        }
    }

    components
}

// Helper DFS that collects members into a vector
fn dfs_collect_component(
    v: Vertex,
    graph: &Graph,
    component: &mut Vec<Option<Component>>,
    label: Component,
    result: &mut Vec<Vertex>,
) {
    component[v] = Some(label);
    result.push(v);
    for &w in &graph.outedges[v] {
        if component[w].is_none() {
            dfs_collect_component(w, graph, component, label, result);
        }
    }
}
