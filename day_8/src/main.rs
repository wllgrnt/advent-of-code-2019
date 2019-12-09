// Day 8

use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let input_filename = "input.txt";
    if let Err(e) = run(input_filename) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(filename: &str) -> Result<(), Box<dyn Error>> {
    // Read the input file
    let contents = fs::read_to_string(filename)?;

    // find the layer that contains the fewest 0 digits. On that layer, what is the number of 1 digits multiplied by the number of 2 digits?
    let img_width = 25;
    let img_height = 6;
    // Read the input into a tensor, which we can index image[k][i][j]
    // We actually don't need to do that yet.
    let raw_image_data: Vec<u32> = contents
        .trim()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();

    // split into layers (requires the image data to be the correct length)
    assert_eq!(raw_image_data.len() % (img_height * img_width), 0);

    let mut layers: Vec<Vec<u32>> = Vec::new();
    for layer in raw_image_data.chunks(img_height * img_width) {
        let layer = layer.to_vec();
        layers.push(layer.clone());
    }
    // Get the layer with the smallest number of zeros
    let mut min_zeros_index = 0;
    let mut min_num_zeros = 100000;
    let mut i = 0;
    for layer in &layers {
        let num_zeros = layer.iter().filter(|&n| *n == 0).count();
        if num_zeros < min_num_zeros {
            min_zeros_index = i;
            min_num_zeros = num_zeros;
        }
        i += 1;
    }

    println!("{}", min_zeros_index);
    let min_zeros_layer = &layers[min_zeros_index];
    println!(
        "num zeros: {}",
        min_zeros_layer.iter().filter(|&n| *n == 0).count()
    );
    // We want the  the number of 1 digits multiplied by the number of 2 digits?
    let num_ones = min_zeros_layer.iter().filter(|&n| *n == 1).count();
    let num_twos = min_zeros_layer.iter().filter(|&n| *n == 2).count();
    println!("num ones * num twos: {}", num_ones * num_twos);

    Ok(())
}
