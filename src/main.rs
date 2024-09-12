
// Define the main function
fn main() {
    // Call the is_even function with the argument 21 and store the result in the variable ans
    let ans = is_even(21);
    // Print the value of ans
    println!("{}", ans);
}

// Define the is_even function that takes an i32 argument num and returns a bool
fn is_even(num: i32) -> bool {
    // Check if num is divisible by 2 with no remainder
    if num % 2 == 0 {
        // If it is, return true
        return true;
    } else {
        // If it's not, return false
        return false;
    }
}
 
