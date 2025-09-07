/*
    Slice Type 🍕

    > Slicers let you reference a contiguous sequence of elements in a connection. A slice is a kind of reference => does not have ownership

    Task A: **Here’s a small programming problem: write a function that takes a string of words separated by spaces and returns the first word it finds in that string.
    If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.**
*/

// Solving Task A without using slicers
fn first_word(s: &String) -> usize {
    // to get each char (each is a byte), get string as bytes
    let bytes = s.as_bytes();

    // iter => returns each element in a collection
    // enumerate => wraps result from iter into tuple (i, item)
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
        // Return length of s if no spaces
        s.len()
    }
}

fn main() {
    println!("Hello, world!");
}
