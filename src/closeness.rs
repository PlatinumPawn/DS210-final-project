// The objective is to compute the closeness centrality of each node to each other node, after talking with a 
// prof we decided to compute closeness slightly differently, which can be seen below. Then the output of the clo
//seness centrality of each node of the graph is written to a file, to avoid rerunning a multi-hour function. 

use std::collections::VecDeque; 
use rayon::prelude::*;
use std::io::{BufWriter, BufReader}; 
use std::fs::File;
use std::io::Write;
use std::io; 
// standard BFS from lecture 
pub fn bfs(graph: &Vec<Vec<u32>>, start: u32) -> Vec<Option<u32>> {
    // make the vector that will be output
    let mut dist = vec![None; graph.len()];
    // init vecdeque for tracker
    let mut queue = VecDeque::new();
    // dist at value 
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

// have the graph be a struct, and then those would be impls inside it 
// then I would just call it with the dot notiatoin

// we want to calculate the closeness of one value, so basically just unwrapping everything that BFS gave us and 
// summing that up 
pub fn closeness(graph: &Vec<Vec<u32>>, start: usize) -> Result<f64, Box<dyn std::error::Error>> {
    // want to return the closeness value and the node
    let calc = bfs(&graph, start as u32); 
    let mut total_distance = 0; // total distance that all the nodes 
    // iterate through our graph and count those iterations 
    for (iter, value) in calc.iter().enumerate() {
        // match it because BFS returns a vec of options, so we have to unwrap them 
        if let Some(yes) = value {
            // making sure that our current value does not equal our starting point
            // because if it does then we don't want to include that 
            if iter != start as usize {
                total_distance += yes; 
            }
        }
    }
    // compute the average distance that we got, from all other nodes
    let done = (total_distance as f64) / (graph.len() as f64 -1.0); // take the struct and not the vec
    Ok(done)
}

pub fn many(graph: Vec<Vec<u32>>) -> Result<Vec<(u32, f64)>, Box<dyn std::error::Error>> {
    let mut storage: Vec<(u32, f64)> = Vec::new(); 
    let length = graph.len(); 
    println!("Length of graph is {}", length);
    for iter in 0..length {
        if iter % 100 == 0 {
            println!("Working on node {}", iter);
        }
        let current_closeness = closeness(&graph, iter); 
        storage.push((iter as u32, current_closeness.unwrap()));
    }

    storage
    .sort_by(|x,y| x.1.partial_cmp(&y.1).unwrap());

    Ok(storage)
}

// help of chatgpt, I gave it my original code because I was struggling to paralleize it 
pub fn parallel(graph: Vec<Vec<u32>>) -> Result<Vec<(u32, f64)>, Box<dyn std::error::Error>> {
    // calling a parallelized iter on a list of data from 0 to our graph len 
    let storage: Vec<(u32, f64)> = (0..graph.len()).into_par_iter()
    // mapping on that list, which works only because we remapped our dataset in the first place 
    .map(|node| {
        // assigning a var value, calling closeness on that node, and unwrapping if we get a wrong value
        let value = closeness(&graph, node).unwrap_or(0.0);
        // assigning our node, which is a node in our dataset, and our value, which is the calculated closeness
        // centrality value for that specific node 
        (node as u32, value)
    }).collect(); 

    // making the vec<(u32, f64)> closeness nodes and values mutable
    let mut change = storage; 

    // sorts in greatest to least 
    change.sort_by(|x,y| y.1.partial_cmp(&x.1).unwrap()); 

    println!("Node {} has a closeness centrality of {}", change[0].0, change[0].1); 
    Ok(change)

}

pub fn writer(path: &str, content: Vec<(u32, f64)>) -> io::Result<()>{
    // creating the file path to write
    let f = File::create(path).expect("that was a wasted four hours"); 
    // make a bufwriter, to enhance the writing
    let mut buf = BufWriter::new(f); 

    // iterate through the input content, which is our vec<(u32, f64)> in order of greatest to least
    // so therefore our most important closeness values, the smallest ones, will be at the bottom of the list 
    for (node, value) in content {
        // write a newline node value pair, with a space in between
        writeln!(buf, "{} {}", node, value)?; 
    }
    // return an io result with nothing in it, because we write the file instead
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*; 
    #[test]
    fn tester() {
        let graph: Vec<Vec<u32>> = vec![
        vec![1, 2],      
        vec![0, 2, 3],    
        vec![0, 1, 3, 4], 
        vec![1, 2, 4],    
        vec![2, 3],       
    ];
    // small graph where our closeness centrality summed up should be 6.5
    // used chat to help me to find what the value should be 

    let storage = parallel(graph).unwrap(); 
    assert_eq!(storage.len(), 5); 
    let store = storage.iter().map(|(x, y)| y).sum();

    // to be clear, my closeness centrality is sort of different from the normal calculation 
    // I am finding the average distance to all other nodes, and as such, the smallest distance 
    // is what I would consider the most centrally located node, or the one with the highest closeness centrality 
    assert_eq!(6.5, store);
    }
}
