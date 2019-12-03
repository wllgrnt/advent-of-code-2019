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

    let first_wire_segments = parse_wire_stringarray_to_segment_list(&first_wire);
    let second_wire_segments = parse_wire_stringarray_to_segment_list(&second_wire);

    // For every segment in the second wire, see if a segment in the first wire crosses it.

    for first_segment in &first_wire_segments {
        let first_wire_segment_start = first_segment.start;
        let first_wire_segment_end = first_segment.end;

        for second_segment in &second_wire_segments {
            let second_wire_segment_start = second_segment.start;
            let second_wire_segment_end = second_segment.end;

            // do these two lines cross
            let line_crossing = do_lines_cross(first_segment, second_segment);
            if line_crossing {
                println!("{:?}", first_wire_segment_start);
                println!("{:?}", first_wire_segment_end);
                println!("{:?}", second_wire_segment_start);
                println!("{:?}", second_wire_segment_end);
            }
        }
    }

    println!("{:?}", first_wire);
    println!("{:?}", first_wire_segments);
    // println!("{:?}",  second_wire);

    Ok(())
}

fn parse_wire_stringarray_to_segment_list(wire_string_array: &Vec<&str>) -> Vec<WireSegment> {
    let mut wire_coords = Vec::new();
    let mut prev_coord = [0, 0];
    for segment in wire_string_array {
        let (direction, magnitude) = segment.split_at(1);
        let magnitude = magnitude.parse::<i32>().unwrap();
        let [prev_x, prev_y] = prev_coord;
        let mut new_coord = [0, 0];
        if direction == "U" {
            new_coord = [prev_x, prev_y + magnitude];
        }
        if direction == "D" {
            new_coord = [prev_x, prev_y - magnitude];
        }
        if direction == "R" {
            new_coord = [prev_x + magnitude, prev_y];
        }
        if direction == "L" {
            new_coord = [prev_x - magnitude, prev_y];
        }
        let segment = WireSegment {
            start: prev_coord,
            end: new_coord,
            direction: direction.to_string(),
        };
        wire_coords.push(segment);
        prev_coord = new_coord
    }
    wire_coords
}

fn do_lines_cross(first_wire_segment: &WireSegment, second_wire_segment: &WireSegment) -> bool {
    // Check whether the two lines cross
    // Two conditions: they're either parallel, or perpendicular

    false
}

#[derive(Debug)]
struct WireSegment {
    start: [i32; 2],
    end: [i32; 2],
    direction: String,
}
