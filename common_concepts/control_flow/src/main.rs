/*
    Control Flow 🔁🧠

    Since *if* is an expression, we can use it as the left side of a *let* variable initiation.

    Like this:
    let number = if condition { 5 } else { 6 }; ✅ `number` binds to the return value
    let number = if condition { 5 } else { "six" }; ❌ mismatch in datatype! Rustie says, no go!

    Loops returns value with break statement, eg. break result;
    Remember how they said blocks return last expression as long as there is no ;?

    Loops return values via break, not by just ending with an expression.
    Break doesn't need to be any place in the loop, end, middle, start,
    wherever in the block it is, attach a value and its returned.

    “I’m out 💨 — oh, and here’s the value: 🧾 result;”
*/

fn looper_nested() {
    let mut count = 0;

    // sets loop label
    let remaining = 'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            // taking a coupe rounds until count is 2, then breaks outer loop
            if count == 2 {
                // break 'counting_up; // returns () type unit
                break 'counting_up remaining; // <-- this returns value
            }
            remaining -= 1;
        }

        count += 1;
    };

    println!("End count = {count}\n End remaining {remaining}");
}

fn looper() -> i32 {
    // mutable variabøe
    let mut counter = 0;

    loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    } // no ; here cause we want the functiont to return this value (attached to break)
}

fn let_if(condition: bool) -> i32 {
    if condition {
        5
    } else {
        6
    } // expression that returns either 5 or 6
}

fn main() {
    let result = let_if(true);
    println!("The value of number is: {result}");

    // Shadowing
    let result = looper();
    println!("The result from loop is {result}");

    looper_nested();
}
