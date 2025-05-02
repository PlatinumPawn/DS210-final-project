use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
// make a graph struct that holds an adj list and the number of nodes 


pub struct Graph {
    pub graph: Vec<Vec<u32>>,
    pub nodes: usize,
    pub old_to_new: HashMap<u32, usize>,
    pub new_to_old: HashMap<usize, u32>,
    pub edges: Vec<(usize, usize)>,
}


// prepare and preprocess graph to be used
// currently there are gaps in the graph, so we have to remap it

pub fn prep(path: &str) -> Result<Graph, io::Error> {//Result<(Vec<Vec<u32>>, HashMap<u32, usize>, Vec<(usize, usize)>), io::Error> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let lines = reader.lines();

    // remapping my graph because it has a few gaps 
    let mut nodes = 0; // counting nodes (spoiler, they are the same!! that's good)
    let mut old_to_new = HashMap::new(); // old to new id, new is going to be the counter
    let mut new_to_old: HashMap<usize, u32> = HashMap::new(); 
    let mut graph: Vec<Vec<u32>> = vec![Vec::new()]; // our adj list that is going to be our new vec<vec<u32>> that has our graph
    let mut counter = 0; // counter that becomes our values each time
    let mut edges: Vec<(usize, usize)> = Vec::new(); 
    for line in lines {
        let line = line?;
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }

        let from = match parts[0].parse::<u32>() {
            Ok(val) => val,
            Err(_) => continue,
        };
        let to = match parts[1].parse::<u32>() {
            Ok(val) => val,
            Err(_) => continue,
        };

        if !old_to_new.contains_key(&from) { 
            old_to_new.insert(from, counter); 
            counter += 1; 
            nodes += 1; 
        }

        if !old_to_new.contains_key(&to) {
            old_to_new.insert(to, counter); 
            counter += 1; 
            nodes += 1; 
        }

        let new_from = old_to_new[&from]; 
        let new_to = old_to_new[&to] as u32;

        edges.push((new_from, new_to as usize)); 

        while graph.len() <= new_from {
            graph.push(Vec::new()); 
        }

        graph[new_from].push(new_to); 
    }
    graph.resize(counter, Vec::new());
    //Ok((graph, old_to_new, edges))

    Ok(Graph {
        graph: graph,
        nodes: nodes,
        old_to_new,
        new_to_old,
        edges,
    })
}



