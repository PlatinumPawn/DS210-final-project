// objective is to calculate the shortest path between two nodes
// found a useful website that actually made me realize that I am able to add one line to the previous 
// BFS function 

use std::collections::VecDeque; 

pub fn shortest_path(graph: &Vec<Vec<u32>>, start: u32, end: u32) -> Option<u32> {
    let mut dist = vec![None; graph.len()];
    // init vecdeque for tracker
    let mut queue = VecDeque::new();
    // dist at value 
    let mut path = vec![None; graph.len()]; 
    dist[start as usize] = Some(0);
    queue.push_back(start);
    while let Some(node) = queue.pop_front() {
        let current_dist = dist[node as usize].unwrap();
        for &neighbor in &graph[node as usize] {
            if dist[neighbor as usize].is_none() {
                path[neighbor as usize] = Some(node); 
                dist[neighbor as usize] = Some(current_dist + 1);
                queue.push_back(neighbor);
            }
        }
    }
    // just return the target node index of our distance because BFS function has already populated that 
    dist[end as usize]
}

#[cfg(test)]
mod tests {
    use super::*; 
    #[test]
    fn tester() {
        // create a small graph 
        let graph: Vec<Vec<u32>> = vec![
            vec![2, 1], 
            vec![3, 2], 
            vec![0, 4], 
            vec![5], 
            vec![], 
            vec![],
            // should go from 0 to 1 to 3 to 5 
        ];
        let storage = shortest_path(&graph, 0, 5).unwrap(); 
        assert_eq!(storage, 3); 
        // shortest path is 3, 0 to 1 to 3 to 5, and it is correct!
    }
}

