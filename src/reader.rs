use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
// The objective of this file is to read in the data graph and make it usable to give to our other functions 
// to process (run closeness centrality on it) and visualize it 
// we read in the file, split it by the from and the to nodes, append those to our hashmaps 
// then resize the vector and add it to our adj list!

pub struct Graph {
    pub graph: Vec<Vec<u32>>,
    pub nodes: usize,
    pub old_to_new: HashMap<u32, usize>,
    pub new_to_old: HashMap<usize, u32>,
}
// prepare and preprocess graph to be used
// currently there are gaps in the graph, so we have to remap it

pub fn prep(path: &str) -> Result<Graph, io::Error> {//Result<(Vec<Vec<u32>>, HashMap<u32, usize>, Vec<(usize, usize)>), io::Error> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let lines = reader.lines();

    // remapping my graph because it has a few gaps 
    let mut nodes = 0; // counting nodes 
    let mut old_to_new = HashMap::new(); // old to new id, new is going to be the counter
    let mut new_to_old: HashMap<usize, u32> = HashMap::new(); 
    let mut graph: Vec<Vec<u32>> = vec![Vec::new()]; // our adj list that is going to be our new vec<vec<u32>> that has our graph
    let mut counter = 0; // counter that becomes our values each time
    for line in lines {
        // unwrap and handle errors
        let line = line?;
        // ensure / remove any lines that start with #, as my graph originally had 
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        // take our line, split by whitespace and collect
        // have a vec with two elements
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }
        // assign from and to nodes and unwrap it with an expect that will throw an error if something happens
        let from = parts[0].parse::<u32>().expect("Somethign went wrong in the read"); 
        let to = parts[1].parse::<u32>().expect("Something went wrong!");

        // check if our hashmap contains from or to
        // if not, insert the hashmap with our counter
        // the counter will become the remapped nodes for each for and to item
        if !old_to_new.contains_key(&from) { 
            old_to_new.insert(from, counter); 
            new_to_old.insert(counter, from); 
            counter += 1; 
        }

        if !old_to_new.contains_key(&to) {
            old_to_new.insert(to, counter); 
            new_to_old.insert(counter, to); 
            counter += 1; 
        }

        // reassign the new from and the new to 
        // grabbing the counter value that was at the time that we assigned the from and to 
        // to the hashmaps 
        let new_from = old_to_new[&from]; 
        let new_to = old_to_new[&to] as u32;

        // resize our graph so we can do this all in one for loop 
        while graph.len() <= new_from {
            graph.push(Vec::new()); 
        }

        // push our new values onto the graph 
        graph[new_from].push(new_to); 
    }
    // return our graph with all our newly assigned values!
    Ok(Graph {
        graph: graph,
        nodes: nodes,
        old_to_new: old_to_new,
        new_to_old: new_to_old,
    })
}



