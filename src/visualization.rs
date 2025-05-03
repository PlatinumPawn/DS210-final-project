use plotters::prelude::*;
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
    let bin_width = (minimum - maximum) / bin_count as f64; 

    let size = cc_output.len(); 

    // seperating my data with nodes and values 
    let mut nodes: Vec<u32> = Vec::new(); 
    let mut values: Vec<f64> = Vec::new(); 
    // populating the two vectors 
    for (n, v) in &cc_output {
        nodes.push(*n); 
        values.push(*v); 
    }
    let mut bins = vec![0u32; bin_count]; 

    for node in nodes {
        
    }

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Histogram Test", ("sans-serif", 50.0))
        .build_cartesian_2d((minimum..maximum).into_segmented(), 0u32..1000u32)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(WHITE.mix(0.3))
        .y_desc("Count")
        .x_desc("Closeness Centrality")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;


    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(values.iter().map(|&v| (v, 1))),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", path);
    
    
    Ok(())
}