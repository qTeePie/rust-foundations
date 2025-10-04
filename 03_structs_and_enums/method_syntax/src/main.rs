/*
    *Methods* are similar to functions: : we declare them with the fn keyword and a name.
    They can have parameters and a return value, and they contain some code that’s run when the method is called from somewhere else.

    Methods are defined within the context or a struct / enum or trait object. First parameter is always *self*, representing the instance of the struct method is called on.

*/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
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
}
