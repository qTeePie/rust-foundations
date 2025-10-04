/*
    *Methods* are similar to functions: : we declare them with the fn keyword and a name.
    They can have parameters and a return value, and they contain some code that’s run when the method is called from somewhere else.

    Methods are defined within the context or a struct / enum or trait object.
    First parameter is always *self*, representing the instance of the struct method is called on.

    Functions defined within an implementation block are refered to as *associated functions*, as they are associated to some type (here Rectangle).
    Not all associated functions has to be methods, a function that doesn't not have *self* as its first parameter while still defined in an implementation block,
    is an associated functions, whilst not a method.

    Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct.

    Methods are great for implementing several design patterns:
    1️⃣ Builder Pattern
    2️⃣ Factory Pattern
    3️⃣ Decorator Pattern

    See markdown file *note_DP* for more.

    Main purpose of methods is a well organized codebase.

    NOTE: **You can have several impl blocks!**
    eg.:

    ``` rust
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
    ```
*/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// implementation block containing Rectangle methods
impl Rectangle {
    // Associated function, but not a method (&self absent in params => not a method)
    // Lets us create a square with only one param instead of Rectangle(n, n)
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    // short for 'self: &Self', here immutable / read-only
    // for immutable => &mut self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height // read-only stuff
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    // Can one rectangle contain / hold another?
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Create square
    let square = Rectangle::square(3); // :: syntax used for associated funcs and namespaces
    println!("The area of the square is {} square pixels.", square.area());
}
