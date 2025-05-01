

pub fn empty(graph: Vec<Vec<u32>>) {
    let mut counter = 0; 
    let mut good = 0; 
    for list in graph {
        if list.is_empty() {
            counter += 1; 
        }
        else {good += 1;}
    }
    println!("{}", counter); 
    println!("{}", good);
}

// do something simple and do the average adj list 