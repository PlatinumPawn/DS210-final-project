use std::collections::VecDeque; 

pub fn bfs(graph: &Vec<Vec<u32>>, start:u32) -> (u32, u32) { 
    let mut queue: VecDeque<u32> = VecDeque::new(); 
    let mut distance = vec![None; graph.len()];
    let mut most = 0; 
    let mut far = start; 
    distance[start as usize] = Some(0); 
    queue.push_back(start as u32); 
    while let Some(v) = queue.pop_front() { // new unprocessed vertex
        //println!("top {:?}",queue);
        for u in graph[v as usize].iter() {
            if let None = distance[*u as usize] { // consider all unprocessed neighbors of v
                distance[*u as usize] = Some(distance[v as usize].unwrap() + 1);
                queue.push_back(*u);
                let new = distance[v as usize].unwrap() + 1; 
                if new > most {
                    most = new;
                    far = *u;
                    
                } 
            }
            
                       
        }
    };
    (far, most)
}