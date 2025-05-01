use std::collections::{HashSet, VecDeque}; 

pub fn shortest_path_length(graph: Vec<Vec<u32>>) -> u32 {
    let length = graph.len(); 
    let all_visited = (1 << length) - 1; // `n` → `length`
    let mut queue: VecDeque<(u32, u32, u32)> = VecDeque::new(); // Fixed tuple syntax
    let mut visited: HashSet<u32> = HashSet::new(); 

    for i in 0..length as u32 {
        queue.push_back((1 << i, i, 0)); 
        visited.insert((1 << i) * 16 + i); 
    }

    while let Some((mask, node, dist)) = queue.pop_front() { // Added missing `,` and renamed `dist`
        if mask == all_visited as u32 {
            return dist; 
        }
        for &neighbor in &graph[node as usize] {
            let new_mask = mask | (1 << neighbor); 
            let hash_value = new_mask * 16 + neighbor; 

            if !visited.contains(&hash_value) {
                visited.insert(hash_value); 
                queue.push_back((new_mask, neighbor, dist + 1));             
            }
        }
    }

    unreachable!(); // Previously you had `-1` in the loop, which isn't valid return for `u32`
}
