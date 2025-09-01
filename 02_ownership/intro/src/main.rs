/*-
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

        Stack:
    ┌───────────────┐
    │     s1        │
    │  ptr →────────┼────┐
    │  len:    5    │    │
    │  cap:    5    │    │
    └───────────────┘    │
                        ↓
    Heap:
    ┌────┬────┬────┬────┬────┐
    │ h  │ e  │ l  │ l  │ o  │
    └────┴────┴────┴────┴────┘

    When we assign s1 to s2, the **String data** is copied, meaning we copy the pointer, the length, and the capacity that are on the stack.
    **We do not copy the data on the heap that the pointer refers to.**

    ! If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone
    Clone is for heap-memory stuff. You don't have to call clone on variables of fixed-size type.

    When the owner of a piece of memory, goes out of scope, rust calls a function named *drop*

    **The ownership of a piece of memory follows the same pattern:
    assigning a value to another variable moves it.**

    When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless ownership of the data has been moved to another variable.

*/

fn print_string_struct(label: &str, s: &String) {
    println!("===============================");
    println!("📦 String Label: {}", label);
    println!("🧠 Stack Address of struct (&s): {:p}", s);
    println!("📍 Heap Pointer (s.as_ptr()):   {:p}", s.as_ptr());
    println!("📏 Length (s.len()):            {}", s.len());
    println!("📦 Capacity (s.capacity()):     {}", s.capacity());
    println!("💬 Contents:                    {}", s);
    println!("===============================\n");
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn modify_string(s: &mut String) {
    s.push_str(", world!");
}

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

fn clone_string() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    // 🧠 Get raw pointers to the heap data of each String
    // They are pointing to different pieces of memory!
    let ptr1 = s1.as_ptr();
    let ptr2 = s2.as_ptr();

    println!("s1 = {s1}, s2 = {s2}");
    println!("s1 pointer = {:p}", ptr1);
    println!("s2 pointer = {:p}", ptr2);

    // Just to be ✨ extra ✨
    print_string_struct("s1", &s1);
    print_string_struct("s2", &s2);
}

fn assign_new_content() {
    let mut s = String::from("hello");
    s = String::from("ahoy"); // byeee "hello", u're free now 🏴‍☠️

    println!("{s}, world!");
}

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

    let mut s = String::from("hello");

    modify_string(&mut s); // 🧠 we’re *borrowing* it mutably
    println!("AFTER: {s}"); // ✅ still valid & changed!

    takes_ownership(s); // 🔥 s is MOVED here
                        // println!("{s}"); // ❌ COMPILE ERROR: s is now GONE

    let s = gives_ownership(); // yey s is valid again 🚀 we recieved it from gives_ownership() omg... so CUTE

    clone_string();
}
