/**
 * Constant vs Variable in Rust:
 *
 * 1. `mut` cannot be used with constants.
 *
 * 2. Constants are declared using the keyword `const` instead of `let`.
 *
 * 3. Constants must have their type annotated — Rust won't infer it.
 *    ❌ Invalid: const X = 5;
 *    ✅ Correct: const X: i32 = 5;
 *
 * 4. Constants can be declared in any scope (including global scope).
 *    But `let` variables can't live globally — only `const` and `static` can.
 *
 * 5. Constants must be known at compile time — they can’t depend on values computed at runtime.
 *
 * Naming convention: Like in C, use ALL_CAPS with underscores for const names.
 */

fn main() {
    let x = 5;

    // 🌘 Shadowing in Rust:
    //
    // Shadowing lets you redeclare a variable with the same name,
    // creating a new variable and hiding the previous one in the same scope.
    //
    // - "The first variable is shadowed by the second."
    // - Each new shadow has its own memory address.
    //
    // Because shadowing creates a new variable, it can have a different data type than the one it replaces.
    // Ex:
    //     let spaces = "   ";             // string
    //     let spaces = spaces.len();      // int

    let x = x + 1;

    {
        // Inner shadowing block:
        //
        // The variable below shadows the previous declaration of `x`,
        // taking over the name until it is shadowed again or until the scope ends.

        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");

        // End of scope
    }

    // Now `x` refers to the version from the outer scope (x + 1)
    println!("The value of x is: {x}")
}
