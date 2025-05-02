// this is easier than running the whole function (which takes >6 hours) but can still extrapolate the results and 
// be parseable!

// then I'm going to match the ID's with my hashmap to see the original nodes that have the lowest closeness 
// centrality! (for my purposes that means they're the most connected)
use std::collections::HashMap; 
use std::fs::File;
use std::io::{self, BufRead};



pub fn reader(path: &str) -> io::Result<(Vec<(u32, f64)>, Vec<(u32, f64)>)> {
    // want to return a tuple with two vectors, one with all of the values, and one with just the node value pairs
    // that have the lowest closeness centrality, because how I calculated it, that means they are the most 
    // connected 

    // I could have just organized my writer to write in inverse order, so that smallest first 
    // and then I could have just taken the first value and compared it to the rest and seen if that works
    // but this isn't that much extra work 
    let file = File::open(path)?;
    let reader = io::BufReader::new(file); 
    let lines = reader.lines(); 
    let mut least: f64 = 1.0e10; 
    let mut storage: Vec<(u32, f64)> = Vec::new(); 
    let mut most: Vec<(u32, f64)> = Vec::new(); 
    for line in lines {
        let line = line.expect("Something went wrong with reading the line!!"); 
        let parts: Vec<&str> = line.split_whitespace().collect();
        let node = match parts[0].parse::<u32>() {
            Ok(point) => point, 
            Err(e) => continue, 
        };
        let value = match parts[1].parse::<f64>() {
            Ok(floast) => floast, 
            Err(error) => continue,
        }; 
        storage.push((node, value)); 
        most.push((node, value)); 

        if value < least {
            least = value; 
        }
        // finding the lowest value 
    }

    // so we assign our iterator methods to the closures 
    // we take our most output, which is just the same as storage currently
    // and we iterate through it, filter it by the 7 value, and must equal least 
    // then collect it back into a vector 
    let closest: Vec<(u32, f64)> = most
    .into_iter()
    .filter(|(x, y)| *y == least)
    .collect(); 
    // filtering by the found lowest value 
    Ok((storage, closest))
}


// make a histogram and then see 
// maybe do shortest path 
// shortest path between two points 