Sure! Below are two examples to illustrate the difference between **normal struct code** (just defining and using a struct) and **implementing structs code** (using `impl` to add methods and behavior to the struct).

### 1. **Normal Struct Code** (Without `impl`)

This version just defines a struct and creates an instance of it, accessing its fields directly. There are no methods or behavior associated with the struct.

```rust
// Define the User struct
struct User {
    first_name: String,
    last_name: String,
    age: u32,
    email: String,
    active: bool,
}

fn main() {
    // Create an instance of the User struct
    let user1 = User {
        first_name: String::from("John"),
        last_name: String::from("Doe"),
        age: 30,
        email: String::from("john.doe@example.com"),
        active: true,
    };

    // Access the fields directly
    println!("User: {} {}", user1.first_name, user1.last_name);
    println!("Age: {}", user1.age);
    println!("Email: {}", user1.email);
    println!("Active: {}", user1.active);
}
```

### Key Points in Normal Struct Code:
- This struct only holds data.
- There are no methods, so to interact with or modify the data, you manually access and modify the fields directly.
- No additional logic is encapsulated in the struct itself.

---

### 2. **Implementing Structs Code** (With `impl`)

This version defines the same struct, but also uses the `impl` block to add methods that provide behavior, such as creating a user, printing user details, and updating the email.

```rust
// Define the User struct
struct User {
    first_name: String,
    last_name: String,
    age: u32,
    email: String,
    active: bool,
}

// Implementation block for the User struct
impl User {
    // Associated function to create a new user (constructor-like function)
    fn create_user(first_name: String, last_name: String, age: u32, email: String) -> User {
        User {
            first_name,
            last_name,
            age,
            email,
            active: true, // Set default value for active
        }
    }

    // Method to return the user's full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Method to update the user's email
    fn update_email(&mut self, new_email: String) {
        self.email = new_email;
    }

    // Method to print user information
    fn print_info(&self) {
        println!(
            "User: {} {}, Age: {}, Email: {}, Active: {}",
            self.full_name(),
            self.age,
            self.email,
            self.active
        );
    }
}

fn main() {
    // Create a new user using the associated function
    let mut user1 = User::create_user(
        String::from("John"),
        String::from("Doe"),
        30,
        String::from("john.doe@example.com"),
    );

    // Print the user's full information
    user1.print_info();

    // Update the user's email and print the info again
    user1.update_email(String::from("john.new@example.com"));
    user1.print_info();
}
```

### Key Points in Implementing Structs Code:
- The `impl` block adds **methods** and **associated functions** to the struct.
- **Associated Function (`create_user`)**: This function is used to create a new `User` instance. It simplifies the creation process and sets default values.
- **Methods (`full_name`, `update_email`, `print_info`)**: These methods allow the struct to encapsulate behavior, such as generating the full name, updating the email, and printing user details.
- The struct now contains both **data** and **behavior**, making the code more modular and reusable.

### Comparison:

- **Normal Struct Code**:
  - You create and use the struct by manually assigning and accessing its fields.
  - No behavior (methods) is associated with the struct.
  
- **Implementing Structs Code**:
  - You use `impl` to define methods and behavior for the struct, making it more powerful.
  - Methods encapsulate logic, making it easier to manipulate and work with the data.
  - The struct becomes more modular, reusable, and easier to maintain.

Both versions are valid depending on the needs of your application. If you only need to store and retrieve data, the **normal struct** is sufficient. If you need to encapsulate behavior or logic with the data, use **implementing structs** with the `impl` block.