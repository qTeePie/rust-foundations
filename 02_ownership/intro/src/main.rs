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

    > A String is made up of three parts: a pointer to the memory that holds the contents of the string, a length, and a capacity.
    > This group of data is stored on the stack. The actual string content (where the pointer leads) is stored on the heap.

    When we assign s1 to s2, the **String data** is copied, meaning we copy the pointer, the length, and the capacity that are on the stack.
    **We do not copy the data on the heap that the pointer refers to.**

    When the owner of a piece of memory, goes out of scope, rust calls a function named *drop*
*/

fn double_free_error() {
    // **Double free error** is when two variables try to free the same memory.
    fn main() {
        let s1 = String::from("hello");
        let s2 = s1;

        // println!("{s1}, world!"); // outputs: borrow of moved value: `s1`
        // s1 is no longer valid 😭 its been **MOVED** to prevent double free errors.
        // s2 is now the owner of the allocated memory.
    }
}

fn create_some_string() {
    // `String` datatype, dynamic queen 💅
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
} // Here s (outer scope) is no loger valid, **its memory is returned** (rust's drop func)

fn main() {
    // string literal `s`, valid from declaration => end of scope (the code block)
    {
        // s not valid (not declared)
        let s = "hello"; // valid from here all the way to...
    } // here. End of scope => s is invalid
}
