/*
    - Each value in Rust has an *owner*.
    - There can only be one owner at a time.
    - When the owner goes out of scope, the value will be dropped.

    **String Type**
    The string type is more complex than the other, fixed size, data types such as uint32 etc.

    - String literals: hardcoded string values eg. let str = "I'm super static 🥱"
    - Strings: Data type suitable for dynamic strings, eg. user input.
        Since the size of these are not known at compile time, they are allocated on the heap.
        Declared like this: let str = String::from("U'll find me on the heap ✨")


*/

fn main() {
    // string literal `s`, valid from declaration => end of scope (the code block)
    {
        // s not valid (not declared)
        let s = "hello"; // valid from here all the way to...
    } // here. End of scope => s is invalid

    // `String` datatype, dynamic queen 💅
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{s}"); // this will print `hello, world!`
}
