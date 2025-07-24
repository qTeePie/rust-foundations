/*
    **Statements and Expressions 🧡**

    Rust is an expression-based language.

    - Statements: Instructions that *do stuff* but don’t return a value, e.g., `let x = 5;`
        A function declaration is also a statement.
        ❌ `let x = (let y = 6);` → invalid! You can’t assign a statement to a variable.
        In C though? `x = y = 6` ✅ works, because `y = 6` returns the value `6`.

    - Expressions: Evaluate to a value ❔
        => simpler put: **they give you something back!** 😏

        Like:
            5 + 3         // returns 8
            { 2 + 2 }     // block returns 4
            "hello"       // string is a value

        Expressions = the givers 🧚‍♀️
        Statements = the doers 🛠️ (but they don’t give you anything back)

    **NB THOO**: Expressions can be *inside* statements. Like here:
        👉 `( let y = (6) )` → outer: statement, inner `(6)`: expression

    Function calls are expressions too — why thooo?
    👉 Because they return a value.

    Even functions with no `return` value return `()` — the unit type 🥁

    The rust book says that blocks that use semicolon gets turned into a statement (they return the unit type)
    ... is this also true when calling a function that returns the unit type????

    idk i guess calling a function is *still* an expression even if it returns ()

   ChatGPT 💙 says:
    //  ## 👑 YOU'RE ASKING:

    // > 1. The book says: **expression + `;` = statement** ✅
    // > 2. So `shout();` should be a **statement** ✅
    // > 3. But a function returning `()` is still an **expression** ❓
    // > 4. And `{ something; }` becomes a statement... but also an expression? ❓❓
    // > 5. Then what even *is* a statement anymore?? 😭😭😭

    // ---

    // ### 💣 LET ME FIX THE DEFINITION FIRST:

    // Rust **does not** have a "Statement" type.
    // There are only two things:

    // * **Expressions**
    // * ...and expressions **used as statements** 😤

    // The thing you’re calling a “statement”? It’s just:

    // > ➡️ **An expression being used in a context where its value is thrown away.**

*/

// ✨ Functions below the notes section ✨

// Every `{}` block in Rust returns a value.
// If it ends without a semicolon → it returns the last expression’s result
fn returns_value_from_block() -> i32 {
    let result = {
        let a = 2;
        let b = 3;
        a + b // ✅ returns 5
    };
    result
}

// If the block ends with a semicolon, girl... you're left with nothing but `()` 😭
fn returns_unit_from_block() -> () {
    let result = {
        let a = 2;
        let b = 3;
        a + b; // 📦 semicolon → returns unit `()`
    };
    println!("Result is: {:?}", result); // prints: ()

    // if you comment out the line above and let this line be the last expression, the function returns the return value of println => also () the unit type!
    // println!("Result is: {:?}", result)
}

// Just a regular function — calling this is also an expression!
// ( ... but its decleration is a statement ❕ )
fn another_function(x: i32) {
    println!("The value of x is: {x}"); // Local variables stay in scope
}

fn main() {
    another_function(5);

    returns_unit_from_block();
    returns_value_from_block();

    let y = {
        let x = 3;
        x + 1 // ✅ block returns 4
    };

    println!("The value of y is: {y}");
}
