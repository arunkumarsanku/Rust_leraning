
/// Calculates the Fibonacci number for the given input.
fn main() {
    println!("{}", fib(10));
}

fn fib(num: u32) -> u32 {
    let mut first = 0; // Initialize the first Fibonacci number
    let mut second = 1; // Initialize the second Fibonacci number

    if num == 0 {
        return first; // Return the first Fibonacci number if input is 0
    }
    if num == 1 {
        return second; // Return the second Fibonacci number if input is 1
    }

    for _ in 2..num {
        let temp = second; // Store the current second Fibonacci number
        second = first + second; // Calculate the next Fibonacci number
        first = temp; // Update the first Fibonacci number
    }
    return second; // Return the calculated Fibonacci number
}
