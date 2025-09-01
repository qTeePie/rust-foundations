/*
    ✨References

    Just some stuff on passing pointers to functions.

    You can pass these as READ or as mutable (function access and change the contents of the memory address pointed to)
    &String points to the String object stored on the stack, which includes the pointer to the actual string contents (on heap).

    The String object in stack, pointing to actual content stored on heap:

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

    Passing &s1 handles over the pointer to the String object on the stack.

        &s1
     ↓
    ┌───────────────┐
    │  ptr → heap   │
    │  len = 5      │
    │  cap = 5      │
    └───────────────┘


*/

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

// Instead of passing the string, we pass a reference to it (&String)
fn calculate_length(s: &String) -> usize {
    s.len()
}
