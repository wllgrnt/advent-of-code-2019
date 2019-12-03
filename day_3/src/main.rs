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

    let mut nearest_crossing_manhattan: [i32; 2] = [1000000000, 1000000000];
    let mut nearest_crossing_wire_timing: i32 = 1000000000;
    let mut first_wire_timing: i32 = 0;
    for first_segment in &first_wire_segments {
        let mut second_wire_timing: i32 = 0;
        for second_segment in &second_wire_segments {
            // do these two lines cross
            let line_crossing: Option<[i32; 2]> = do_lines_cross(first_segment, second_segment);

            match line_crossing {
                None => (),
                Some(crossing) => {
                    // If there's a crossing, get the total length of wire to this point.
                    // Then add the length of the wire up the the crossing point.
                    // This is the distance from first_segment.start to the crossing,
                    // Plus the distance from second_segment.start to the crossing.
                    let crossing_total_timing = first_wire_timing
                        + second_wire_timing
                        + (first_segment.start[0] - crossing[0]).abs()
                        + (first_segment.start[1] - crossing[1]).abs()
                        + (second_segment.start[0] - crossing[0]).abs()
                        + (second_segment.start[1] - crossing[1]).abs();

                    if crossing_total_timing < nearest_crossing_wire_timing {
                        nearest_crossing_wire_timing = crossing_total_timing;
                    }

                    if crossing[0].abs() + crossing[1].abs()
                        < nearest_crossing_manhattan[0].abs() + nearest_crossing_manhattan[1].abs()
                    {
                        nearest_crossing_manhattan = [crossing[0], crossing[1]];
                    }
                }
            }
            let second_wire_magnitude = (second_segment.start[0] - second_segment.end[0]).abs()
                + (second_segment.start[1] - second_segment.end[1]).abs();
            second_wire_timing += second_wire_magnitude;
        }
        let first_wire_magnitude = (first_segment.start[0] - first_segment.end[0]).abs()
            + (first_segment.start[1] - first_segment.end[1]).abs();
        first_wire_timing += first_wire_magnitude;
    }
    println!(
        "Location of nearest crossing by Manhattan distance: {:?}",
        nearest_crossing_manhattan
    );
    println!(
        "Magnitude of nearest crossing by total wire length to that point: {:?}",
        nearest_crossing_wire_timing
    );

    // println!("{:?}", first_wire_segments);
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

fn do_lines_cross(
    first_wire_segment: &WireSegment,
    second_wire_segment: &WireSegment,
) -> Option<[i32; 2]> {
    // Check whether the two lines cross
    // Two conditions: they're either parallel, or perpendicular
    // If they're parallel, they meet if there respective co-ordinates overlap. Then choose the point
    // With the lowest value.
    // If they're perpendicular, they cross if A_X1 <= B_X1 <= B_X2 and B_Y1 <= A_Y1 <= B_Y2
    // When B is vertical, A is horizontal, and the points are (A_X1, A_Y1) and (A_X2, A_Y2) etc.
    if lines_parallel(first_wire_segment, second_wire_segment) {
        // Ignore this case for now
        None
    } else {
        let (a_x1, a_x2, a_y1, _a_y2, b_x1, _b_x2, b_y1, b_y2) =
            return_ordered_co_ords(first_wire_segment, second_wire_segment);

        // cross if a_x1 <= b_x1 <= a_x2 and b_y1 <= a_y1 <= b_y2. then the cross is at b_x1, a_y1
        if (a_x1 <= b_x1) && (b_x1 <= a_x2) && (b_y1 <= a_y1) && (a_y1 <= b_y2) {
            Some([b_x1, a_y1])
        } else {
            None
        }
    }
}

fn lines_parallel(first_wire_segment: &WireSegment, second_wire_segment: &WireSegment) -> bool {
    if (first_wire_segment.direction == "U" || first_wire_segment.direction == "D")
        && (second_wire_segment.direction == "U" || second_wire_segment.direction == "D")
    {
        true
    } else if (first_wire_segment.direction == "L" || first_wire_segment.direction == "R")
        && (second_wire_segment.direction == "L" || second_wire_segment.direction == "R")
    {
        true
    } else {
        false
    }
}

#[derive(Debug)]
struct WireSegment {
    start: [i32; 2],
    end: [i32; 2],
    direction: String,
}

