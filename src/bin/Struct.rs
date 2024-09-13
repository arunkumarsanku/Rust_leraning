// struct  User {
//     first_name: String,
//     last_name: String,
//     age: u32,
//     email: String,
//     active: bool,
// }

// fn main () {
//     let user1 = User {
//         first_name: String::from("John"),
//         last_name: String::from("Doe"),
//         age: 30,
//         email: String::from("example@gmail.com"), 
//         active: true,
// };
//     println!("First Name: {}", user1.first_name);
//     println!("Last Name: {}", user1.last_name);
//     println!("Age: {}", user1.age);
//     println!("Email: {}", user1.email);
//     println!("Active: {}", user1.active);
// }



struct User {
    first_name: String,
    last_name: String,
    age: u32,
    email: String,
    active: bool,
}

impl User {
    // Method to print user information
    fn print_info(&self) {
        println!(
            "User: {} {}, Age: {}, Email: {}, Active: {}",
            self.first_name, self.last_name, self.age, self.email, self.active
        );
    }

    // Method to update the email
    fn update_email(&mut self, new_email: String) {
        self.email = new_email;
    }

    // Associated function (not tied to a specific instance)
    fn create_user(first_name: String, last_name: String, age: u32, email: String) -> User {
        User {
            first_name,
            last_name,
            age,
            email,
            active: true,
        }
    }
}

fn main() {
    // Creating a user using the create_user function
    let mut user1 = User::create_user(
        String::from("John"),
        String::from("Doe"),
        30,
        String::from("john.doe@example.com"),
    );

    // Calling methods on the user
    user1.print_info(); // Print user info

    // Update the user's email
    user1.update_email(String::from("john.new1111@example.com"));
    user1.print_info(); // Print updated user info
}
