use plotters::prelude::*;
use std::collections::HashMap;
// I am trying to visualize the data that I got from my graph 

pub fn histo(cc_output: Vec<(u32, f64)>, cc_min: Vec<(u32, f64)>) -> Result<(), Box<dyn std::error::Error>> {

    let root = BitMapBackend::new("closeness_histogram.png", (1920, 1080)).into_drawing_area();
    root.fill(&WHITE)?;

    let values: Vec<f64> = cc_output.iter().map(|(_, v)| *v).collect();
    
    // Find actual min and max values
    let max_val: f64 = values[0]; 
    let min_val: f64 = cc_min[0].1; 
    
    println!("Raw data - Max: {}, Min: {}", max_val, min_val);
    println!("Number of data points: {}", values.len());

    // Create more reasonable number of bins based on data size
    let num_bins = 1000;
    let bin_width = (max_val - min_val) / num_bins as f64;
    
    if bin_width <= 0.0 {
        println!("Warning: All values are the same or nearly the same.");
        return Err("Invalid bin width: closeness values are all the same.".into());
    }
    
    println!("Bin width: {}", bin_width);

    // Create histogram bins
    let mut bins = vec![0u32; num_bins];
    let mut bin_counts = HashMap::new();
    
    // Count values in each bin
    for &val in &values {
        if val.is_finite() {
            let bin_idx = ((val - min_val) / bin_width).floor() as usize;
            let idx = if bin_idx >= num_bins { num_bins - 1 } else { bin_idx };
            bins[idx] += 1;
            
            // Track which values went into which bins for debugging
            bin_counts.entry(idx).and_modify(|e| *e += 1).or_insert(1);
        }
    }

    // Check bin distribution
    let max_bin_count = *bins.iter().max().unwrap_or(&1);
    println!("Max bin count: {}", max_bin_count);
    
    // Show distribution of first few and last few bins
    println!("First 10 bins:");
    for (i, count) in bins.iter().enumerate().take(10) {
        let bin_start = min_val + (i as f64 * bin_width);
        let bin_end = bin_start + bin_width;
        println!("Bin {}: [{:.8} - {:.8}] count {}", i, bin_start, bin_end, count);
    }
    
    println!("Last 10 bins:");
    for (i, count) in bins.iter().enumerate().rev().take(10) {
        let bin_start = min_val + (i as f64 * bin_width);
        let bin_end = bin_start + bin_width;
        println!("Bin {}: [{:.8} - {:.8}] count {}", i, bin_start, bin_end, count);
    }

    // Create reasonable scale for y-axis
    let y_max = max_bin_count + (max_bin_count / 10); // Add 10% headroom

    // Set up actual value ranges for x-axis instead of just bin numbers
    let x_range = min_val..max_val;
    
    // Build chart with proper ranges
    let mut chart = ChartBuilder::on(&root)
        .caption("Closeness Centrality Histogram", ("sans-serif", 40).into_font())
        .margin(10)
        .x_label_area_size(60)
        .y_label_area_size(80)
        .build_cartesian_2d(x_range, 0u32..y_max)?;

    chart
        .configure_mesh()
        .x_desc("Closeness Centrality")
        .y_desc("Node Count")
        .x_labels(20) // Reasonable number of x-axis labels
        .y_labels(10) // Reasonable number of y-axis labels
        .draw()?;

    // Draw proper histogram bars that span the correct range
    chart.draw_series(
        (0..num_bins).map(|i| {
            let bin_start = min_val + (i as f64 * bin_width);
            let bin_end = bin_start + bin_width;
            let count = bins[i];
            
            Rectangle::new(
                [(bin_start, 0), (bin_end, count)],
                RED.mix(0.6).filled(),
            )
        })
    )?;

    // Add some visual aids
    // Add mean line if we can calculate it
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    if mean.is_finite() {
        chart.draw_series(std::iter::once(PathElement::new(
            vec![(mean, 0), (mean, y_max)],
            BLUE.stroke_width(2),
        )))?;
        
        // Add label for mean
        chart.draw_series(std::iter::once(Text::new(
            format!("Mean: {:.6}", mean),
            (mean, y_max - (y_max / 10)),
            ("sans-serif", 20).into_font().color(&BLACK),
        )))?;
    }

    // Add legend
    chart.configure_series_labels()
        .background_style(WHITE.filled())
        .border_style(BLACK)
        .draw()?;

    // Save the result
    root.present().expect("Failed to present drawing");
    println!("Histogram rendered successfully to closeness_histogram.png");
    
    Ok(())
}