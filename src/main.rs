mod reader; 
mod closeness; 
mod closeness_centrality_translator;
mod histogram;
use std::clone::Clone; 

use crate::reader::Graph; 

// histo
// SCC
fn parallel_closeness_runner(graph: Vec<Vec<u32>>) -> Vec<(u32, f64)> {
    // this is the code that I ran to intiially calculate the closeness centrality (or average distance)
    // took more than four hours 
    let closest = closeness::parallel(graph).unwrap(); 
    // clone because the significant portion calculation is done, now just printing
    let another = closest.clone(); 
    let returner = another.clone();
    let path = "closeness_centrality_storage"; 
    closeness::writer(path, closest).expect("Something went wrong!");
    for (node, value) in another {
        println!("The node {node} has a closeness centrality of {value}"); 
   }
   returner
}
fn main() {
    let graph = reader::prep("roadNet-CA.txt").unwrap();
    //tester::empty(graph); 
    //println!("{}", diameter); 
    println!("{:?}", graph.graph[0]); 
    let path = "closeness_centrality_storage";

    // call the reader, store it 
    let storage = closeness_centrality_translator::reader(path).unwrap(); 
    // grab the only usable 
    let usable = storage.1; 
    let mut checker: f64 = 0.0; 
    for (node, value) in &usable {
        if *value != 0.0000005088527659964228 {
            println!("This doesn't work"); 
            break
        }
        println!("The node is {node} and the value is {value}"); 
        checker += value; 
    }
    checker = checker / 4478.0; 
    println!("This is the checker {}", checker); 
    println!("The number of nodes with an equal closeness centrality and therefore equal connected-ness is {}", usable.len()); 
    let median = storage.0[storage.0.len() / 2];
    let mut meannn: f64 = 0.0; 
    for i in storage.0.clone() {
        meannn += i.1; 
    }
    meannn = meannn / storage.0.len() as f64; 
    println!("{}", meannn); 
    //let draw = histogram::histo(storage.0, usable); 



    //let store = stronglyconnected::find_strongly_connected_components(&graph); 

}


