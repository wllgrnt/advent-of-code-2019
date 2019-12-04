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
    */

    for candidate_password in password_range[0]..password_range[1] {
        println!("{:?}", candidate_password);
        // Get the digits in order
        
        let digits = get_digits(candidate_password);
        println!{"{:?}", digits}
    }
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