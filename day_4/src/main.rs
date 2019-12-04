use std::error::Error;
use std::process;

fn main() {
    let input_password_range = [147981, 691423];
    if let Err(e) = run(input_password_range) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(password_range: [i32; 2]) -> Result<(), Box<dyn Error>> {
    /*
    - Password is a six-digit number
    - The value is within the given range
    - Two adjacent digits are the same
    - Going from left to right, the digits never decrease

    How many passwords meet this criteria?

    Part 2:
    - the two adjacent matching digits are not part of a larger group of matching digits.
    (i.e the set of double digits occurs exactly twice)
    });
    */
    let mut num_passwords_part_one = 0;
    let mut num_passwords_part_two = 0;
    for candidate_password in password_range[0]..password_range[1] {
        // Get the digits in order
        let digits = get_digits(candidate_password);
        if is_valid_part_one(digits.clone()) {
            num_passwords_part_one += 1;
        }
        if is_valid_part_two(digits.clone()) {
            num_passwords_part_two += 1;
        }
    }
    println!("Num of valid passwords: {}", num_passwords_part_one);
    println!(
        "Num of valid passwords with extra criterion: {}",
        num_passwords_part_two
    );
    Ok(())
}

fn get_digits(n: i32) -> Vec<i32> {
    fn x_inner(n: i32, xs: &mut Vec<i32>) {
        if n >= 10 {
            x_inner(n / 10, xs);
        }
        xs.push(n % 10);
    }
    let mut xs = Vec::new();
    x_inner(n, &mut xs);
    xs
}

fn is_valid_part_one(digits: Vec<i32>) -> bool {
    // Check that at least two digits are same
    // And the digits never decrease

    let mut prev_digit = 0;
    let mut pair_equal = false;
    let mut decreasing = false;
    for digit in digits {
        if digit < prev_digit {
            decreasing = true;
            break;
        }
        if digit == prev_digit {
            pair_equal = true;
        }
        prev_digit = digit;
    }
    !decreasing && pair_equal
}

fn is_valid_part_two(digits: Vec<i32>) -> bool {
    // Check that there is a set of exactly two digits the same
    // And the digits never decrease

    let mut prev_digit = 0;
    let mut exactly_pair_equal = false;
    let mut decreasing = false;
    let mut sets = vec![];
    let mut set = vec![];
    for &digit in &digits {
        if digit < prev_digit {
            decreasing = true;
            break;
        }
        if digit == prev_digit {
            set.push(digit);
        } else {
            sets.push(set);
            set = vec![digit];
        }
        prev_digit = digit;
    }
    sets.push(set);

    for set in &sets {
        if set.len() == 2 {
            exactly_pair_equal = true;
            break;
        }
    }

    !decreasing && exactly_pair_equal
}

// if !decreasing {
//     println!("hit");
//     println!("{:?}", digits);
//     println!("{:?}", sets);

//     for set in &sets {
//         if set.len() > 3 {
//             println!("{:?}", digits);
//             println!("{:?}", sets);
//             break;
//         }
//     }