fn return_ordered_co_ords(
    first_wire_segment: &WireSegment,
    second_wire_segment: &WireSegment,
) -> (i32, i32, i32, i32, i32, i32, i32, i32) {
    let (mut a_x1, mut a_x2, mut a_y1, mut a_y2) = (-1, -1, -1, -1);
    let (mut b_x1, mut b_x2, mut b_y1, mut b_y2) = (-1, -1, -1, -1);

    match first_wire_segment.direction.as_str() {
        "R" => {
            a_x1 = first_wire_segment.start[0];
            a_x2 = first_wire_segment.end[0];
            a_y1 = first_wire_segment.start[1];
            a_y2 = first_wire_segment.end[1];
            assert_eq!(a_y1, a_y2);
            match second_wire_segment.direction.as_str() {
                "U" => {
                    b_x1 = second_wire_segment.start[0];
                    b_x2 = second_wire_segment.end[0];
                    b_y1 = second_wire_segment.start[1];
                    b_y2 = second_wire_segment.end[1];
                    assert_eq!(b_x1, b_x2);
                }
                "D" => {
                    b_x2 = second_wire_segment.start[0];
                    b_x1 = second_wire_segment.end[0];
                    b_y2 = second_wire_segment.start[1];
                    b_y1 = second_wire_segment.end[1];
                    assert_eq!(b_x1, b_x2);
                }
                _ => (),
            }
        }
        "L" => {
            a_x2 = first_wire_segment.start[0];
            a_x1 = first_wire_segment.end[0];
            a_y2 = first_wire_segment.start[1];
            a_y1 = first_wire_segment.end[1];
            assert_eq!(a_y1, a_y2);
            match second_wire_segment.direction.as_str() {
                "U" => {
                    b_x1 = second_wire_segment.start[0];
                    b_x2 = second_wire_segment.end[0];
                    b_y1 = second_wire_segment.start[1];
                    b_y2 = second_wire_segment.end[1];
                    assert_eq!(b_x1, b_x2);
                }
                "D" => {
                    b_x2 = second_wire_segment.start[0];
                    b_x1 = second_wire_segment.end[0];
                    b_y2 = second_wire_segment.start[1];
                    b_y1 = second_wire_segment.end[1];
                    assert_eq!(b_x1, b_x2);
                }
                _ => (),
            }
        }
        "U" => {
            b_x1 = first_wire_segment.start[0];
            b_x2 = first_wire_segment.end[0];
            b_y1 = first_wire_segment.start[1];
            b_y2 = first_wire_segment.end[1];
            assert_eq!(b_x1, b_x2);
            match second_wire_segment.direction.as_str() {
                "L" => {
                    a_x2 = second_wire_segment.start[0];
                    a_x1 = second_wire_segment.end[0];
                    a_y2 = second_wire_segment.start[1];
                    a_y1 = second_wire_segment.end[1];
                    assert_eq!(a_y1, a_y2);
                }
                "R" => {
                    a_x1 = second_wire_segment.start[0];
                    a_x2 = second_wire_segment.end[0];
                    a_y1 = second_wire_segment.start[1];
                    a_y2 = second_wire_segment.end[1];
                    assert_eq!(a_y1, a_y2);
                }
                _ => (),
            }
        }
        "D" => {
            b_x2 = first_wire_segment.start[0];
            b_x1 = first_wire_segment.end[0];
            b_y2 = first_wire_segment.start[1];
            b_y1 = first_wire_segment.end[1];
            assert_eq!(b_x1, b_x2);
            match second_wire_segment.direction.as_str() {
                "L" => {
                    a_x2 = second_wire_segment.start[0];
                    a_x1 = second_wire_segment.end[0];
                    a_y2 = second_wire_segment.start[1];
                    a_y1 = second_wire_segment.end[1];
                    assert_eq!(a_y1, a_y2);
                }
                "R" => {
                    a_x1 = second_wire_segment.start[0];
                    a_x2 = second_wire_segment.end[0];
                    a_y1 = second_wire_segment.start[1];
                    a_y2 = second_wire_segment.end[1];
                    assert_eq!(a_y1, a_y2);
                }
                _ => (),
            }
        }
        _ => (),
    }

    assert_ne!(a_x1, -1);
    assert_ne!(a_x2, -1);
    assert_ne!(a_y1, -1);
    assert_ne!(a_y2, -1);
    assert_ne!(b_x1, -1);
    assert_ne!(b_x2, -1);
    assert_ne!(b_y1, -1);
    assert_ne!(b_y2, -1);

    (a_x1, a_x2, a_y1, a_y2, b_x1, b_x2, b_y1, b_y2)
}
