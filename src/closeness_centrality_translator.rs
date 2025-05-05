// The objective of this file is to parse my file that I wrote to store the values of my closeness calculations
// and transform it slightly. Instead of taking 4-6 hours each time I want to run, I stored it in a file
// which this reads. Because I was basically calculating average path length, but I want closeness centrality 
// I output both. The direct read is the second vector, which is the average path length of each node, in teh form 
// of node value pair. And the first vector I inverse the value, making it the standard closeness centrality cal-
// culation. 
use std::fs::File;
use std::io::{self, BufRead};
// returning an io Result with two vectors, one for closeness centrality node, value pairs and the other 
// for avg path length pairs. 
// takes a path to the file that my closeness::writer wrote to only run the long calculation once
// making a reader, two vectors then iterating through the lines and assigning the node value pairs
pub fn reader(path: &str) -> io::Result<(Vec<(u32, f64)>, Vec<(u32, f64)>)> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file); 
    let lines = reader.lines(); 
    let mut avg_path_length: Vec<(u32, f64)> = Vec::new(); 
    let mut closeness_centrality: Vec<(u32, f64)> = Vec::new();
    // inverse of what we previously calculated is the standard closeness centrality defintion  
    for line in lines {
        // standard line reading 
        let line = line.expect("Something went wrong with reading the line!!"); 
        // splitting the line by space, so that node and value are seperate parts of the collected vector
        let parts: Vec<&str> = line.split_whitespace().collect();
        // very similar to my first reader function, just different in that one is an f64
        // matching and parsing with our u32 assignment 
        let node = parts[0].parse::<u32>().expect("Something went wrong!");
        let value = parts[1].parse::<f64>().expect("Something went wrong"); 
        // pushing our node value pairs
        avg_path_length.push((node, value)); 
        // closeness centrality is the inverse of what I calculated, so I need to inverse the value to get 
        // closeness centrality as it is normally defined 
        closeness_centrality.push((node, 1.0 / value)); 
    }
    Ok((closeness_centrality, avg_path_length))
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn tester() {

        // reading in my two vectors, which are closeness_centrality calculations and the inverse, which is what
        // I initially calculated 
        let (closeness, avg)  = reader("closeness_centrality_storage").expect("Something went wrong");
        for ((n1, cc), (n2, apl)) in closeness.iter().zip(avg) {
            // checking to see that the inverse of 1 equals the other 
            let inverse: f64 = 1.0 / apl; 
            // allowing for margin of error when dealing with f64
            let margin = 1e-6; 
            let combine = (cc - inverse).abs(); 
            // making sure that the inverse of one is basically equal to the other, within a margin of error
            assert!(combine < margin);
        }
    }
}
