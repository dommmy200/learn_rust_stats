use std::option::Option::{self, Some, None};  // Importing the Option enum to use Some and None directly

fn main() {
    let numbers = vec![1.0, 3.0, 2.0, 4.0, 5.0, 6.0, 7.0, 9.0, 8.0, 10.0]; // A vector of numbers to calculate the median
    match median(numbers) {
        Some(med) => println!("The median is: {}", med),
        None => println!("No numbers provided."),
    }
    // This is a simple Rust program that calculates the median of a list of numbers.
    // It uses the Option enum to handle cases where the list might be empty.
    // The median function sorts the numbers and returns the middle value or the average of the two middle values.
    // The main function demonstrates how to call the median function and handle its result.
}
// This function calculates the median of a list of numbers.
// It returns an Option<f64> to handle the case where the list is empty.
// The median is the middle value in a sorted list of numbers.
// If the list has an even number of elements, it returns the average of the two middle values.
// If the list is empty, it returns None.
// The function sorts the numbers and then calculates the median based on the length of the list.
// The function uses the `partial_cmp` method to compare floating-point numbers, which can handle NaN values gracefully.
// The `Option` type is used to represent a value that can be either present (Some) or absent (None).
// This function calculates the median of a list of numbers.
// It returns an Option<f64> to handle the case where the list is empty.
// The median is the middle value in a sorted list of numbers.
// If the list has an even number of elements, it returns the average of the two middle values.
// If the list is empty, it returns None.
fn median(mut numbers: Vec<f64>) -> Option<f64> {
    let len = numbers.len();
    if len == 0 {
        return None;
    }
    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = len / 2;
    if len % 2 == 0 {
        Some((numbers[mid - 1] + numbers[mid]) / 2.0)
    } else {
        Some(numbers[mid])
    }
}