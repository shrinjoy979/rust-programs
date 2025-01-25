// Enums
/*
enum Direction {
    North,
    East,
    South,
    West,
}

fn main() {
    move_around("north".to_string());
}

fn move_around(direction: String) {
    if direction == "north" {
        println!("Moving North");
    }
}
*/

/*
// Define an enum called Shape
enum Shape {
    Circle(f64),  // Variant with associated data (radius)
    Square(f64),  // Variant with associated data (side length)
    Rectangle(f64, f64),  // Variant with associated data (width, height)
}

// Function to calculate area based on the shape
fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(side_length) => side_length * side_length,
        Shape::Rectangle(width, height) => width * height,
    }
}

fn main() {
    // Create instances of different shapes
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);

    // Calculate and print the areas
    println!("Area of circle: {}", calculate_area(circle));
    println!("Area of square: {}", calculate_area(square));
    println!("Area of rectangle: {}", calculate_area(rectangle));
}
 */

 /*
 use std::fs;

 fn main() {
     let greeting_file_result = fs::read_to_string("hello.txt");
 
     match greeting_file_result {
         Ok(file_content) => {
             println!("File read successfully: {:?}", file_content);
         },
         Err(error) => {
             println!("Failed to read file: {:?}", error);
         }
     }
 }
*/

/*
use core::fmt;
use std::{fmt::{Debug, Formatter}, fs};

pub struct FileReadError {

}

fn main() {
    let contents = read_file("hello.txt".to_string());
    match contents {
        Ok(file_content) => {
            println!("File content: {}", file_content);
        },
        Err(error) => {
            println!("Error reading file: {:?}", error);
        }
    }
}

fn read_file(file_path: String) -> Result<String, FileReadError> {
    let greeting_file_result = fs::read_to_string("hello.txt");
    match greeting_file_result {
        Ok(file_content) => {
            Ok(file_content)
        },
        Err(error) => {
            let err = FileReadError {};
            Err(err)
        }
    }
}
 */

