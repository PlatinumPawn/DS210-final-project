use std::collections::VecDeque; 
pub fn find_largest_component_node(graph: &Vec<Vec<u32>>) -> u32 {
    let mut visited = vec![false; graph.len()];
    let mut max_size = 0;
    let mut best_start = 0;

    for i in 0..graph.len() {
        if !visited[i] && !graph[i].is_empty() {
            let mut queue = VecDeque::new();
            let mut size = 0;
            queue.push_back(i);
            visited[i] = true;

            while let Some(v) = queue.pop_front() {
                size += 1;
                for &u in &graph[v] {
                    if !visited[u as usize] {
                        visited[u as usize] = true;
                        queue.push_back(u as usize);
                    }
                }
            }

            if size > max_size {
                max_size = size;
                best_start = i;
            }
        }
    }

    best_start as u32
}


