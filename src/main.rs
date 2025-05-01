mod bfs; 
mod another; 
mod tester; 
mod diameter; 
mod reader4; 
mod largest_component;
mod closeness; 
mod closeness_centrality_translator;
mod stronglyconnected;
use std::collections::VecDeque; 
use std::clone::Clone; 


fn parallel_closeness_runner(graph: Vec<Vec<u32>>) -> Vec<(u32, f64)> {
    let closest = closeness::parallel(graph).unwrap(); 
    let another = closest.clone(); 
    let returner = another.clone();
    let path = "closeness_centrality_storage"; 
    closeness::writer(path, closest).expect("shhhhhhiittttttttttttt");
    for (node, value) in another {
        println!("The node {node} has a closeness centrality of {value}"); 
   }
   returner
}
fn main() {
    let graph = reader4::prep("roadNet-CA.txt").unwrap();
    //tester::empty(graph); 
    //let start = largest_component::find_largest_component_node(&graph.0); 
    //let (a, b) = diameter::bfs(&graph.0, start); 
    //let (c, diameter) = diameter::bfs(&graph.0, a); 
    //println!("{}", diameter); 
    println!("{:?}", graph.0[0]); 
    let path = "closeness_centrality_storage";

    let storage = closeness_centrality_translator::reader(path).unwrap(); 
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
    println!("This is a test {}", checker - storage.1[0].1);
    println!("The number of nodes with an equal closeness centrality and therefore equal connected-ness is {}", usable.len()); 



   //let store = stronglyconnected::find_strongly_connected_components(&graph); 

}
