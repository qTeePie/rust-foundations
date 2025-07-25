/*
    Control Flow 🔁🧠

    Since *if* is an expression, we can use it as the left side of a *let* variable initiation.

    Like this:
    let number = if condition { 5 } else { 6 }; ✅ `number` binds to the return value
    let number = if condition { 5 } else { "six" }; ❌ mismatch in datatype! Rustie says, no go!


*/
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 }; // expression that returns either 5 or 6

    println!("The value of number is: {number}");
}
