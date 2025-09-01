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

    Passing a reference to the String lets other functions interact with it without taking ownership.
    This way we don't need to pass it back and forwards like hooligans 🤪 to prevent it being dropped (from being borrowed to a function going out of scope).

    Rustaceans refer to the action of creating references as ✨ "borrowing" ✨.

    > Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value. This code that attempts to create two mutable references to s will fail:
        - This is to prevent data races (“I’m modifying memory at the same time as someone else either modifying !OR! reading from the same memory 😰 and we have NO synchronization 💥”)
    🔒 If you ever need to have multiple references — especially multiple mutable ones — Rust won’t let you do it safely unless you use a synchronization primitive like Mutex, RwLock, or Atomic* types.

    **GPTBabes Advice**
    Imagine s is a hot new celeb 🧑‍🎤, and mutable refs are journalists.
    Rust is the strict PR agent like:

    "Only one person gets exclusive interview rights at a time.
    You want another one? Wait until the first one’s out the door. 😤🧃"
    ***

    > Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used.

    *** ✅ Goodie! *** => r1 and r2' last use prior to initializing r3 🩺
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s; // no problem
    println!("{r3}");
    ******

    *** ❌ Not Goodie *** => immutable references still active when r2 is initialized 😰
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{r1}, {r2}, and {r3}");
    ******

*/

/*
// ❌ Dangling reference alert: Trying to return a ref to memory that just ✨dissolved✨
// s goes out of scope = stack gone + heap gone = &s points to a ghost 👻
fn dangle() -> &String {
    let s = String::from("hello"); // // Reminder: When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless ownership of the data has been moved to another variable.

    &s
} // s (String object on stack) + its content on heap are dropped here!
  // &s would point to a void 😰 Rust protects

// ✅When initializing a String object in a function, return the full object like below to avoid Rust errors on dangling references !
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
*/

fn change(some_string: &mut String) {
    // This badBoy needs a mutable reference since he be changing stuff 😈
    some_string.push_str(", world");
}

// Instead of passing the string, we pass a reference to it (&String)
fn calculate_length(s: &String) -> usize {
    // Reference to String object on stack passed as param, length ✅ Only needs READ access
    s.len()
} // Here, s goes out of scope. But because s does not have ownership of what
  // it refers to, the String is not dropped.

fn main() {
    let mut s1 = String::from("First!");

    let len = calculate_length(&s1);

    // s1 is alive and well, never left our side 💅
    println!("The length of '{s1}' is {len}.");

    // Passing mutable reference (even tho s1 is mutable we need a mutable reference to allow borrowers to make changes)
    change(&mut s1);

    // ❗ NB
    let mut s = String::from("Everyone wants me lol, get in line!");

    let r1 = &mut s; // ✅
                     // let r2 = &mut s; // ❌ Fails 😭 can only have one mutable reference to s
                     // let r2 = &s; // ❌ While r1 is stil in use u can't make immutable pointers either 😵
    println!("{r1}");

    let r2 = &mut s; // ✅ r1 is no longer in use
    println!("{r2}");

    {
        let r1 = &mut s; // ✅ YEPSI! Works we **shadow** r1, initializing a brand new variable, r1 being immutable / not doesn't matter
                         // r1 = &mut s; ❌ NOPE! this wouldn't work since r1 isn't immutable
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}
