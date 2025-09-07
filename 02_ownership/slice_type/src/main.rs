/*
    Slice Type 🍕

    > Slicers let you reference a contiguous sequence of elements in a connection. A slice is a kind of reference => does not have ownership

    ```
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    ```
    [starting_index..ending_index], where starting_index is the first position in the slice and ending_index is one more than the last position in the slice.

    Task A: **Here’s a small programming problem: write a function that takes a string of words separated by spaces and returns the first word it finds in that string.
    If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.**
*/

// Solving A without using slicers
fn first_word_buggy(s: &String) -> usize {
    // to get each char (each is a byte), get string as bytes
    let bytes = s.as_bytes();

    // iter => returns each element in a collection
    // enumerate => wraps result from iter into tuple (i, item)
    for (i, &item) in bytes.iter().enumerate() {
        // if curent byte is space, return the index
        if item == b' ' {
            return i;
        }
    }
    // no spaces found => return length of s
    s.len()
}

// Solving A with slicers
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // return slice pointing including start up to index of item (first space)
            return &s[0..i];
        }
    }

    &s[..] // <- also a slice, but the whole string s
}

fn first_word_accepting_literals(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // return slice pointing including start up to index of item (first space)
            return &s[0..i];
        }
    }

    &s[..] // <- also a slice, but the whole string s
}

fn main() {
    // buggy demo
    let mut s1 = String::from("First String");
    let _word = first_word_buggy(&s1); // word will get the value 5

    s1.clear(); // empties s1
                // _word is now invalid reference (dangling index)

    // slice demo, ✨ proper ✨
    let s2 = String::from("Second String");
    let word = first_word(&s2); // word is an immutable borrow (&str) into s2

    //s2.clear(); // error!
    // ^^^^^^^^^ mutable borrow occurs here, clear() function wants to mutably borrow s2 to clear it, but it's being used in statement below!

    println!("the first word is: {word}");

    // demo of func that also accepts literals 🩺
    let s3 = "Literal String";

    let word = first_word_accepting_literals(&s3[..]); // lets just shadow word from slice demo
    println!("the second word is: {word}");
}
