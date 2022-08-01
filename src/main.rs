fn main() {
    run(0, 10);
}

/**
 * This function prints in the terminal whether an integer is an even
 *  or odd number given a range of number.
 * 
 * It accepts two parameters:
 * start: start value of range
 * limit: end value of range
 * 
 * Example:
 * 
 * run(0, 10);
 * 
 * Should print:
 * 0 is an even number
 * 1 is an odd number
 * 2 is an even number
 * 3 is an odd number
 *          .
 *          .
 *          .
 * 9 is an odd number
 */
pub fn run(start: u32, limit: u32) {
    let mut n = start;

    while n < limit {
        if n % 2 == 0 {
            println!("{} is even", n);
        }
        else {
            println!("{} is odd", n);
        }

        // Increment counter
        n += 1;
    }  
}