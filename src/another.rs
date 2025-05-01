use std::collections::VecDeque; 
pub fn bfs_eccentricities(graph: &Vec<Vec<u32>>) -> Vec<u32> {
    let mut eccentricities = vec![0; graph.len()];

    for start in 0..graph.len() as u32 {
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

        eccentricities[start as usize] = dist.iter().filter_map(|&d| d).max().unwrap_or(0);
    }

    eccentricities
}
