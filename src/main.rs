// this is my main function, where I run my initial graph reader, my closeness centrality graph reader 
// and call my function that creates a histogram. 
// These is also a function called parallel_closeness_runner in this, where I stored the code that I ran in main
// I have commented out the call, but if ran it would run for greater than 4 hours and created my 
// closeness_centrality_store with each individual value

mod reader; 
mod closeness; 
mod closeness_centrality_translator;
mod visualization; 
mod shortest_path; 
use std::clone::Clone; 

use crate::reader::Graph; 

fn parallel_closeness_runner(graph: Vec<Vec<u32>>) -> Vec<(u32, f64)> {
    // this is the code that I ran to initially calculate the inverse closeness centrality 
    // (I will find the normally defined closeness by taking the inverse later)
    // took more than four hours 
    // it is commented out of main, so that everything else can run 
    println!("Calculating the closeness centrality of the input graph!"); 
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

    let path: &str = "closeness_centrality_storage";
    //parallel_closeness_runner(graph.graph); 
    // this would calculate the average closeness and run it 

    // call the reader, store it 
    let storage = closeness_centrality_translator::reader(path).unwrap(); 
    // grab the only usable 
    let length = storage.clone().0.len(); 
    let closeness_centrality = storage.clone().0; 
    let avg_path_length = storage.1.clone(); 

    // our writer sorts by greatest to least, so the least value is going to be at the end of the graph
    let smallest_apl = storage.clone().1[length - 1].1;
    let smallest_cc: f64 = storage.clone().0[length -1].1; 

    // making our visualization for avg path length, as I find it more compelling and interesting to visualize
    // average path length than closeness centrality 
    visualization::plotter(avg_path_length, &smallest_apl, &1000, "visualization_apl.png");

    let nodes = graph.graph.len(); 
    println!("The node count of this graph is {}", nodes); 

    let start = 0; 
    let end = (graph.graph.len() -1) as u32; 

    // finding the shortest path between the start and end node
    let shortest_total = shortest_path::shortest_path(&graph.graph, start, end).unwrap(); 
    println!("This is the shortest path between {} and {}, is {}", &start, &end, &shortest_total); 

    let mean_cc: f64 = closeness_centrality.iter().map(|(_, v)| v).sum::<f64>() / closeness_centrality.len() as f64;
    println!("mean closeness centrality: {:.8}", mean_cc);


    // outliers are inflating our mean 
    // so we want to filter them out
    // use a filter, and then a collect to filter out the values that are greater than zero
    // I kept them in my average path length because it was good to visualize, and they were so small 
    // that they did not much affect the mean
    // but because they are inverses, when that wasn't a problem, now it is for closeness centrality 

    // using past filter function to implement
    let filtered_cc: Vec<f64> = closeness_centrality.iter()
    .map(|(_, v)| *v)
    .filter(|v| *v < 1.0) 
    .collect();

    let mean_cc: f64 = filtered_cc.iter().sum::<f64>() / filtered_cc.len() as f64;
    println!("filter mean closeness centrality, where nodes are greater than zero: {:.8}", mean_cc);
    println!("This is the amount of outliers that we have in our dataset! {}", nodes - filtered_cc.len())





}


