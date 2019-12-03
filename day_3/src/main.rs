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
    // Convert comma-separated string to vector of ints
    let input_data: Vec<&str> = contents.trim().split("\n").collect();

    let first_wire: Vec<&str> = input_data[0].split(",").collect();
    let second_wire: Vec<&str> = input_data[1].split(",").collect();

    // Transfrom into a set of lines, given by (start_coord, end_coord)

    let first_wire_coords = parse_wire_stringarray_to_coord_list(&first_wire);
    let second_wire_coords = parse_wire_stringarray_to_coord_list(&second_wire);

    // For every segment in the second wire, see if a segment in the first wire crosses it.

    for i in 0..first_wire_coords.len() - 1 {
        let first_wire_segment_start = first_wire_coords[i];
        let first_wire_segment_end = first_wire_coords[i + 1];

        for i in 0..second_wire_coords.len() - 1 {
            let second_wire_segment_start = second_wire_coords[i];
            let second_wire_segment_end = second_wire_coords[i + 1];

            // do these two lines cross
            let line_crossing = do_lines_cross(
                first_wire_segment_start,
                first_wire_segment_end,
                second_wire_segment_start,
                second_wire_segment_end,
            );
            if line_crossing {
                println!("{:?}", first_wire_segment_start);
                println!("{:?}", first_wire_segment_end);
                println!("{:?}", second_wire_segment_start);
                println!("{:?}", second_wire_segment_end);
            }
        }
    }

    println!("{:?}", first_wire);
    println!("{:?}", first_wire_coords);
    // println!("{:?}",  second_wire);

    Ok(())
}

fn parse_wire_stringarray_to_coord_list(wire_string_array: &Vec<&str>) -> Vec<[i32; 2]> {
    let mut wire_coords = vec![[0, 0]];
    let mut prev_coord = [0, 0];
    for segment in wire_string_array {
        let (direction, magnitude) = segment.split_at(1);
        let magnitude = magnitude.parse::<i32>().unwrap();
        let [prev_x, prev_y] = prev_coord;
        if direction == "U" {
            prev_coord = [prev_x, prev_y + magnitude];
        }
        if direction == "D" {
            prev_coord = [prev_x, prev_y - magnitude];
        }
        if direction == "R" {
            prev_coord = [prev_x + magnitude, prev_y];
        }
        if direction == "L" {
            prev_coord = [prev_x - magnitude, prev_y];
        }
        wire_coords.push(prev_coord);
    }

    wire_coords
}

fn do_lines_cross(
    first_wire_segment_start: [i32; 2],
    first_wire_segment_end: [i32; 2],
    second_wire_segment_start: [i32; 2],
    second_wire_segment_end: [i32; 2],
) -> bool {

    // Check whether the two lines cross
    // Two conditions: they're either parallel, or perpendicular
    



    false
}
