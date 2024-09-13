/// write a function that takes a string and returns the length of the string

 
fn main() { 
    let string = String::from("Hello, World!"); // Create a string
    let length = get_string_length(&string);    // Get the length of the string
    println!("The length of the string is: {}", length); // Print the length of the string
}

fn get_string_length(string: &str) -> usize { // Function to get the length of the string
    string.chars().count()  // Return the length of the string
}
