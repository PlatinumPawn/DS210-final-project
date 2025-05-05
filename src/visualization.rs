use plotters::prelude::*;
// I want to plot my calculated average path distance. We do this by making our min and our maxes
// then we use the structure that is from the rust online library and the lecture from Prof Gardos class
pub fn plotter(cc_output: Vec<(u32, f64)>, cc_min: &f64, bin_count: &usize, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // make a drawing area 
    // fill it with white 
    let mut root = BitMapBackend::new(path, (2000, 1000)).into_drawing_area();
    root.fill(&WHITE)?; 
    // calculating our minimum values and our maximum closeness values 
    let minimum: f64 = *cc_min; 
    // my cc_output from my translator, which goes in order
    // and originally I wrote the file in order of greatest to least based on cc value 
    // so therefore first value in vector = largest value 
    let maximum: f64 = cc_output[0].1; 
    // bin count (I will be using 1000 )
    let bin_count = *bin_count; 
    // minimum is small 
    let bin_width = (maximum - minimum) / bin_count as f64; 
    // seperating my data with nodes and values 
    let mut nodes: Vec<u32> = Vec::new(); 
    let mut values: Vec<f64> = Vec::new(); 
    // populating the two vectors 
    for (n, v) in &cc_output {
        nodes.push(*n); 
        values.push(*v); 
    }
    let mut bins = vec![0u32; bin_count]; 
    for value in values {
        // we have to iterate through our values, and adjust by the bin width 
        let bin_index = ((value-minimum) / bin_width).floor() as usize; 
        // if our bin_index is less than bin count 
        if bin_index < bin_count {
            bins[bin_index] += 1;
        }
    }
    let mut chart = ChartBuilder::on(&root)
    // denotes the xlable area and ylabel area
        .x_label_area_size(35)
        .y_label_area_size(40)
        // works wit hthe margin
        .margin(5)
        .caption("Average Shortest Path Length", ("sans-serif", 50.0))
        .build_cartesian_2d(0..bin_count, 0u32..*bins.iter().max().unwrap_or(&1))?;


    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(WHITE.mix(0.3))
        .y_desc("Count")
        .x_desc("Value")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;


    chart.draw_series(
        Histogram::vertical(&chart)
    .style(RED.mix(0.5).filled())
    .data(bins.iter().enumerate().map(|(i, &count)| (i, count)))); 


    // got from lecture, basically trying to make the png and err it if it doesn't work
    root.present().expect("Can't write the data, something went wrong!");
    // print the title 
    println!("Wrote to {}", path);
    
    
    Ok(())
}