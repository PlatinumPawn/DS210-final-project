use std::collections::VecDeque;
pub fn bfs(graph: &Vec<Vec<u32>>, start: u32) -> Vec<Option<u32>> {
    let mut dist = vec![None; graph.len()];
    let mut queue = VecDeque::new();

    dist[start as usize] = Some(0);
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        let current_dist = dist[node as usize].unwrap();

        for &neighbor in &graph[node as usize] {
            if dist[neighbor as usize].is_none() {
                dist[neighbor as usize] = Some(current_dist + 1);
                queue.push_back(neighbor);
            }
        }
    }
    dist
}
