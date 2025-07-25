/*
    Control Flow 🔁🧠

    Since *if* is an expression, we can use it as the left side of a *let* variable initiation.

    Like this:
    let number = if condition { 5 } else { 6 }; ✅ `number` binds to the return value
    let number = if condition { 5 } else { "six" }; ❌ mismatch in datatype! Rustie says, no go!


*/

fn looper() -> i32 {
    // mutable variabøe
    let mut counter = 0;

    loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    } // no ; here cause we want the functiont to return this value (beak return)
}

fn let_if(condition: bool) -> i32 {
    if condition {
        5
    } else {
        6
    } // expression that returns either 5 or 6
}

fn main() {
    let result = letIf(true);
    println!("The value of number is: {result}");

    // Shadowing
    let result = looper();
    println!("The result is {result}");
}
