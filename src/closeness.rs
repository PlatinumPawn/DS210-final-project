use std::collections::VecDeque; 
use rayon::prelude::*;
use std::io::{BufWriter, BufReader}; 
use std::fs::File;
use std::io::Write;
use std::io; 
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

// have the graph be a struct, and then those would be impls inside it 
// then I would just call it with the dot notiatoin


pub fn closeness(graph: &Vec<Vec<u32>>, start: usize) -> Result<f64, Box<dyn std::error::Error>> {
    // want to return the closeness value and the node
    let calc = bfs(&graph, start as u32); 
    let mut counter = 0; // total number of nodes that we can reach 
    let mut total_distance = 0; // total distance that all the nodes 
    for (iter, value) in calc.iter().enumerate() {
        if let Some(yes) = value {
            if iter != start as usize {
                counter += 1; 
                total_distance += yes; 
            }
        }
    }
    let done = (total_distance as f64) / (graph.len() as f64 -1.0);
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


pub fn parallel(graph: Vec<Vec<u32>>) -> Result<Vec<(u32, f64)>, Box<dyn std::error::Error>> {
    let storage: Vec<(u32, f64)> = (0..graph.len()).into_par_iter()
    .map(|node| {
        let value = closeness(&graph, node).unwrap_or(0.0);
        (node as u32, value)
    }).collect(); 

    let mut change = storage; 

    // sorts from least to greatest 
    change.sort_by(|x,y| y.1.partial_cmp(&x.1).unwrap()); 
    // reverse our vector so now it's greatest to least 
    // change.reverse(); 
    println!("Node {} has a closeness centrality of {}", change[0].0, change[0].1); 
    Ok(change)

}

pub fn writer(path: &str, content: Vec<(u32, f64)>) -> io::Result<()>{
    let f = File::create(path).expect("that was a wasted four hours"); 
    let mut buf = BufWriter::new(f); 

    for (node, value) in content {
        writeln!(buf, "{} {}", node, value)?; 
    }
    Ok(())
}


// get BFS distances to all other nodes, add them up, then divide them by n-1 
// that is the closeness centrality 



// approx algorithm 
// rando selected 1% of nodes, and do the full calculation of the distances for those
// for those nodes I will have the distance for those nodes and every other node
// for all of the other nodes they will be closest to one of them
// there will be one node that they are closest to 
// take the other node, and get the thing, sort it, and find where they're closest 
// then for that node, their neighbors will get a distance of 1, then for all of the other nodes, they 
// for every other node, it's a distance from a plus the distance that it takes to get to

// if it becomes untennable, get 10% of the nodes, and then the edges that contain those nodes
// then only keep the pairs that obth sides are the subset that I decided on 

// strongly connected is much easier 