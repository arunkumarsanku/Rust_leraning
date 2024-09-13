// Define an enum Shape with three variants
enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}

fn main() {
    // Create a rectangle instance of Shape
    let rect = Shape::Rectangle(3.0, 4.0);
    let area = calculate_area(rect);
    println!("Area of the rectangle: {}", area);

    // Create a circle instance of Shape
    let circle = Shape::Circle(5.0);
    let area = calculate_area(circle);
    println!("Area of the circle: {}", area);

    // Create a square instance of Shape
    let square = Shape::Square(6.0);
    let area = calculate_area(square);
    println!("Area of the square: {}", area);
}

// Define a function to calculate the area of different shapes
fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => 3.14 * radius * radius,        // Area of a circle
        Shape::Square(side) => side * side,                     // Area of a square
        Shape::Rectangle(length, width) => length * width,      // Area of a rectangle
    }
}
